-- 023_tasks_review.sql
-- Add index on shared_tasks status if not exists

CREATE INDEX IF NOT EXISTS idx_shared_tasks_status ON shared_tasks(status);
