-- +goose Up
-- +goose StatementBegin
CREATE SCHEMA IF NOT EXISTS ohc_tasks;
CREATE SCHEMA IF NOT EXISTS ohc_memory;

CREATE TABLE IF NOT EXISTS ohc_tasks.mission_queue (
    mission_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title VARCHAR(255) NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'QUEUED', -- QUEUED, IN_PROGRESS, BLOCKED, DONE
    assigned_agent VARCHAR(100),
    priority VARCHAR(10) NOT NULL,
    payload JSONB NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE EXTENSION IF NOT EXISTS vector;

CREATE TABLE IF NOT EXISTS ohc_memory.autodream_vectors (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    task_id UUID REFERENCES ohc_tasks.mission_queue(mission_id),
    content TEXT NOT NULL,
    embedding vector(1536),
    metadata JSONB,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS ohc_tasks.sub_agent_queue (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    parent_task_id UUID NOT NULL,
    payload JSONB,
    status TEXT NOT NULL DEFAULT 'QUEUED',
    worker_id TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
DROP TABLE IF EXISTS ohc_tasks.sub_agent_queue;
DROP TABLE IF EXISTS ohc_memory.autodream_vectors;
DROP TABLE IF EXISTS ohc_tasks.mission_queue;
DROP SCHEMA IF EXISTS ohc_memory;
DROP SCHEMA IF EXISTS ohc_tasks;
-- +goose StatementEnd
