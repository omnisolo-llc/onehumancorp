-- Create an index for faster polling of PENDING and REVIEW tasks
CREATE INDEX IF NOT EXISTS idx_swarm_tasks_status_locked ON swarm_tasks (status, locked_until);
CREATE INDEX IF NOT EXISTS idx_swarm_tasks_agent ON swarm_tasks (assigned_agent_id);
