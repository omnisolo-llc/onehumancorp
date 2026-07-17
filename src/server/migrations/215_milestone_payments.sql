-- Migration 215: Add milestone_payments table

CREATE TABLE IF NOT EXISTS milestone_payments (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    milestone_id TEXT,
    quote_id TEXT REFERENCES quotes(id) ON DELETE CASCADE,
    percentage DECIMAL(5,2),
    amount BIGINT NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending' CHECK (status IN ('pending', 'paid', 'refunded', 'voided')),
    due_condition TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_milestone_payments_tenant_id ON milestone_payments(tenant_id);

ALTER TABLE milestone_payments ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_milestone_payments ON milestone_payments;
CREATE POLICY tenant_isolation_milestone_payments
ON milestone_payments
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- Update Quote table if not already updated
ALTER TABLE quotes ADD COLUMN IF NOT EXISTS status TEXT DEFAULT 'draft';
