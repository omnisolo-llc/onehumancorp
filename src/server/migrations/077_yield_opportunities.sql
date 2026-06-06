CREATE TABLE IF NOT EXISTS yield_opportunities (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    service_id TEXT NOT NULL,
    target_date TEXT NOT NULL,
    empty_slots INTEGER NOT NULL,
    proposed_discount INTEGER NOT NULL,
    status TEXT NOT NULL DEFAULT 'PENDING',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_yield_opportunities_tenant ON yield_opportunities(tenant_id);

ALTER TABLE yield_opportunities ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_yield_opportunities ON yield_opportunities USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
