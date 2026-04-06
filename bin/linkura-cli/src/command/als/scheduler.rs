use std::collections::{HashMap, HashSet};

use chrono::{Duration as ChronoDuration, Utc};
use tokio::{
    sync::{mpsc, watch},
    task::JoinHandle,
};

use super::model::{JobKey, PlanItem};

#[derive(Debug)]
pub struct ScheduledTimer {
    pub handle: JoinHandle<()>,
    pub trigger_at_millis: i64,
}

pub fn reconcile_schedule(
    plans: Vec<PlanItem>,
    scheduled: &mut HashMap<JobKey, ScheduledTimer>,
    running: &HashSet<JobKey>,
    trigger_tx: &mpsc::Sender<PlanItem>,
    shutdown_tx: &watch::Sender<bool>,
    advance_seconds: i64,
) {
    let mut desired: HashMap<JobKey, PlanItem> = HashMap::new();
    for plan in plans {
        let key = plan.key();
        match desired.get(&key) {
            Some(existing) if existing.live_open_time >= plan.live_open_time => {}
            _ => {
                desired.insert(key, plan);
            }
        }
    }

    let keys_to_remove: Vec<JobKey> = scheduled
        .keys()
        .filter(|k| !desired.contains_key(*k))
        .cloned()
        .collect();
    for key in keys_to_remove {
        if let Some(timer) = scheduled.remove(&key) {
            timer.handle.abort();
        }
    }

    for (key, item) in desired {
        if running.contains(&key) {
            continue;
        }

        let effective_advance = advance_seconds.max(0);
        let trigger_time = item.live_open_time - ChronoDuration::seconds(effective_advance);
        let trigger_at_millis = trigger_time.timestamp_millis();
        if let Some(existing) = scheduled.get(&key) {
            if existing.trigger_at_millis == trigger_at_millis {
                continue;
            }
        }
        if let Some(timer) = scheduled.remove(&key) {
            timer.handle.abort();
        }

        let now = Utc::now();
        let delay = if trigger_time > now {
            (trigger_time - now)
                .to_std()
                .unwrap_or_else(|_| std::time::Duration::from_secs(0))
        } else {
            std::time::Duration::from_secs(0)
        };

        let mut shutdown_rx = shutdown_tx.subscribe();
        let tx = trigger_tx.clone();
        let item_clone = item.clone();
        let handle = tokio::spawn(async move {
            if delay > std::time::Duration::from_secs(0) {
                tokio::select! {
                    _ = tokio::time::sleep(delay) => {}
                    _ = shutdown_rx.changed() => return,
                }
            }
            let _ = tx.send(item_clone).await;
        });

        scheduled.insert(
            key,
            ScheduledTimer {
                handle,
                trigger_at_millis,
            },
        );
    }
}
