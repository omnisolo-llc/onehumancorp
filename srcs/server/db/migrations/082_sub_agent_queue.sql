-- +goose Up
-- +goose StatementBegin
-- +goose sqlite3
CREATE TABLE IF NOT EXISTS sub_agent_queue (
    id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL,
    parent_task_id TEXT NOT NULL,
    payload TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'PENDING',
    worker_id TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_sub_agent_queue_status ON sub_agent_queue(status);
CREATE INDEX IF NOT EXISTS idx_sub_agent_queue_organization_id ON sub_agent_queue(organization_id);
-- +goose StatementEnd

-- +goose StatementBegin
-- +goose postgres
CREATE TABLE IF NOT EXISTS sub_agent_queue (
    id UUID PRIMARY KEY,
    organization_id UUID NOT NULL,
    parent_task_id UUID NOT NULL,
    payload JSONB NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'PENDING',
    worker_id VARCHAR(255),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_sub_agent_queue_status ON sub_agent_queue(status);
CREATE INDEX IF NOT EXISTS idx_sub_agent_queue_organization_id ON sub_agent_queue(organization_id);
-- +goose StatementEnd

-- +goose Down
DROP TABLE IF EXISTS sub_agent_queue;
