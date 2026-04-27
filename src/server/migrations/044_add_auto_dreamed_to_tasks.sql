-- 044_add_auto_dreamed_to_tasks.sql
ALTER TABLE tasks ADD COLUMN IF NOT EXISTS auto_dreamed BOOLEAN DEFAULT FALSE;
