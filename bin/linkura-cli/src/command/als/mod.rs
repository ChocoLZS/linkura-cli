use std::{
    collections::{HashMap, HashSet},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};

use anyhow::{Context, Result};
use chrono::{DateTime, Local, Utc};
use clap::Args as ClapArgs;
use tokio::{
    sync::{mpsc, watch},
    task::JoinSet,
};

use crate::config::Global;

mod api_runtime;
mod model;
mod packer;
mod poller;
mod record_task;
mod retry;
mod scheduler;
mod test_mode;

use api_runtime::ApiRuntime;
use model::{JobKey, PlanItem};
use record_task::RecordTask;
use scheduler::ScheduledTimer;

#[derive(Debug, Clone, ClapArgs)]
pub struct ArgsALS {
    #[clap(
        short = 'i',
        long = "immediate",
        default_value_t = false,
        help = "run one RecordTask immediately with latest plan before scheduler loop"
    )]
    pub immediate: bool,
    #[clap(
        long = "poll-interval",
        default_value_t = 3600,
        help = "seconds, poll interval for new With Meets and Fes Live recordings"
    )]
    pub poll_interval: u64,
    #[clap(
        long = "record-retries",
        default_value_t = 5,
        help = "retry times for the record task"
    )]
    pub record_retries: u32,
    #[clap(
        long = "advance-seconds",
        default_value_t = 0,
        help = "trigger RecordTask this many seconds before open_time (e.g. 3600 means 1 hour early)"
    )]
    pub advance_seconds: i64,
    #[clap(
        long = "raw-dir",
        default_value = "data/als",
        help = "root directory for raw recording files"
    )]
    pub raw_dir: String,
    #[clap(
        long = "archive-dir",
        default_value = "data/als-archive",
        help = "root directory for tar.gz archives"
    )]
    pub archive_dir: String,
    #[clap(
        long = "test",
        default_value_t = false,
        help = "test mode, validate token/connect info only"
    )]
    pub test: bool,
    #[clap(long = "test-result-file", value_name = "FILE")]
    pub test_result_file: Option<String>,
    #[clap(long = "live-id")]
    pub live_id: Option<String>,
    #[clap(long = "live-type")]
    pub live_type: Option<u64>,
}

pub async fn run(ctx: &mut Global, args: &ArgsALS) -> Result<()> {
    if args.test {
        return test_mode::run(ctx, args).await;
    }

    let api = Arc::new(ApiRuntime::from_global(ctx));
    if args.immediate {
        run_immediate_once(api, args.clone()).await
    } else {
        run_supervisor(api, args.clone()).await
    }
}

async fn run_supervisor(api: Arc<ApiRuntime>, args: ArgsALS) -> Result<()> {
    let (plan_tx, mut plan_rx) = mpsc::channel::<Vec<PlanItem>>(8);
    let (trigger_tx, mut trigger_rx) = mpsc::channel::<PlanItem>(256);
    let (shutdown_tx, mut shutdown_rx) = watch::channel(false);

    let running_signal = Arc::new(AtomicBool::new(true));

    let ctrlc_running = running_signal.clone();
    let ctrlc_shutdown = shutdown_tx.clone();
    tokio::spawn(async move {
        if tokio::signal::ctrl_c().await.is_ok() {
            tracing::info!("Received Ctrl+C signal, shutting down...");
            ctrlc_running.store(false, Ordering::Relaxed);
            let _ = ctrlc_shutdown.send(true);
        }
    });

    tokio::spawn(poller::plan_poller_task(
        api.clone(),
        args.poll_interval,
        plan_tx,
        shutdown_tx.subscribe(),
    ));

    let mut scheduled: HashMap<JobKey, ScheduledTimer> = HashMap::new();
    let mut running: HashSet<JobKey> = HashSet::new();
    let mut record_tasks: JoinSet<(JobKey, Result<()>)> = JoinSet::new();

    loop {
        tokio::select! {
            changed = shutdown_rx.changed() => {
                if changed.is_err() || *shutdown_rx.borrow() {
                    break;
                }
            }
            plans = plan_rx.recv() => {
                if let Some(plans) = plans {
                    scheduler::reconcile_schedule(
                        plans,
                        &mut scheduled,
                        &running,
                        &trigger_tx,
                        &shutdown_tx,
                        args.advance_seconds,
                    );
                    log_status(&scheduled, &running);
                } else {
                    break;
                }
            }
            trigger = trigger_rx.recv() => {
                if let Some(item) = trigger {
                    let key = item.key();
                    if running.contains(&key) {
                        continue;
                    }
                    if let Some(timer) = scheduled.remove(&key) {
                        timer.handle.abort();
                    }

                    let api_clone = api.clone();
                    let args_clone = args.clone();
                    let signal_clone = running_signal.clone();
                    let key_clone = key.clone();

                    running.insert(key.clone());
                    log_status(&scheduled, &running);
                    record_tasks.spawn(async move {
                        let result = RecordTask::new(api_clone, item, args_clone, signal_clone)
                            .run()
                            .await;
                        (key_clone, result)
                    });
                }
            }
            done = record_tasks.join_next(), if !record_tasks.is_empty() => {
                if let Some(done) = done {
                    match done {
                        Ok((key, result)) => {
                            running.remove(&key);
                            log_status(&scheduled, &running);
                            match result {
                                Ok(()) => tracing::info!(
                                    "Record job done: live_id={}, live_type={}",
                                    key.live_id, key.live_type
                                ),
                                Err(e) => tracing::error!(
                                    "Record job failed: live_id={}, live_type={}, err={}",
                                    key.live_id, key.live_type, e
                                ),
                            }
                        }
                        Err(e) => {
                            tracing::error!("Record job join failed: {}", e);
                        }
                    }
                }
            }
        }
    }

    for (_, timer) in scheduled.drain() {
        timer.handle.abort();
    }
    let _ = shutdown_tx.send(true);
    running_signal.store(false, Ordering::Relaxed);

    while let Some(done) = record_tasks.join_next().await {
        if let Err(e) = done {
            tracing::error!("Record job join failed during shutdown: {}", e);
        }
    }

    Ok(())
}

fn log_status(scheduled: &HashMap<JobKey, ScheduledTimer>, running: &HashSet<JobKey>) {
    let mut queued_rows: Vec<(i64, String, u64, String)> = scheduled
        .iter()
        .map(|(k, timer)| {
            let when = DateTime::<Utc>::from_timestamp_millis(timer.trigger_at_millis)
                .map(|dt| dt.with_timezone(&Local).format("%Y-%m-%d %H:%M:%S").to_string())
                .unwrap_or_else(|| timer.trigger_at_millis.to_string());
            (
                timer.trigger_at_millis,
                k.live_id.clone(),
                k.live_type,
                format!("{}:{}@{}", k.live_id, k.live_type, when),
            )
        })
        .collect();
    queued_rows.sort_by(|a, b| {
        a.0.cmp(&b.0)
            .then_with(|| a.1.cmp(&b.1))
            .then_with(|| a.2.cmp(&b.2))
    });

    let mut running_rows: Vec<String> = running
        .iter()
        .map(|k| format!("{}:{}", k.live_id, k.live_type))
        .collect();
    running_rows.sort();

    tracing::info!(
        "ALS status => queued={}, running={}",
        queued_rows.len(),
        running_rows.len()
    );
    if !queued_rows.is_empty() {
        tracing::info!(
            "ALS queued => {}",
            queued_rows
                .iter()
                .map(|(_, _, _, row)| row.as_str())
                .collect::<Vec<_>>()
                .join(" | ")
        );
    }
    if !running_rows.is_empty() {
        tracing::info!("ALS running => {}", running_rows.join(" | "));
    }
}

async fn select_immediate_item(api: Arc<ApiRuntime>, args: &ArgsALS) -> Result<Option<PlanItem>> {
    let designated_live_id = args.live_id.clone();
    let designated_live_type = args.live_type;
    let plans = api
        .get_plan_items_with_auto_refresh()
        .await
        .context("failed while loading plans for immediate mode")?;

    if let (Some(live_id), Some(live_type)) = (designated_live_id, designated_live_type) {
        return Ok(plans
            .into_iter()
            .find(|p| p.live_id == live_id && p.live_type == live_type));
    }

    Ok(plans.into_iter().next())
}

async fn run_immediate_once(api: Arc<ApiRuntime>, args: ArgsALS) -> Result<()> {
    let item = select_immediate_item(api.clone(), &args)
        .await?
        .context("immediate mode: no valid plan found")?;
    let running_signal = Arc::new(AtomicBool::new(true));
    let ctrlc_signal = running_signal.clone();
    let ctrlc = tokio::spawn(async move {
        if tokio::signal::ctrl_c().await.is_ok() {
            ctrlc_signal.store(false, Ordering::Relaxed);
        }
    });

    let result = RecordTask::new(api, item, args, running_signal).run().await;
    ctrlc.abort();
    result
}
