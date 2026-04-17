-- +goose Up
-- +goose StatementBegin

CREATE SCHEMA IF NOT EXISTS ohc_tasks;
CREATE SCHEMA IF NOT EXISTS ohc_memory;

CREATE EXTENSION IF NOT EXISTS vector;

CREATE TABLE IF NOT EXISTS ohc_tasks.missions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    epic_id UUID,
    title VARCHAR NOT NULL,
    status VARCHAR NOT NULL DEFAULT 'PENDING',
    assigned_agent_id VARCHAR,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS ohc_tasks.mission_dependencies (
    task_id UUID NOT NULL REFERENCES ohc_tasks.missions(id) ON DELETE CASCADE,
    depends_on_task_id UUID NOT NULL REFERENCES ohc_tasks.missions(id) ON DELETE CASCADE,
    PRIMARY KEY (task_id, depends_on_task_id)
);

CREATE TABLE IF NOT EXISTS ohc_memory.autodream_vectors (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    task_id UUID REFERENCES ohc_tasks.missions(id),
    content TEXT NOT NULL,
    embedding vector(1536),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
DROP TABLE IF EXISTS ohc_memory.autodream_vectors;
DROP TABLE IF EXISTS ohc_tasks.mission_dependencies;
DROP TABLE IF EXISTS ohc_tasks.missions;
DROP SCHEMA IF EXISTS ohc_memory;
DROP SCHEMA IF EXISTS ohc_tasks;
-- +goose StatementEnd
