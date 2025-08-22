use std::{fs::File, sync::{
    atomic::{AtomicBool, Ordering}, Arc
}};
use clap::{Args as ClapArgs};

use crate::{cli, config::Global};
use anyhow::{Context, Result};
use chrono::{DateTime, Duration, Local, TimeDelta, Utc};
use indicatif::{ProgressBar, ProgressStyle};
use linkura_api::ApiClient;
use linkura_common::jwt::extract_jwt_payload;
use linkura_packet::als::client::{Client, ConnectionInfo};

#[derive(Debug, ClapArgs)]
pub struct ArgsALS {
    #[clap(short('a'), long = "address", value_name = "ADDRESS")]
    pub addr: Option<String>,
    #[clap(short('p'), long = "port", value_name = "PORT")]
    pub port: Option<u16>,
    #[clap(short('l'), long = "room-id", value_name = "ROOM_ID")]
    pub room_id: Option<String>,
    #[clap(short('t'), long = "token", value_name = "TOKEN")]
    pub token: Option<String>,
    #[clap(
        short('w'),
        long = "watch",
        value_name = "WATCH_MODE",
        default_value_t = false
    )]
    pub watch: bool,
    #[clap(long = "retrieve-token-interval", default_value_t = 3, help="seconds")]
    pub retrieve_token_interval: u64,
    #[clap(long = "retrieve-token-offset", default_value_t = 2, help="seconds, positive for delay")]
    pub retrieve_token_advance_offset: i64,
    #[clap(short('i'), long = "immediate", default_value_t = false, help="immediate mode, will not wait for token to be retrieved")]
    pub immediate: bool,
    #[clap(long = "test", default_value_t = false, help="test mode, will not connect to server")]
    pub test: bool,
    #[clap(long = "test-result-file", value_name = "FILE")]
    pub test_result_file: Option<String>,
}

pub struct AlsConnectionInfo {
    pub address: Option<String>,
    pub port: Option<u16>,
    pub room_id: Option<String>,
    pub token: Option<String>,
}

pub fn run(ctx: &Global, connection_info: AlsConnectionInfo, args: &ArgsALS) -> Result<()> {
    if args.test {
        let api_client = &ctx.api_client;
        let plan_list = api_client.high_level().get_plan_list()?;
        let res = try_get_tokens(api_client, &plan_list)?;
        if res.iter().all(|r| r.is_err()) {
            return Err(anyhow::anyhow!("No valid tokens found"));
        }
        let mut test_results: Vec<serde_json::Value> = Vec::new();
        for (i, r) in res.iter().enumerate() {
            let item = plan_list.as_array().unwrap().get(i).unwrap();
            let live_id = item["live_id"].as_str().unwrap();
            let live_type = item["live_type"].as_u64().unwrap();
            let name = item["name"].as_str().unwrap();
            if let Ok(token) = r {
                let payload_json = extract_jwt_payload(token.as_str())?;
                tracing::info!(
                    "Dry run mode: address: {}, port: {}, room_id: {}",
                    payload_json["pod"]["address"].as_str().unwrap(),
                    payload_json["pod"]["port"].as_u64().unwrap(),
                    payload_json["room_id"].as_str().unwrap()
                );
                tracing::debug!("Token: {}", token);
                test_results.push(serde_json::json!({
                    "name": name,
                    "live_id": live_id,
                    "live_type": live_type,
                    "address": payload_json["pod"]["address"].as_str().unwrap(),
                    "port": payload_json["pod"]["port"].as_u64().unwrap(),
                    "room_id": payload_json["room_id"].as_str().unwrap(),
                }));
            } else {
                tracing::warn!("Failed to retrieve token for live:  name: {}, live_id: {}", name, live_id);
            }
        }
        if let Some(test_result_file) = args.test_result_file.clone() {
            let mut file = File::create(test_result_file)?;
            serde_json::to_writer_pretty(&mut file, &test_results)?;
        }
        return Ok(());
    }
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
        if args.immediate {
            fetch_connection_info_immediate(ctx, connection_info)?
        } else {
            fetch_connection_info(ctx, connection_info, args.watch, args.retrieve_token_interval, args.retrieve_token_advance_offset)?
        }
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

fn fetch_token(api_client: &ApiClient, live_type: u64, live_id: &str) -> Result<String> {
    if live_type == 2 {
        return api_client
            .high_level()
            .get_with_meets_connect_token(&live_id);
    }
    if live_type == 1 {
        return api_client.high_level().get_fes_live_connect_token(&live_id);
    }
    return Err(anyhow::anyhow!(
        "Unsupported live type: {}. Only 1 (FES) and 2 (Meets) are supported.",
        live_type
    ));
}

fn get_connect_info(token: String, connection_info: AlsConnectionInfo) -> Result<ConnectionInfo> {
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

fn try_get_tokens(api_client: &ApiClient, plan_list: &serde_json::Value) -> Result<Vec<Result<String>>> {
    let mut tokens = Vec::new();
    for item in plan_list.as_array().unwrap() {
        tokens.push(get_token(api_client, item));
    }
    if tokens.is_empty() {
        return Err(anyhow::anyhow!("No valid tokens found"));
    }
    Ok(tokens)
}

fn get_token(api_client: &ApiClient, item: &serde_json::Value) -> Result<String> {
    let live_id = item.get("live_id").unwrap().as_str().unwrap().to_string();
    let live_type = item.get("live_type").unwrap().as_u64().unwrap();
    fetch_token(api_client, live_type, &live_id)
}

fn get_latest_token(ctx: &Global) -> Result<String> {
    let api_client = &ctx.api_client;
    let plan_list = api_client.high_level().get_plan_list()?;
    let res: Option<&serde_json::Value> = plan_list.as_array().unwrap().first();
    if res.is_none() {
        return Err(anyhow::anyhow!("No plan found"));
    }
    get_token(api_client, res.unwrap())
}

fn fetch_connection_info_immediate(
    ctx: &Global,
    connection_info: AlsConnectionInfo,
) -> Result<ConnectionInfo> {
    let token = get_latest_token(ctx)?;
    get_connect_info(token, connection_info)
}

fn fetch_connection_info(
    ctx: &Global,
    connection_info: AlsConnectionInfo,
    watch_mode: bool,
    retrieve_token_interval: u64,
    retrieve_token_advance_offset: i64,
) -> Result<ConnectionInfo> {
    let api_client = &ctx.api_client;
    let plan_list = api_client.high_level().get_plan_list()?;
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
    let start_time_offset: TimeDelta = Duration::minutes(10) - Duration::seconds(retrieve_token_advance_offset);
    if now < live_start_time - start_time_offset {
        if !watch_mode {
            return Err(anyhow::anyhow!("The plan has not started yet"));
        }
        let pb = ProgressBar::new_spinner();
        pb.enable_steady_tick(std::time::Duration::from_millis(80));
        pb.set_style(
            ProgressStyle::with_template("{spinner:.blue} {msg}")
                .unwrap()
                .tick_strings(&cli::spinner::TICK_CLOCK),
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
            match fetch_token(api_client, live_type, &live_id) {
                Ok(t) => {
                    token = t;
                    break;
                }
                Err(e) => {
                    tracing::error!("Failed to retrieve token: {}", e);
                }
            }
        }

        tracing::info!("Cannot get token, retrying in 3 seconds...");
        retry_count += 1;
        if retry_count > max_retry {
            return Err(anyhow::anyhow!("Max retry limit reached"));
        }
        std::thread::sleep(std::time::Duration::from_secs(retrieve_token_interval));
    }
    get_connect_info(token, connection_info)
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
