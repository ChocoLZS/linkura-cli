use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

use crate::config::Global;
use anyhow::{Context, Result};
use chrono::{DateTime, Duration, TimeDelta, Utc};
use linkura_client::{
    als::client::{self, Client, ConnectionInfo},
    api::extract_jwt_payload,
};

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
    let mut res: Option<&serde_json::Value> = None;
    for item in plan_list.as_array().unwrap().into_iter() {
        let end_time = item.get("end_time").unwrap().as_str().unwrap();
        let end_time = DateTime::parse_from_rfc3339(end_time).unwrap();
        if now <= end_time {
            res = Some(item)
        } else {
            break;
        }
    }
    if res.is_none() {
        return Err(anyhow::anyhow!("No plan found"));
    }
    let item = res.unwrap();
    let live_start_time = item.get("live_start_time").unwrap().as_str().unwrap();
    let live_start_time = DateTime::parse_from_rfc3339(live_start_time).unwrap();
    let live_id = item.get("live_id").unwrap().as_str().unwrap().to_string();
    // watch mode if setting
    const START_TIME_OFFSET: TimeDelta = Duration::minutes(10);
    if now < live_start_time {
        if !watch_mode {
            return Err(anyhow::anyhow!("The plan has not started yet"));
        }
        tracing::info!(
            "The plan has not started yet, waiting for {} minutes",
            (live_start_time - now.with_timezone(&live_start_time.timezone())).num_minutes()
        );
        std::thread::sleep(
            (live_start_time - now.with_timezone(&live_start_time.timezone()))
                .to_std()
                .unwrap(),
        );
    }
    let live_type = item.get("live_type").unwrap().as_u64().unwrap();
    let token;
    loop {
        let now = Utc::now();
        if now >= live_start_time - START_TIME_OFFSET {
            if live_type == 2 {
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
        }

        tracing::info!("Cannot get token, retrying in 5 seconds...");
        std::thread::sleep(std::time::Duration::from_secs(5));
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

fn run_client(ctx: &Global, connection_info: ConnectionInfo) -> Result<()> {
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
