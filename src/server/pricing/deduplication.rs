use std::sync::Arc;
use std::time::{Duration, Instant};
use dashmap::DashMap;
use tokio::sync::watch;
use sha2::{Sha256, Digest};

#[derive(Clone, Debug, PartialEq)]
pub struct DeduplicationResult {
    pub response: String,
}

#[derive(Clone, Debug)]
pub enum DeduplicationState {
    Pending,
    Completed(Result<DeduplicationResult, String>),
}

struct InFlightRequest {
    receiver: watch::Receiver<DeduplicationState>,
    created_at: Instant,
}

pub struct RequestDeduplicator {
    in_flight: Arc<DashMap<String, InFlightRequest>>,
    ttl: Duration,
    last_prune: tokio::sync::Mutex<Instant>,
}

impl RequestDeduplicator {
    pub fn new(ttl: Duration) -> Self {
        RequestDeduplicator {
            in_flight: Arc::new(DashMap::new()),
            ttl,
            last_prune: tokio::sync::Mutex::new(Instant::now()),
        }
    }

    fn hash_request(&self, request: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(request.as_bytes());
        hex::encode(hasher.finalize())
    }

    pub async fn deduplicate<F, Fut>(&self, request: &str, fetcher: F) -> Result<DeduplicationResult, String>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<DeduplicationResult, String>>,
    {
        // 1. Throttle prune via a mutex (do not block execution, use try_lock to lazily prune)
        if let Ok(mut last) = self.last_prune.try_lock() {
            if last.elapsed() > Duration::from_secs(1) {
                *last = Instant::now();
                self.in_flight.retain(|_, v| v.created_at.elapsed() <= self.ttl);
            }
        }

        let key = self.hash_request(request);

        let (mut rx, is_leader, tx) = {
            let mut is_leader = false;
            let mut captured_tx = None;

            let entry = self.in_flight.entry(key.clone()).or_insert_with(|| {
                is_leader = true;
                let (tx, rx) = watch::channel(DeduplicationState::Pending);
                captured_tx = Some(tx);
                InFlightRequest {
                    receiver: rx,
                    created_at: Instant::now(),
                }
            });
            (entry.receiver.clone(), is_leader, captured_tx)
        };

        if is_leader {
            let tx = tx.unwrap();
            let result = fetcher().await;

            let _ = tx.send(DeduplicationState::Completed(result.clone()));

            // Remove it from in-flight requests shortly after. It's safe if it stays slightly,
            // the watch channel retains the last value. Removing it allows subsequent new
            // calls (not batched with this one) to retry execution.
            self.in_flight.remove(&key);

            result
        } else {
            tracing::info!("💰 Miser cost optimization: Deduplicated identical concurrent LLM request.");

            // Fast path: maybe already completed?
            let state = rx.borrow().clone();
            if let DeduplicationState::Completed(res) = state {
                return res;
            }

            // Wait for completion
            while rx.changed().await.is_ok() {
                let state = rx.borrow().clone();
                if let DeduplicationState::Completed(res) = state {
                    return res;
                }
            }
            Err("Deduplication leader failed or timed out".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use tokio::time::sleep;

    #[tokio::test]
    async fn test_deduplication_success() {
        let deduplicator = Arc::new(RequestDeduplicator::new(Duration::from_secs(5)));
        let call_count = Arc::new(AtomicUsize::new(0));

        let req1 = {
            let dedup = deduplicator.clone();
            let count = call_count.clone();
            tokio::spawn(async move {
                dedup.deduplicate("test request", || async {
                    count.fetch_add(1, Ordering::SeqCst);
                    sleep(Duration::from_millis(50)).await;
                    Ok(DeduplicationResult { response: "success".to_string() })
                }).await
            })
        };

        let req2 = {
            let dedup = deduplicator.clone();
            let count = call_count.clone();
            tokio::spawn(async move {
                sleep(Duration::from_millis(10)).await;
                dedup.deduplicate("test request", || async {
                    count.fetch_add(1, Ordering::SeqCst);
                    sleep(Duration::from_millis(50)).await;
                    Ok(DeduplicationResult { response: "success".to_string() })
                }).await
            })
        };

        let req3 = {
            let dedup = deduplicator.clone();
            let count = call_count.clone();
            tokio::spawn(async move {
                sleep(Duration::from_millis(100)).await; // this arrives *after* completion!
                dedup.deduplicate("test request", || async {
                    count.fetch_add(1, Ordering::SeqCst);
                    Ok(DeduplicationResult { response: "success 3".to_string() })
                }).await
            })
        };

        let (res1, res2, res3) = tokio::join!(req1, req2, req3);

        assert_eq!(res1.unwrap().unwrap().response, "success");
        assert_eq!(res2.unwrap().unwrap().response, "success");
        assert_eq!(res3.unwrap().unwrap().response, "success 3"); // Executed again because it was removed!
        assert_eq!(call_count.load(Ordering::SeqCst), 2); // Executed twice!
    }

    #[tokio::test]
    async fn test_deduplication_different_requests() {
        let deduplicator = Arc::new(RequestDeduplicator::new(Duration::from_secs(5)));
        let call_count = Arc::new(AtomicUsize::new(0));

        let req1 = {
            let dedup = deduplicator.clone();
            let count = call_count.clone();
            tokio::spawn(async move {
                dedup.deduplicate("test request 1", || async {
                    count.fetch_add(1, Ordering::SeqCst);
                    sleep(Duration::from_millis(50)).await;
                    Ok(DeduplicationResult { response: "success 1".to_string() })
                }).await
            })
        };

        let req2 = {
            let dedup = deduplicator.clone();
            let count = call_count.clone();
            tokio::spawn(async move {
                dedup.deduplicate("test request 2", || async {
                    count.fetch_add(1, Ordering::SeqCst);
                    sleep(Duration::from_millis(50)).await;
                    Ok(DeduplicationResult { response: "success 2".to_string() })
                }).await
            })
        };

        let (res1, res2) = tokio::join!(req1, req2);

        assert_eq!(res1.unwrap().unwrap().response, "success 1");
        assert_eq!(res2.unwrap().unwrap().response, "success 2");
        assert_eq!(call_count.load(Ordering::SeqCst), 2); // Executed twice!
    }

    #[tokio::test]
    async fn test_deduplication_error_propagation() {
        let deduplicator = Arc::new(RequestDeduplicator::new(Duration::from_secs(5)));
        let call_count = Arc::new(AtomicUsize::new(0));

        let req1 = {
            let dedup = deduplicator.clone();
            let count = call_count.clone();
            tokio::spawn(async move {
                dedup.deduplicate("error request", || async {
                    count.fetch_add(1, Ordering::SeqCst);
                    sleep(Duration::from_millis(50)).await;
                    Err("failed".to_string())
                }).await
            })
        };

        let req2 = {
            let dedup = deduplicator.clone();
            let count = call_count.clone();
            tokio::spawn(async move {
                sleep(Duration::from_millis(10)).await;
                dedup.deduplicate("error request", || async {
                    count.fetch_add(1, Ordering::SeqCst);
                    sleep(Duration::from_millis(50)).await;
                    Err("failed".to_string())
                }).await
            })
        };

        let (res1, res2) = tokio::join!(req1, req2);

        assert_eq!(res1.unwrap(), Err("failed".to_string()));
        assert_eq!(res2.unwrap(), Err("failed".to_string()));
        assert_eq!(call_count.load(Ordering::SeqCst), 1);
    }
}
