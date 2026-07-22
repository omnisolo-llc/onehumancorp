CREATE TABLE IF NOT EXISTS payment_disputes (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    transaction_id TEXT NOT NULL,
    dispute_reason TEXT NOT NULL,
    status TEXT NOT NULL,
    amount_cents BIGINT NOT NULL,
    currency TEXT NOT NULL,
    evidence_due_by TIMESTAMPTZ,
    compiled_evidence JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE IF EXISTS payment_disputes ENABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_payment_disputes ON payment_disputes;
CREATE POLICY tenant_isolation_payment_disputes ON payment_disputes USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

CREATE INDEX IF NOT EXISTS idx_payment_disputes_tenant_id ON payment_disputes(tenant_id);
