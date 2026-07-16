CREATE TABLE IF NOT EXISTS applied_client_mutations (
    client_mutation_id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    applied_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE applied_client_mutations ENABLE ROW LEVEL SECURITY;

CREATE POLICY applied_client_mutations_tenant_isolation_policy ON applied_client_mutations
    USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true)::uuid);

CREATE INDEX IF NOT EXISTS idx_applied_client_mutations_tenant ON applied_client_mutations(tenant_id);
