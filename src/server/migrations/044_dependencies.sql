-- 044_dependencies.sql
ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS dependencies JSONB NOT NULL DEFAULT '[]'::jsonb;
