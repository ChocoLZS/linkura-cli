use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

use crate::config::Global;
use anyhow::{Context, Result};
use chrono::{DateTime, Duration, Local, TimeDelta, Utc};
use indicatif::{ProgressBar, ProgressStyle};
use linkura_packet::{
    als::client::{Client, ConnectionInfo},
};
use linkura_common::jwt::extract_jwt_payload;

pub struct AlsConnectionInfo {
    pub address: Option<String>,
    pub port: Option<u16>,
    pub room_id: Option<String>,
    pub token: Option<String>,
}

pub fn run(ctx: &Global, connection_info: AlsConnectionInfo, watch_mode: bool) -> Result<()> {
    let needs_fetch_connection_info = connection_info.address.is_none()
        || connection_info.port.is_none()
        || connection_info.room_id.is_none()
        || connection_info.token.is_none();
    tracing::info!(
        "Connection info: address: {:?}, port: {:?}, room_id: {:?}, token: {:?}, needs_fetch: {:?}",
        connection_info.address,
        connection_info.port,
        connection_info.room_id,
        connection_info.token,
        needs_fetch_connection_info
    );
    let connection_info = if needs_fetch_connection_info {
        fetch_connection_info(ctx, connection_info, watch_mode)?
    } else {
        ConnectionInfo {
            host: connection_info.address.unwrap(),
            port: connection_info.port.unwrap(),
            room_id: connection_info.room_id.unwrap(),
            token: connection_info.token.unwrap(),
        }
    };
    tracing::info!(
        "Connecting to ALS server at {}:{} with room_id: {}",
        connection_info.host,
        connection_info.port,
        connection_info.room_id
    );
    run_client(ctx, connection_info)
}

fn fetch_connection_info(
    ctx: &Global,
    connection_info: AlsConnectionInfo,
    watch_mode: bool,
) -> Result<ConnectionInfo> {
    let api_client = &ctx.api_client;
    let plan_list = api_client.get_plan_list()?;
    let now = Utc::now();
    let res: Option<&serde_json::Value> = plan_list.as_array().unwrap().first();
    if res.is_none() {
        return Err(anyhow::anyhow!("No plan found"));
    }
    let item = res.unwrap();
    let name = item.get("name").unwrap().as_str().unwrap();
    let description = item.get("description").unwrap().as_str().unwrap();
    let live_start_time = item.get("live_start_time").unwrap().as_str().unwrap();
    let live_start_time = DateTime::parse_from_rfc3339(live_start_time).unwrap();
    let live_id = item.get("live_id").unwrap().as_str().unwrap().to_string();
    tracing::info!(
        "Waiting for plan: \ntitle:\n{}\ndescription: \n{}\nstart_time: {}",
        name,
        description,
        live_start_time
            .with_timezone(&Local)
            .format("%Y-%m-%d %H:%M:%S %:z")
    );

    // watch mode if setting
    let start_time_offset: TimeDelta = Duration::minutes(10) - Duration::seconds(2);
    if now < live_start_time - start_time_offset {
        if !watch_mode {
            return Err(anyhow::anyhow!("The plan has not started yet"));
        }
        let pb = ProgressBar::new_spinner();
        pb.enable_steady_tick(std::time::Duration::from_millis(80));
        pb.set_style(
            ProgressStyle::with_template("{spinner:.blue} {msg}")
                .unwrap()
                .tick_strings(&[
                    "🕐 ", "🕑 ", "🕒 ", "🕓 ", "🕔 ", "🕕 ", "🕖 ", "🕗 ", "🕘 ", "🕙 ", "🕚 ",
                    "🕛 ",
                ]),
        );
        loop {
            let now = Utc::now();
            let delta = live_start_time
                - now.with_timezone(&live_start_time.timezone())
                - start_time_offset;
            if delta.num_seconds() <= 0 {
                pb.finish_with_message("Live is starting soon, fetching connection info...");
                break;
            }
            if delta.num_minutes() >= 0 && delta.num_seconds() >= 0 {
                pb.set_message(format!(
                    "[{}] Waiting for live to start at {} within {} minutes, {} seconds",
                    now.with_timezone(&Local).format("%Y-%m-%d %H:%M:%S %:z"),
                    (live_start_time.with_timezone(&Local) - start_time_offset)
                        .format("%Y-%m-%d %H:%M:%S %:z"),
                    delta.num_minutes(),
                    delta.num_seconds() - 60 * delta.num_minutes()
                ));
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
        }
    }
    let live_type = item.get("live_type").unwrap().as_u64().unwrap();
    let token;
    let max_retry = 10;
    let mut retry_count = 0;
    loop {
        let now = Utc::now();
        if now >= live_start_time - start_time_offset {
            if live_type == 2 {
                // 401 需要尝试重新登录
                match api_client.get_with_meets_connect_token(&live_id) {
                    Ok(t) => {
                        token = t;
                        break;
                    }
                    Err(e) => {
                        tracing::error!("Failed to retrieve token: {}", e);
                    }
                }
            }
            if live_type == 1 {
                match api_client.get_fes_live_connect_token(&live_id) {
                    Ok(t) => {
                        token = t;
                        break;
                    }
                    Err(e) => {
                        tracing::error!("Failed to retrieve token: {}", e);
                    }
                }
            }
        }

        tracing::info!("Cannot get token, retrying in 3 seconds...");
        retry_count += 1;
        if retry_count > max_retry {
            return Err(anyhow::anyhow!("Max retry limit reached"));
        }
        std::thread::sleep(std::time::Duration::from_secs(3));
    }
    let payload_json = extract_jwt_payload(token.as_str())?;
    Ok(ConnectionInfo {
        host: connection_info
            .address
            .unwrap_or_else(|| payload_json["pod"]["address"].as_str().unwrap().to_string()),
        port: connection_info
            .port
            .unwrap_or_else(|| payload_json["pod"]["port"].as_u64().unwrap_or(9201) as u16),
        room_id: connection_info
            .room_id
            .unwrap_or_else(|| payload_json["room_id"].as_str().unwrap().to_string()),
        token,
    })
}

fn run_client(_ctx: &Global, connection_info: ConnectionInfo) -> Result<()> {
    let running_signal = Arc::new(AtomicBool::new(true));
    let running_signal_clone = running_signal.clone();

    ctrlc::set_handler(move || {
        tracing::info!("Received Ctrl+C signal, initiating graceful shutdown...");
        running_signal_clone.store(false, Ordering::Relaxed);
    })
    .context("Error setting Ctrl+C handler")?;

    // 创建客户端实例
    let mut client = Client::new(running_signal, connection_info, None)
        .context("Failed to create MrsClient instance")?;

    tracing::info!("MRS Client starting...");
    tracing::info!("Press Ctrl+C to gracefully shutdown");

    // 运行客户端
    match client.run() {
        Ok(_) => {
            tracing::info!("Client shut down gracefully");
        }
        Err(e) => {
            tracing::error!("Client error: {}", e);
            client.disconnect();
        }
    }

    let now = chrono::Local::now();
    tracing::info!("Program ended at: {}", now.format("%Y-%m-%d %H:%M:%S"));

    Ok(())
}
