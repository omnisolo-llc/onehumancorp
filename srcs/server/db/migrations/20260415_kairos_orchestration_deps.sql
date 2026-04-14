-- +goose Up
-- +goose StatementBegin
ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS dependencies JSONB NOT NULL DEFAULT '[]';
ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS parent_plan_id TEXT;
ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS agent_id VARCHAR;

DROP TABLE IF EXISTS consolidated_memory;
CREATE TABLE IF NOT EXISTS consolidated_memory (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    task_id UUID REFERENCES shared_tasks(id),
    content TEXT NOT NULL,
    embedding VECTOR(1536),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
DROP TABLE IF EXISTS consolidated_memory;
ALTER TABLE shared_tasks DROP COLUMN dependencies;
-- +goose StatementEnd
