use std::sync::Arc;
use tokio;
use crate::queue::{TaskQueue, MemoryTaskQueue, Job};
use chrono::Utc;

#[tokio::main]
async fn main() {
    let mem_queue = MemoryTaskQueue::new();
    let job = Job {
        id: "job_1".to_string(),
        tenant_id: "test".to_string(),
        parent_task_id: "parent_1".to_string(),
        agent_role: "test_agent".to_string(),
        payload: "{}".to_string(),
        status: "PENDING".to_string(),
        attempts: 0,
        max_attempts: 3,
        run_after: Utc::now(),
        locked_until: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    mem_queue.enqueue_batch(vec![job]).await.unwrap();
    let result = mem_queue.dequeue(vec!["test_agent".to_string()]).await.unwrap();
    println!("Dequeue result: {:?}", result.is_some());
}
