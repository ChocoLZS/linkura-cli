use anyhow::{Context, Result};
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

use linkura_client::mrs::{MrsClient, MrsConnectionInfo};
use linkura_client::log;

const HOST: &str = "192.168.114.218";
const PORT: u16 = 21011;
const ROOM_ID: u32 = 1639274124;
const PLAYER_ID: u16 = 11998;

#[tokio::main]
async fn main() -> Result<()> {
    log::init();
    // 设置优雅退出信号处理
    let running = Arc::new(AtomicBool::new(true));
    let running_clone = running.clone();

    ctrlc::set_handler(move || {
        tracing::info!("Received Ctrl+C signal, initiating graceful shutdown...");
        running_clone.store(false, Ordering::Relaxed);
    })
    .context("Error setting Ctrl+C handler")?;

    // 创建客户端实例
    let mut client = MrsClient::new(
        running,
        MrsConnectionInfo {
            host: HOST.to_string(),
            port: PORT,
            room_id: ROOM_ID,
            player_id: PLAYER_ID,
        },
    )
    .context("Failed to create MrsClient instance")?;

    tracing::info!("MRS Client starting...");
    tracing::info!("Press Ctrl+C to gracefully shutdown");

    // 运行客户端
    match client.run().await {
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
