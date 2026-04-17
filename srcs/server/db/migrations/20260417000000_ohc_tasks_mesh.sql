-- +goose Up
-- +goose StatementBegin
CREATE SCHEMA IF NOT EXISTS ohc_tasks;
CREATE SCHEMA IF NOT EXISTS ohc_memory;

CREATE TABLE IF NOT EXISTS ohc_tasks.missions (
    id VARCHAR PRIMARY KEY,
    epic_id VARCHAR,
    title VARCHAR,
    status VARCHAR,
    assigned_agent_id VARCHAR
);

CREATE TABLE IF NOT EXISTS ohc_tasks.mission_dependencies (
    mission_id VARCHAR REFERENCES ohc_tasks.missions(id),
    depends_on_id VARCHAR
);

CREATE TABLE IF NOT EXISTS ohc_memory.autodream_vectors (
    id UUID PRIMARY KEY,
    task_id VARCHAR,
    content TEXT,
    embedding vector(1536),
    metadata JSONB,
    created_at TIMESTAMPTZ
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
