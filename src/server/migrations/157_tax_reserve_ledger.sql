CREATE TABLE IF NOT EXISTS ledger_entries (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    channel TEXT NOT NULL,
    amount DECIMAL NOT NULL,
    tax_amount DECIMAL NOT NULL,
    tax_region TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS tax_reserves (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    total_reserved DECIMAL NOT NULL DEFAULT 0.0,
    last_updated TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    CONSTRAINT unique_tenant_tax_reserve UNIQUE (tenant_id)
);

CREATE TABLE IF NOT EXISTS finance_agent_jobs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending',
    scheduled_for TIMESTAMP WITH TIME ZONE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

ALTER TABLE IF EXISTS ledger_entries ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS tax_reserves ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS finance_agent_jobs ENABLE ROW LEVEL SECURITY;

CREATE POLICY tenant_isolation_ledger_entries ON ledger_entries
    FOR ALL
    USING (tenant_id = current_setting('app.current_tenant_id')::UUID);

CREATE POLICY tenant_isolation_tax_reserves ON tax_reserves
    FOR ALL
    USING (tenant_id = current_setting('app.current_tenant_id')::UUID);

CREATE POLICY tenant_isolation_finance_agent_jobs ON finance_agent_jobs
    FOR ALL
    USING (tenant_id = current_setting('app.current_tenant_id')::UUID);
