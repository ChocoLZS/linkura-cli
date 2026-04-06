use std::future::Future;

use anyhow::Result;
use tokio::time::Duration;

pub async fn retry_async<T, Op, Fut>(
    name: &str,
    max_attempts: u32,
    retry_interval: Duration,
    mut op: Op,
    mut reset_on_error: impl FnMut(&anyhow::Error) -> bool,
) -> Result<T>
where
    Op: FnMut(u32) -> Fut,
    Fut: Future<Output = Result<T>>,
{
    let attempts = max_attempts.max(1);
    let mut attempt = 1;
    loop {
        match op(attempt).await {
            Ok(v) => return Ok(v),
            Err(e) => {
                if reset_on_error(&e) {
                    tracing::warn!(
                        "{} got progress error, reset retry counter and retry in {:?}: {}",
                        name,
                        retry_interval,
                        e
                    );
                    attempt = 1;
                    tokio::time::sleep(retry_interval).await;
                    continue;
                }
                if attempt >= attempts {
                    return Err(e);
                }
                tracing::warn!(
                    "{} failed on attempt {}/{}, retry in {:?}: {}",
                    name,
                    attempt,
                    attempts,
                    retry_interval,
                    e
                );
                attempt += 1;
                tokio::time::sleep(retry_interval).await;
            }
        }
    }
}
