-- Optimize the dequeue polling query to prevent sequential scans and sorting overhead
CREATE INDEX IF NOT EXISTS idx_ohc_job_queue_polling_optimized
ON ohc_job_queue(job_type, next_retry_at ASC, created_at ASC)
WHERE status = 'PENDING';
