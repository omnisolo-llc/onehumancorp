-- +goose Up
CREATE TABLE IF NOT EXISTS client_portal_sessions (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    customer_id TEXT NOT NULL,
    magic_token TEXT NOT NULL UNIQUE,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_client_portal_sessions_token ON client_portal_sessions(magic_token);
CREATE INDEX IF NOT EXISTS idx_client_portal_sessions_tenant ON client_portal_sessions(tenant_id);

ALTER TABLE client_portal_sessions ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_client_portal_sessions ON client_portal_sessions
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));


CREATE TABLE IF NOT EXISTS client_approval_requests (
    id TEXT PRIMARY KEY,
    session_id TEXT NOT NULL REFERENCES client_portal_sessions(id) ON DELETE CASCADE,
    status TEXT NOT NULL DEFAULT 'Pending', -- 'Pending', 'Approved', 'Rejected'
    type TEXT NOT NULL, -- 'Quote', 'Design', 'ChangeOrder'
    description TEXT NOT NULL,
    reference_id TEXT, -- e.g., quote_id, project_id
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_client_approval_requests_session ON client_approval_requests(session_id);

ALTER TABLE client_approval_requests ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_client_approval_requests ON client_approval_requests
    USING (
        session_id IN (SELECT id FROM client_portal_sessions WHERE tenant_id::text = current_setting('app.current_tenant', true))
    )
    WITH CHECK (
        session_id IN (SELECT id FROM client_portal_sessions WHERE tenant_id::text = current_setting('app.current_tenant', true))
    );

-- +goose Down
DROP POLICY IF EXISTS tenant_isolation_client_approval_requests ON client_approval_requests;
DROP TABLE IF EXISTS client_approval_requests CASCADE;

DROP POLICY IF EXISTS tenant_isolation_client_portal_sessions ON client_portal_sessions;
DROP TABLE IF EXISTS client_portal_sessions CASCADE;
