CREATE TABLE IF NOT EXISTS applied_client_mutations (
    client_mutation_id VARCHAR PRIMARY KEY,
    tenant_id VARCHAR NOT NULL,
    applied_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_applied_client_mutations_tenant ON applied_client_mutations(tenant_id);

ALTER TABLE applied_client_mutations ENABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_applied_client_mutations ON applied_client_mutations;
CREATE POLICY tenant_isolation_applied_client_mutations ON applied_client_mutations
    FOR ALL
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
