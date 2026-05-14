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
DROP POLICY IF EXISTS tenant_isolation_agent_missions ON agent_missions;

-- +goose Down
DROP TABLE IF EXISTS agent_missions;
