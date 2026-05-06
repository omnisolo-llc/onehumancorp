-- organizations tier tracking
CREATE TABLE IF NOT EXISTS organizations (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    plan_tier TEXT DEFAULT 'free',
    current_period_end TIMESTAMPTZ,
    tenant_id TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Note: RLS policies for organizations are handled centrally or assume app.current_tenant
ALTER TABLE organizations ENABLE ROW LEVEL SECURITY;

CREATE POLICY "organizations_isolation" ON organizations
    FOR ALL
    USING (tenant_id = current_setting('app.current_tenant', true));
