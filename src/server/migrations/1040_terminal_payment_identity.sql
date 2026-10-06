-- Provider identity and conservative replay state for amount-only terminal payments.
-- Existing intents are deliberately NOT backfilled with inferred authorization.
CREATE TABLE IF NOT EXISTS terminal_payment_operations (
    tenant_id TEXT NOT NULL,
    operation_id TEXT NOT NULL,
    amount_cents BIGINT NOT NULL CHECK (amount_cents > 0 AND amount_cents <= 99999999),
    currency TEXT NOT NULL,
    provider_fingerprint TEXT NOT NULL,
    stripe_payment_intent_id TEXT,
    state TEXT NOT NULL CHECK (state IN ('creating','ready','capturing','succeeded','reconciliation_required')),
    provider_receipt JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (tenant_id, operation_id),
    UNIQUE (tenant_id, stripe_payment_intent_id),
    UNIQUE (provider_fingerprint, stripe_payment_intent_id),
    CHECK (state NOT IN ('ready','capturing','succeeded') OR stripe_payment_intent_id IS NOT NULL),
    CHECK (state <> 'succeeded' OR provider_receipt IS NOT NULL)
);
ALTER TABLE terminal_payment_operations ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_terminal_payment_operations ON terminal_payment_operations
    USING (tenant_id = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
