pub struct MemoryTaskQueue {
    jobs: DashMap<String, Job>,
    role_queues: DashMap<String, Mutex<VecDeque<String>>>,
}

impl MemoryTaskQueue {
    pub fn new() -> Self {
        MemoryTaskQueue {
            jobs: DashMap::new(),
            role_queues: DashMap::new(),
        }
    }
}

#[async_trait]
impl TaskQueue for MemoryTaskQueue {
    async fn enqueue_batch(&self, jobs: Vec<Job>) -> Result<(), String> {
        let mut grouped: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();

        for job in jobs {
            let role = job.agent_role.clone();
            let id = job.id.clone();
            self.jobs.insert(id.clone(), job);
            grouped.entry(role).or_insert_with(Vec::new).push(id);
        }

        for (role, ids) in grouped {
            let queue = self.role_queues.entry(role).or_insert_with(|| Mutex::new(VecDeque::new()));
            let mut q = queue.lock().unwrap();
            q.extend(ids);
        }

        Ok(())
    }

    async fn enqueue(&self, job: Job) -> Result<(), String> {
        let role = job.agent_role.clone();
        let id = job.id.clone();
        self.jobs.insert(id.clone(), job);

        let queue = self.role_queues.entry(role).or_insert_with(|| Mutex::new(VecDeque::new()));
        let mut q = queue.lock().unwrap();
        q.push_back(id);

        Ok(())
    }

    async fn dequeue(&self, roles: Vec<String>) -> Result<Option<Job>, String> {
        for role in roles {
            if let Some(queue) = self.role_queues.get(&role) {
                let mut q = queue.lock().unwrap();
                // Pop until we find a valid pending job, or queue is empty
                while let Some(job_id) = q.pop_front() {
                    if let Some(mut job_ref) = self.jobs.get_mut(&job_id) {
                        if job_ref.status == "PENDING" {
                            job_ref.status = "IN_PROGRESS".to_string();
                            job_ref.updated_at = Utc::now();
                            return Ok(Some(job_ref.clone()));
                        }
                    }
                }
            }
        }
        Ok(None)
    }

    async fn complete(&self, job_id: &str, tenant_id: &str) -> Result<(), String> {
        if let Some(mut job) = self.jobs.get_mut(job_id) {
            if job.tenant_id != tenant_id {
                return Err("tenant mismatch".to_string());
            }
            job.status = "COMPLETED".to_string();
            job.updated_at = Utc::now();
            Ok(())
        } else {
            Err("job not found".to_string())
        }
    }

    async fn fail(&self, job_id: &str, tenant_id: &str, reason: &str) -> Result<(), String> {
        if let Some(mut job) = self.jobs.get_mut(job_id) {
            if job.tenant_id != tenant_id {
                return Err("tenant mismatch".to_string());
            }
            job.status = "FAILED".to_string();
            job.payload = format!("{} (Reason: {})", job.payload, reason);
            job.updated_at = Utc::now();
            Ok(())
        } else {
            Err("job not found".to_string())
        }
    }

    async fn requeue(&self, job: Job) -> Result<(), String> {
        let role = job.agent_role.clone();
        let id = job.id.clone();
        self.jobs.insert(id.clone(), job);

        let queue = self.role_queues.entry(role).or_insert_with(|| Mutex::new(VecDeque::new()));
        let mut q = queue.lock().unwrap();
        q.push_back(id);
        Ok(())
    }
}
