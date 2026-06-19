-- New payment intents tracking table for idempotency and status
CREATE TABLE IF NOT EXISTS payment_intents (
    tenant_id TEXT NOT NULL,
    payment_id TEXT NOT NULL,
    idempotency_key TEXT NOT NULL UNIQUE,
    amount DOUBLE PRECISION NOT NULL,
    currency TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending',
    source TEXT NOT NULL, -- e.g. 'tap_to_pay', 'payment_link'
    stripe_payment_intent_id TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (tenant_id, payment_id)
);

CREATE INDEX IF NOT EXISTS idx_payment_intents_tenant ON payment_intents(tenant_id);
CREATE INDEX IF NOT EXISTS idx_payment_intents_stripe_id ON payment_intents(stripe_payment_intent_id);
CREATE INDEX IF NOT EXISTS idx_payment_intents_idempotency ON payment_intents(idempotency_key);

ALTER TABLE payment_intents ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_payment_intents ON payment_intents;
CREATE POLICY tenant_isolation_payment_intents ON payment_intents
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Extend ledger accounts if we want specific account tracking
-- We will use ledger_accounts, ledger_transactions, ledger_entries which already exist
