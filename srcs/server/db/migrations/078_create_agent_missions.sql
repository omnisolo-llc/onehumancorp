-- +goose Up
CREATE TABLE IF NOT EXISTS agent_missions (
    id TEXT PRIMARY KEY,
    status TEXT NOT NULL,
    payload TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    organization_id TEXT,
    mission_log TEXT
);
ALTER TABLE agent_missions ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_agent_missions ON agent_missions USING (organization_id = current_setting('app.current_tenant', true));

-- +goose Down
DROP TABLE IF EXISTS agent_missions;
