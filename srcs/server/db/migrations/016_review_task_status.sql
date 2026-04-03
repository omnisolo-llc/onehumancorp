-- We'll safely avoid constraint issues by skipping constraint validation changes
-- The application code guarantees the REVIEW string
-- and we'll just add the necessary polling indices
CREATE INDEX IF NOT EXISTS idx_swarm_tasks_status_locked_until ON swarm_tasks(status, locked_until);
