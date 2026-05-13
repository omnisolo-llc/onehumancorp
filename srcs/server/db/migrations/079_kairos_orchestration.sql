-- +goose Up
-- +goose StatementBegin
-- +goose sqlite3
CREATE TABLE IF NOT EXISTS kairos_shared_tasks (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    status TEXT NOT NULL,
    payload TEXT
);

CREATE TABLE IF NOT EXISTS kairos_state_transitions (
    id TEXT PRIMARY KEY,
    task_id TEXT NOT NULL,
    from_state TEXT,
    to_state TEXT NOT NULL,
    transitioned_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS kairos_sub_agent_jobs (
    id TEXT PRIMARY KEY,
    parent_task_id TEXT NOT NULL,
    agent_id TEXT NOT NULL,
    payload TEXT,
    status TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS autodream_vector_memories (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    embedding TEXT,
    metadata TEXT
);
-- +goose StatementEnd

-- +goose StatementBegin
-- +goose postgres
CREATE TABLE IF NOT EXISTS kairos_shared_tasks (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    status TEXT NOT NULL,
    payload JSONB
);
ALTER TABLE kairos_shared_tasks ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_kairos_shared_tasks ON kairos_shared_tasks USING (tenant_id = current_setting('app.current_tenant', true)::uuid);

CREATE TABLE IF NOT EXISTS kairos_state_transitions (
    id UUID PRIMARY KEY,
    task_id UUID NOT NULL,
    from_state TEXT,
    to_state TEXT NOT NULL,
    transitioned_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS kairos_sub_agent_jobs (
    id UUID PRIMARY KEY,
    parent_task_id UUID NOT NULL,
    agent_id TEXT NOT NULL,
    payload JSONB,
    status TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS autodream_vector_memories (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    embedding vector(1536),
    metadata JSONB
);
ALTER TABLE autodream_vector_memories ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_autodream_vector_memories ON autodream_vector_memories USING (tenant_id = current_setting('app.current_tenant', true)::uuid);
-- +goose StatementEnd

-- +goose Down
DROP TABLE IF EXISTS autodream_vector_memories;
DROP TABLE IF EXISTS kairos_sub_agent_jobs;
DROP TABLE IF EXISTS kairos_state_transitions;
DROP TABLE IF EXISTS kairos_shared_tasks;
