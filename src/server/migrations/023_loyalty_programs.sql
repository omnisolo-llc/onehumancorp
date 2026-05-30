-- Create loyalty program tables
CREATE TABLE IF NOT EXISTS loyalty_programs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id TEXT NOT NULL,
    store_name TEXT NOT NULL,
    discount_amount TEXT NOT NULL,
    loyalty_tier TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Row level security
ALTER TABLE loyalty_programs ENABLE ROW LEVEL SECURITY;

CREATE POLICY tenant_isolation_loyalty_programs ON loyalty_programs
    USING (tenant_id = current_setting('app.current_tenant', true));
