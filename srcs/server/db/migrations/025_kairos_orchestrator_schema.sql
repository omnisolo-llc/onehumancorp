ALTER TABLE shared_tasks ADD COLUMN parent_plan_id TEXT;
ALTER TABLE shared_tasks ADD COLUMN dependencies JSONB NOT NULL DEFAULT '[]';
