-- 072_ohc_tasks_parent_workflow.sql

ALTER TABLE ohc_tasks ADD COLUMN IF NOT EXISTS parent_task_id TEXT;
ALTER TABLE ohc_tasks ADD COLUMN IF NOT EXISTS workflow_state TEXT;
