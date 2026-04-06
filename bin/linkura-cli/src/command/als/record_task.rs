use std::{
    fs,
    path::{Path, PathBuf},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};

use anyhow::{Context, Result};
use linkura_packet::als::client::{Client, ClientConfig, ConnectionInfo};
use tokio::time::Duration;

use super::{
    ArgsALS,
    api_runtime::{ApiRuntime, is_connect_token_room_unavailable_error},
    model::PlanItem,
    packer, retry,
};

pub struct RecordTask {
    api: Arc<ApiRuntime>,
    item: PlanItem,
    args: ArgsALS,
    running_signal: Arc<AtomicBool>,
}

const ALS_PROGRESS_RETRY_RESET: &str = "ALS_PROGRESS_RETRY_RESET";

impl RecordTask {
    pub fn new(
        api: Arc<ApiRuntime>,
        item: PlanItem,
        args: ArgsALS,
        running_signal: Arc<AtomicBool>,
    ) -> Self {
        Self {
            api,
            item,
            args,
            running_signal,
        }
    }

    pub async fn run(self) -> Result<()> {
        let retry_interval = Duration::from_millis(1);
        retry::retry_async(
            "RecordTask::_run",
            self.args.record_retries,
            retry_interval,
            |_| self._run(),
            is_progress_reset_error,
        )
        .await
    }

    async fn _run(&self) -> Result<()> {
        if !self.running_signal.load(Ordering::Relaxed) {
            tracing::info!(
                "RecordTask shutdown requested before run: live_id={}, live_type={}",
                self.item.live_id,
                self.item.live_type
            );
            return Ok(());
        }

        // token refreshed here
        if !self.still_in_plan().await? {
            tracing::info!(
                "Skip job because live disappeared from plan list: live_id={}, live_type={}",
                self.item.live_id,
                self.item.live_type
            );
            self.package_existing_raw_dir_if_any("live_disappeared").await?;
            return Ok(());
        }

        let connection_info: ConnectionInfo = match self.fetch_connection_info().await {
            Ok(v) => v,
            Err(e) if is_connect_token_room_unavailable_error(&e) => {
                tracing::warn!(
                    "connect_token room unavailable (error_code=14002_111604), treat as normal completion: live_id={}, live_type={}",
                    self.item.live_id,
                    self.item.live_type
                );
                self.package_existing_raw_dir_if_any("room_unavailable").await?;
                return Ok(());
            }
            Err(e) => return Err(e),
        };
        let raw_dir = packer::create_task_raw_dir(Path::new(&self.args.raw_dir), &self.item)?;
        self.run_client(connection_info, raw_dir.clone()).await?;
        if !self.running_signal.load(Ordering::Relaxed) {
            tracing::info!(
                "RecordTask shutdown requested, continue packaging before exit: live_id={}, live_type={}",
                self.item.live_id,
                self.item.live_type
            );
        }
        self.package(raw_dir).await?;
        Ok(())
    }

    async fn still_in_plan(&self) -> Result<bool> {
        let api = self.api.clone();
        let live_id = self.item.live_id.clone();
        let live_type = self.item.live_type;
        tokio::task::spawn_blocking(move || {
            // RecordTask requirement: always refresh token before API call.
            api.refresh_token()?;
            api.contains_live(&live_id, live_type)
        })
        .await
        .context("join failed when checking plan existence")?
    }

    async fn fetch_connection_info(&self) -> Result<ConnectionInfo> {
        if !self.running_signal.load(Ordering::Relaxed) {
            return Err(anyhow::anyhow!("shutdown requested before connect"));
        }
        let api = self.api.clone();
        let live_id = self.item.live_id.clone();
        let live_type = self.item.live_type;
        tokio::task::spawn_blocking(move || {
            // RecordTask requirement: always refresh token before API call.
            api.refresh_token()?;
            api.fetch_live_connection_info(&live_id, live_type)
        })
        .await
        .context("join failed while fetching connection info")?
    }

    async fn run_client(&self, connection_info: ConnectionInfo, raw_dir: PathBuf) -> Result<()> {
        let running_signal = self.running_signal.clone();
        let live_id = self.item.live_id.clone();
        let live_type = self.item.live_type;
        tracing::info!(
            "ALS client run start: live_id={}, live_type={}, raw_dir={}",
            live_id,
            live_type,
            raw_dir.display()
        );
        tokio::task::spawn_blocking(move || -> Result<()> {
            let before = snapshot_bin_state(&raw_dir)?;
            let config = ClientConfig {
                data_directory: raw_dir.to_string_lossy().to_string(),
                ..ClientConfig::default()
            };
            let mut client = Client::new(running_signal, connection_info, Some(config))
                .context("failed to build ALS client")?;
            match client.run() {
                Ok(()) => Ok(()),
                Err(err) => {
                    let after = snapshot_bin_state(&raw_dir)?;
                    if after.total_bytes > before.total_bytes || after.bin_files > before.bin_files {
                        return Err(anyhow::anyhow!(ALS_PROGRESS_RETRY_RESET)
                            .context(format!("ALS client failed after stream progressed: {}", err)));
                    }
                    Err(err).context("ALS client run failed")
                }
            }
        })
        .await
        .context("join failed while running ALS client")??;
        tracing::info!(
            "ALS client run finished: live_id={}, live_type={}",
            self.item.live_id,
            self.item.live_type
        );
        Ok(())
    }

    async fn package(&self, raw_dir: PathBuf) -> Result<()> {
        let archive_root = PathBuf::from(self.args.archive_dir.clone());
        let item = self.item.clone();
        tracing::info!(
            "Pack start: live_id={}, live_type={}, source_dir={}, archive_root={}",
            item.live_id,
            item.live_type,
            raw_dir.display(),
            archive_root.display()
        );
        tokio::task::spawn_blocking(move || {
            packer::package_recording(&raw_dir, &archive_root, &item)
        })
        .await
        .context("join failed while packaging record")??;
        Ok(())
    }

    async fn package_existing_raw_dir_if_any(&self, reason: &str) -> Result<()> {
        let raw_dir = packer::task_raw_dir(Path::new(&self.args.raw_dir), &self.item);
        if !raw_dir.exists() {
            tracing::info!(
                "Skip package on {}: raw dir not found: live_id={}, live_type={}, raw_dir={}",
                reason,
                self.item.live_id,
                self.item.live_type,
                raw_dir.display()
            );
            return Ok(());
        }
        tracing::info!(
            "Package existing raw dir on {}: live_id={}, live_type={}, raw_dir={}",
            reason,
            self.item.live_id,
            self.item.live_type,
            raw_dir.display()
        );
        self.package(raw_dir).await
    }
}

#[derive(Default, Clone, Copy)]
struct BinState {
    bin_files: u64,
    total_bytes: u64,
}

fn snapshot_bin_state(raw_dir: &Path) -> Result<BinState> {
    if !raw_dir.exists() {
        return Ok(BinState::default());
    }

    let mut state = BinState::default();
    for entry in fs::read_dir(raw_dir).with_context(|| {
        format!("failed to read raw dir while snapshotting: {}", raw_dir.display())
    })? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        if !file_type.is_file() {
            continue;
        }

        let path = entry.path();
        if path
            .extension()
            .and_then(|s| s.to_str())
            .is_some_and(|ext| ext.eq_ignore_ascii_case("bin"))
        {
            state.bin_files += 1;
            state.total_bytes += entry.metadata()?.len();
        }
    }
    Ok(state)
}

fn is_progress_reset_error(err: &anyhow::Error) -> bool {
    err.chain()
        .any(|cause| cause.to_string().contains(ALS_PROGRESS_RETRY_RESET))
}
