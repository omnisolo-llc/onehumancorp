CREATE TABLE IF NOT EXISTS agent_missions (
    id TEXT PRIMARY KEY,
    status TEXT NOT NULL,
    payload TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    organization_id TEXT NOT NULL DEFAULT 'system',
    cloud_mission_id TEXT,
    sync_error TEXT,
    last_synced_at TIMESTAMPTZ,
    synced_to_cloud BOOLEAN DEFAULT FALSE,
    _sync_status TEXT DEFAULT 'pending',
    version INTEGER DEFAULT 1
);

ALTER TABLE agent_missions ENABLE ROW LEVEL SECURITY;
CREATE POLICY agent_missions_isolation_policy ON agent_missions
USING (organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');
