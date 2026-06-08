CREATE TABLE IF NOT EXISTS custom_quotes (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    customer_id TEXT,
    status TEXT NOT NULL DEFAULT 'DRAFT',
    total_amount DOUBLE PRECISION NOT NULL,
    proposed_completion_date TIMESTAMPTZ,
    line_items JSONB NOT NULL DEFAULT '[]'::jsonb,
    original_request TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_custom_quotes_tenant ON custom_quotes(tenant_id);
CREATE INDEX IF NOT EXISTS idx_custom_quotes_status ON custom_quotes(status);

ALTER TABLE custom_quotes ENABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_custom_quotes ON custom_quotes;
CREATE POLICY tenant_isolation_custom_quotes ON custom_quotes
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
