CREATE TABLE IF NOT EXISTS pos_transactions (
    id UUID PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    idempotency_key TEXT UNIQUE NOT NULL,
    amount_cents BIGINT NOT NULL,
    currency TEXT NOT NULL DEFAULT 'usd',
    status TEXT NOT NULL DEFAULT 'PENDING',
    payment_method TEXT,
    stripe_payment_intent_id TEXT,
    items JSONB NOT NULL DEFAULT '[]'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_pos_transactions_tenant ON pos_transactions(tenant_id);
CREATE INDEX IF NOT EXISTS idx_pos_transactions_status ON pos_transactions(status);

ALTER TABLE pos_transactions ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_pos_transactions ON pos_transactions;
CREATE POLICY tenant_isolation_pos_transactions ON pos_transactions USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
