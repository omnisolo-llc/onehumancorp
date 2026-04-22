-- +goose Up
-- +goose StatementBegin
CREATE TABLE IF NOT EXISTS shared_tasks_v5 (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    status VARCHAR(50) NOT NULL DEFAULT 'PENDING' CHECK (status IN ('PENDING', 'CLAIMED', 'DONE', 'FAILED', 'COMPLETED')),
    agent_id VARCHAR(100),
    priority VARCHAR(50) NOT NULL DEFAULT 'P2',
    payload JSONB
);
CREATE TABLE IF NOT EXISTS consolidated_memory (
    id UUID PRIMARY KEY,
    content TEXT NOT NULL,
    embedding vector(1536),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
DROP TABLE IF EXISTS shared_tasks_v5;
DROP TABLE IF EXISTS consolidated_memory;
-- +goose StatementEnd
