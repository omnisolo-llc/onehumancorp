-- +goose Up
-- +goose StatementBegin
-- +goose sqlite3
CREATE TABLE IF NOT EXISTS swarm_long_term_memory (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    content TEXT,
    embedding TEXT,
    metadata TEXT
);
-- +goose StatementEnd

-- +goose StatementBegin
-- +goose postgres
CREATE TABLE IF NOT EXISTS swarm_long_term_memory (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    content TEXT,
    embedding vector(1536),
    metadata JSONB
);

ALTER TABLE swarm_long_term_memory ENABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_swarm_long_term_memory ON swarm_long_term_memory;
CREATE POLICY tenant_isolation_swarm_long_term_memory ON swarm_long_term_memory
    FOR ALL
    USING (
        tenant_id = current_setting('app.current_tenant', true)::uuid
        OR current_user = 'system'
    );
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
-- +goose sqlite3
DROP TABLE IF EXISTS swarm_long_term_memory;
-- +goose StatementEnd

-- +goose StatementBegin
-- +goose postgres
DROP POLICY IF EXISTS tenant_isolation_swarm_long_term_memory ON swarm_long_term_memory;
ALTER TABLE swarm_long_term_memory DISABLE ROW LEVEL SECURITY;
DROP TABLE IF EXISTS swarm_long_term_memory;
-- +goose StatementEnd
