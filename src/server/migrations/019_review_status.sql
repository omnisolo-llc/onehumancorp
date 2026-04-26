-- 019_review_status.sql
-- Add index and update check constraint for review status

CREATE INDEX IF NOT EXISTS idx_swarm_tasks_status_locked_until ON swarm_tasks(status, locked_until);

ALTER TABLE swarm_tasks DROP CONSTRAINT IF EXISTS swarm_tasks_status_check;
ALTER TABLE swarm_tasks ADD CONSTRAINT swarm_tasks_status_check CHECK (status IN ('PENDING', 'IN_PROGRESS', 'REVIEW', 'COMPLETED', 'FAILED'));
