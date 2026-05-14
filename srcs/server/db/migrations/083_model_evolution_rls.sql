-- +goose Up

-- Agents
CREATE TABLE IF NOT EXISTS agents (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    department TEXT NOT NULL,
    status TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE agents ENABLE ROW LEVEL SECURITY;

CREATE POLICY tenant_isolation_agents ON agents
    USING (tenant_id = nullif(current_setting('app.current_tenant', true), '')::text);

-- Memories (general)
CREATE TABLE IF NOT EXISTS memories (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    agent_id TEXT NOT NULL,
    content TEXT NOT NULL,
    embedding vector(1536),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE memories ENABLE ROW LEVEL SECURITY;

CREATE POLICY tenant_isolation_memories ON memories
    USING (tenant_id = nullif(current_setting('app.current_tenant', true), '')::text);

-- Tasks isolation enforcement
ALTER TABLE tasks ADD COLUMN IF NOT EXISTS tenant_id TEXT NOT NULL DEFAULT 'system';
ALTER TABLE tasks ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_tasks_strict ON tasks
    USING (tenant_id = nullif(current_setting('app.current_tenant', true), '')::text);

-- +goose Down
DROP POLICY IF EXISTS tenant_isolation_tasks_strict ON tasks;
ALTER TABLE tasks DISABLE ROW LEVEL SECURITY;
ALTER TABLE tasks DROP COLUMN IF EXISTS tenant_id;

DROP POLICY IF EXISTS tenant_isolation_memories ON memories;
ALTER TABLE memories DISABLE ROW LEVEL SECURITY;
DROP TABLE IF EXISTS memories;

DROP POLICY IF EXISTS tenant_isolation_agents ON agents;
ALTER TABLE agents DISABLE ROW LEVEL SECURITY;
DROP TABLE IF EXISTS agents;
