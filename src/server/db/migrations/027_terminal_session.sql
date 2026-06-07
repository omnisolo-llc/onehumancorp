CREATE TABLE IF NOT EXISTS terminal_sessions (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    device_id TEXT NOT NULL,
    staff_id TEXT,
    status TEXT NOT NULL DEFAULT 'ACTIVE',
    started_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    ended_at TIMESTAMPTZ,
    last_sync_at TIMESTAMPTZ
);
CREATE INDEX IF NOT EXISTS idx_terminal_sessions_tenant ON terminal_sessions(tenant_id, status);
ALTER TABLE terminal_sessions ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_terminal_sessions ON terminal_sessions;
CREATE POLICY tenant_isolation_terminal_sessions
ON terminal_sessions
USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
