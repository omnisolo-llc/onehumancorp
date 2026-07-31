-- Agentic Invoicing System Update

CREATE TABLE IF NOT EXISTS payment_schedules (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    invoice_id TEXT NOT NULL REFERENCES invoices(id) ON DELETE CASCADE,
    amount DOUBLE PRECISION NOT NULL,
    type TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_payment_schedules_invoice_id ON payment_schedules(invoice_id);
CREATE INDEX IF NOT EXISTS idx_payment_schedules_tenant_id ON payment_schedules(tenant_id);

ALTER TABLE payment_schedules ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_payment_schedules ON payment_schedules;
CREATE POLICY tenant_isolation_payment_schedules
ON payment_schedules
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));


CREATE TABLE IF NOT EXISTS receivables_actions (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    invoice_id TEXT NOT NULL REFERENCES invoices(id) ON DELETE CASCADE,
    action TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_receivables_actions_invoice_id ON receivables_actions(invoice_id);
CREATE INDEX IF NOT EXISTS idx_receivables_actions_tenant_id ON receivables_actions(tenant_id);

ALTER TABLE receivables_actions ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_receivables_actions ON receivables_actions;
CREATE POLICY tenant_isolation_receivables_actions
ON receivables_actions
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
