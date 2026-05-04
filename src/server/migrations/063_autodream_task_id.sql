-- 063_autodream_task_id.sql

ALTER TABLE autodream_memories ADD COLUMN IF NOT EXISTS task_id TEXT;
