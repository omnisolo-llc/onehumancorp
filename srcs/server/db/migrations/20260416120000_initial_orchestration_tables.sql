-- +goose Up
-- +goose StatementBegin

CREATE TABLE IF NOT EXISTS shared_tasks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    parent_id UUID REFERENCES shared_tasks(id),
    epic_id VARCHAR(255),
    title VARCHAR(255) NOT NULL,
    status VARCHAR(50) DEFAULT 'PENDING',
    assigned_agent VARCHAR(255),
    payload JSONB,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS agent_mesh_messages (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    sender VARCHAR(255) NOT NULL,
    recipient VARCHAR(255),
    channel VARCHAR(100) NOT NULL,
    content JSONB NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
DROP TABLE IF EXISTS agent_mesh_messages;
DROP TABLE IF EXISTS shared_tasks;
-- +goose StatementEnd
