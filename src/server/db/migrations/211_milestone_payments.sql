-- +goose Up
CREATE TABLE IF NOT EXISTS milestone_payments (
    milestone_id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    quote_id UUID NOT NULL REFERENCES quotes(id) ON DELETE CASCADE,
    percentage DECIMAL(5,2),
    amount BIGINT NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending' CHECK (status IN ('pending', 'paid')),
    due_condition TEXT NOT NULL CHECK (due_condition IN ('on_approval', 'on_completion')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_milestone_payments_tenant_id ON milestone_payments(tenant_id);
CREATE INDEX IF NOT EXISTS idx_milestone_payments_quote_id ON milestone_payments(quote_id);

ALTER TABLE milestone_payments ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_milestone_payments ON milestone_payments;
CREATE POLICY tenant_isolation_milestone_payments
ON milestone_payments
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- +goose Down
DROP POLICY IF EXISTS tenant_isolation_milestone_payments ON milestone_payments;
DROP TABLE IF EXISTS milestone_payments CASCADE;
