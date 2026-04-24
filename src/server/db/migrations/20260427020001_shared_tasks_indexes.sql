ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS locked_until TIMESTAMP;
CREATE INDEX IF NOT EXISTS idx_shared_tasks_status ON shared_tasks(status);
CREATE INDEX IF NOT EXISTS idx_shared_tasks_locked_until ON shared_tasks(locked_until);
