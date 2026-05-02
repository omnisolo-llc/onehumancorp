-- 054_shared_tasks_schema_sync.sql
-- Synchronize shared_tasks schema with SharedTask struct

ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS mission_id TEXT;
ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS parent_plan_id TEXT;
ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS dependencies JSONB DEFAULT '[]';
ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS locked_until TIMESTAMPTZ;
ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS ultraplan_phase TEXT;
ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS deliberation_log TEXT;
ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS depth INTEGER;
ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS action_risk TEXT;
ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS approval_status TEXT;
ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS proposed_content TEXT;
ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS auto_dreamed BOOLEAN DEFAULT FALSE;
