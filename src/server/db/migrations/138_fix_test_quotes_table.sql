-- +goose Up
-- Migration 138: Ensure quotes table exists for test environments
CREATE TABLE IF NOT EXISTS quotes (
    id UUID PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    customer_id UUID NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('DRAFT', 'PENDING_APPROVAL', 'SENT', 'ACCEPTED', 'REJECTED', 'EXPIRED')),
    valid_until TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    total_amount_cents BIGINT,
    required_deposit_cents BIGINT,
    stripe_payment_link TEXT,
    last_follow_up_at TIMESTAMPTZ,
    follow_up_count INTEGER DEFAULT 0
);

DO $$
BEGIN
    IF to_regclass('quotes') IS NOT NULL THEN
        ALTER TABLE quotes ENABLE ROW LEVEL SECURITY;
        IF NOT EXISTS (
            SELECT 1
            FROM pg_policies
            WHERE schemaname = current_schema()
                AND tablename = 'quotes'
                AND policyname = 'tenant_isolation_quotes'
        ) THEN
            CREATE POLICY tenant_isolation_quotes ON quotes USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
        END IF;
    END IF;
END
$$;

-- +goose Down
DROP TABLE IF EXISTS quotes CASCADE;
