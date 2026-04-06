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
                let api_clone = api.clone();
                let plans = tokio::task::spawn_blocking(move || api_clone.get_plan_items_with_auto_refresh()).await;
                match plans {
                    Ok(Ok(plans)) => {
                        if tx.send(plans).await.is_err() {
                            break;
                        }
                    }
                    Ok(Err(e)) => {
                        tracing::warn!("Plan polling failed: {}", e);
                    }
                    Err(e) => {
                        tracing::warn!("Plan polling task join error: {}", e);
                    }
                }
            }
        }
    }
}
