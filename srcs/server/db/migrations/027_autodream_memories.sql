-- +goose Up
ALTER TABLE autodream_memories ADD COLUMN IF NOT EXISTS organization_id VARCHAR;
ALTER TABLE autodream_memories ADD COLUMN IF NOT EXISTS task_id UUID REFERENCES shared_tasks_decomposition(id);
ALTER TABLE autodream_memories ADD COLUMN IF NOT EXISTS source_type VARCHAR;

-- +goose Down
ALTER TABLE autodream_memories DROP COLUMN IF EXISTS source_type;
ALTER TABLE autodream_memories DROP COLUMN IF EXISTS task_id;
ALTER TABLE autodream_memories DROP COLUMN IF EXISTS organization_id;
