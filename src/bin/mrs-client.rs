use anyhow::{Context, Result};
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use clap::Parser;


use linkura_client::mrs::{MrsClient, MrsConnectionInfo};
use linkura_client::log;

/** ARG PARSER **/
#[derive(Parser, Debug)]
#[command(
    name = "linkura-mrs-client",
    version = "0.0.0",
    author = "ChocoLZS, chocoielzs@outlook.com",
    about = "Interactive MRS Client for Linkura",
    long_about = None,
    bin_name = "mrs-client",
)]
pub struct Args {
    #[clap(short('a'), long = "address", value_name = "ADDRESS")]
    pub addr: String,
    #[clap(short('p'), long = "port", value_name = "PORT", default_value_t = 21011)]
    pub port: u16,
    #[clap(short('r'), long = "room-id", value_name = "ROOM_ID")]
    pub room_id: u32,
    #[clap(short('i'), long = "player-id", value_name = "PLAYER_ID")]
    pub player_id: u16,
}
fn main() -> Result<()> {
    let args = Args::parse();
    log::init();
    let running_signal = Arc::new(AtomicBool::new(true));
    let running_signal_clone = running_signal.clone();

    ctrlc::set_handler(move || {
        tracing::info!("Received Ctrl+C signal, initiating graceful shutdown...");
        running_signal_clone.store(false, Ordering::Relaxed);
    })
    .context("Error setting Ctrl+C handler")?;

    // 创建客户端实例
    let mut client = MrsClient::new(
        running_signal,
        MrsConnectionInfo {
            host: args.addr,
            port: args.port,
            room_id: args.room_id,
            player_id: args.player_id,
        },
        None
    )
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
