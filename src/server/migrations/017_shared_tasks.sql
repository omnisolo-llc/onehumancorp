-- 017_shared_tasks.sql
-- Add columns to shared_tasks from Go migration 014

ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS parent_plan_id TEXT;
ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS assigned_agent_id TEXT;
ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS dependencies TEXT DEFAULT '[]';
