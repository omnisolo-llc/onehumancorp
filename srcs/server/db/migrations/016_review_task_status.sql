-- Drop constraint on status to allow REVIEW
ALTER TABLE swarm_tasks DROP CONSTRAINT IF EXISTS swarm_tasks_status_check;
ALTER TABLE swarm_tasks ADD CONSTRAINT swarm_tasks_status_check CHECK (status IN ('PENDING', 'IN_PROGRESS', 'REVIEW', 'COMPLETED', 'FAILED'));

-- Add indexes for polling
CREATE INDEX IF NOT EXISTS idx_swarm_tasks_status_locked_until ON swarm_tasks(status, locked_until);
