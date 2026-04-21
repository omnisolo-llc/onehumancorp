ALTER TABLE shared_tasks ADD COLUMN parent_plan_id TEXT;
ALTER TABLE shared_tasks ADD COLUMN assigned_agent_id TEXT;
ALTER TABLE shared_tasks ADD COLUMN dependencies TEXT DEFAULT '[]';
