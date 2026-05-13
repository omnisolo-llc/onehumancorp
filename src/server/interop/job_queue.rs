
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::{HashMap, BinaryHeap};
use crate::msgbus::{Bus, DistributedLock};
use crate::interop::protocol::proto;
use std::cmp::Ordering;

#[derive(Clone)]
pub struct QueuedJob {
    pub priority: i32,
    pub job_id: String, pub timestamp_ms: i64, pub job: proto::JobDispatch,
    pub retries: u32,
}

impl Ord for QueuedJob {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority)
            .then_with(|| other.timestamp_ms.cmp(&self.timestamp_ms))
    }
}

impl PartialOrd for QueuedJob {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct DistributedJobQueue {
    node_id: String,
    bus: Arc<dyn Bus>,
    lock: Arc<dyn DistributedLock>,
    queue: Arc<RwLock<BinaryHeap<QueuedJob>>>,
    dlq: Arc<RwLock<Vec<QueuedJob>>>,
}

impl DistributedJobQueue {
    pub fn new(node_id: String, bus: Arc<dyn Bus>, lock: Arc<dyn DistributedLock>) -> Self {
        Self {
            node_id, bus, lock,
            queue: Arc::new(RwLock::new(BinaryHeap::new())),
            dlq: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn enqueue(&self, job: proto::JobDispatch, priority: i32) -> Result<bool, String> {
        let mut q = self.queue.write().await;
        q.push(QueuedJob { priority, job_id: job.job_id.clone(), timestamp_ms: job.timestamp_ms, job, retries: 0 });
        Ok(true)
    }

    pub async fn process_next(&self) -> Option<proto::JobDispatch> {
        let mut q = self.queue.write().await;
        q.pop().map(|qj| qj.job)
    }

    pub async fn handle_failure(&self, job: proto::JobDispatch, current_retries: u32) {
        if current_retries >= 5 {
            let mut dlq = self.dlq.write().await;
            dlq.push(QueuedJob { priority: 0, job_id: job.job_id.clone(), timestamp_ms: job.timestamp_ms, job, retries: current_retries });
        } else {
            let mut q = self.queue.write().await;
            q.push(QueuedJob { priority: 0, job_id: job.job_id.clone(), timestamp_ms: job.timestamp_ms, job, retries: current_retries + 1 });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::msgbus::MemoryBus;

    #[tokio::test]
    async fn test_queue_priority_ordering() {
        let bus = Arc::new(MemoryBus::new());
        let lock = Arc::new(MemoryBus::new());
        let dq = DistributedJobQueue::new("node".to_string(), bus, lock);

        let mut j1 = proto::JobDispatch::default();
        j1.job_id = "low".to_string();
        let mut j2 = proto::JobDispatch::default();
        j2.job_id = "high".to_string();

        dq.enqueue(j1, 1).await.unwrap();
        dq.enqueue(j2, 10).await.unwrap();

        let first = dq.process_next().await.unwrap();
        assert_eq!(first.job_id, "high");
    }

    #[tokio::test]
    async fn test_queue_dlq() {
        let bus = Arc::new(MemoryBus::new());
        let lock = Arc::new(MemoryBus::new());
        let dq = DistributedJobQueue::new("node".to_string(), bus, lock);

        let j1 = proto::JobDispatch::default();
        dq.handle_failure(j1.clone(), 4).await;
        assert_eq!(dq.queue.read().await.len(), 1);

        dq.handle_failure(j1, 5).await;
        assert_eq!(dq.dlq.read().await.len(), 1);
    }
}

impl PartialEq for QueuedJob {
    fn eq(&self, other: &Self) -> bool {
        self.job_id == other.job_id
    }
}

impl Eq for QueuedJob {}
