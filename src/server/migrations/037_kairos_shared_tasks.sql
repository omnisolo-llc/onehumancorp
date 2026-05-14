-- 037_kairos_shared_tasks.sql
-- Add index on shared_tasks id if not exists (redundant but following history)

CREATE INDEX IF NOT EXISTS idx_shared_tasks_id ON shared_tasks(id);
