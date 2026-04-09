ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS parent_plan_id TEXT;
ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS dependencies JSONB;
ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS assigned_agent_id TEXT;
