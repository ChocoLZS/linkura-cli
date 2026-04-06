use std::sync::Arc;

use tokio::{
    sync::{mpsc, watch},
    time::Duration,
};

use super::{api_runtime::ApiRuntime, model::PlanItem};

pub async fn plan_poller_task(
    api: Arc<ApiRuntime>,
    poll_interval: u64,
    tx: mpsc::Sender<Vec<PlanItem>>,
    mut shutdown_rx: watch::Receiver<bool>,
) {
    let interval_seconds = poll_interval;
    let mut ticker = tokio::time::interval(Duration::from_secs(interval_seconds));

    loop {
        tokio::select! {
            changed = shutdown_rx.changed() => {
                if changed.is_err() || *shutdown_rx.borrow() {
                    break;
                }
            }
            _ = ticker.tick() => {
                match api.get_plan_items_with_auto_refresh().await {
                    Ok(plans) => {
                        if tx.send(plans).await.is_err() {
                            break;
                        }
                    }
                    Err(e) => tracing::warn!("Plan polling failed: {}", e),
                }
            }
        }
    }
}
