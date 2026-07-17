-- Migration: Milestone Payments (Agentic Quoting Workflow)

CREATE TABLE IF NOT EXISTS milestone_payments (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    quote_id TEXT NOT NULL REFERENCES quotes(id) ON DELETE CASCADE,
    percentage INT NOT NULL DEFAULT 100,
    amount_cents BIGINT NOT NULL DEFAULT 0,
    status TEXT NOT NULL DEFAULT 'pending',
    due_condition TEXT NOT NULL DEFAULT 'on_approval',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_milestone_payments_tenant ON milestone_payments(tenant_id);
CREATE INDEX IF NOT EXISTS idx_milestone_payments_quote ON milestone_payments(quote_id);

ALTER TABLE milestone_payments ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_milestone_payments ON milestone_payments USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
