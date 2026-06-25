-- +goose Up
CREATE TABLE IF NOT EXISTS client_portals (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id TEXT NOT NULL,
    client_id UUID NOT NULL REFERENCES customers(id),
    name VARCHAR(255) NOT NULL,
    branding_config JSONB DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE client_portals ENABLE ROW LEVEL SECURITY;
CREATE POLICY client_portals_tenant_isolation ON client_portals FOR ALL USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

CREATE TABLE IF NOT EXISTS client_portal_sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id TEXT NOT NULL,
    client_portal_id UUID NOT NULL REFERENCES client_portals(id),
    token_hash VARCHAR(255) NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_client_portal_sessions_token ON client_portal_sessions(token_hash);

ALTER TABLE client_portal_sessions ENABLE ROW LEVEL SECURITY;
CREATE POLICY client_portal_sessions_tenant_isolation ON client_portal_sessions FOR ALL USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

CREATE TABLE IF NOT EXISTS shared_documents (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id TEXT NOT NULL,
    client_portal_id UUID NOT NULL REFERENCES client_portals(id),
    title VARCHAR(255) NOT NULL,
    document_url TEXT NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'pending_approval', -- pending_approval, approved, rejected
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE shared_documents ENABLE ROW LEVEL SECURITY;
CREATE POLICY shared_documents_tenant_isolation ON shared_documents FOR ALL USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

CREATE TABLE IF NOT EXISTS approval_threads (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id TEXT NOT NULL,
    client_portal_id UUID NOT NULL REFERENCES client_portals(id),
    topic VARCHAR(255) NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'open',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE shared_documents ADD COLUMN approval_thread_id UUID REFERENCES approval_threads(id);

ALTER TABLE approval_threads ENABLE ROW LEVEL SECURITY;
CREATE POLICY approval_threads_tenant_isolation ON approval_threads FOR ALL USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- +goose Down
DROP POLICY IF EXISTS approval_threads_tenant_isolation ON approval_threads;
ALTER TABLE shared_documents DROP COLUMN IF EXISTS approval_thread_id;
DROP TABLE IF EXISTS approval_threads CASCADE;

DROP POLICY IF EXISTS shared_documents_tenant_isolation ON shared_documents;
DROP TABLE IF EXISTS shared_documents CASCADE;

DROP POLICY IF EXISTS client_portal_sessions_tenant_isolation ON client_portal_sessions;
DROP INDEX IF EXISTS idx_client_portal_sessions_token;
DROP TABLE IF EXISTS client_portal_sessions CASCADE;

DROP POLICY IF EXISTS client_portals_tenant_isolation ON client_portals;
DROP TABLE IF EXISTS client_portals CASCADE;
