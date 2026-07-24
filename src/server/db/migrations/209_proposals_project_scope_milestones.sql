-- +goose Up
ALTER TABLE proposals ADD COLUMN IF NOT EXISTS project_scope TEXT;
ALTER TABLE proposals ADD COLUMN IF NOT EXISTS milestones JSONB;

-- +goose Down
ALTER TABLE proposals DROP COLUMN IF EXISTS project_scope;
ALTER TABLE proposals DROP COLUMN IF EXISTS milestones;
