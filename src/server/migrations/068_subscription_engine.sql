-- Migration 068: Subscription Box & Membership Engine

CREATE TABLE IF NOT EXISTS subscription_plans (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    name TEXT NOT NULL,
    price_cents INTEGER NOT NULL,
    frequency TEXT NOT NULL, -- 'monthly', 'weekly', 'yearly'
    cutoff_date INTEGER, -- day of month, e.g., 5
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    _sync_status TEXT DEFAULT 'pending',
    version INTEGER DEFAULT 1
);

CREATE TABLE IF NOT EXISTS subscribers (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    plan_id TEXT NOT NULL,
    customer_id TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'ACTIVE', -- 'ACTIVE', 'PAST_DUE', 'CANCELED'
    next_billing_date TIMESTAMP,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    _sync_status TEXT DEFAULT 'pending',
    version INTEGER DEFAULT 1
);

CREATE TABLE IF NOT EXISTS fulfillment_batches (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    plan_id TEXT NOT NULL,
    target_date TIMESTAMP NOT NULL,
    status TEXT NOT NULL DEFAULT 'PENDING', -- 'PENDING', 'PRINTED', 'SHIPPED'
    label_url TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    _sync_status TEXT DEFAULT 'pending',
    version INTEGER DEFAULT 1
);

CREATE INDEX IF NOT EXISTS idx_subscription_plans_tenant ON subscription_plans(tenant_id);
CREATE INDEX IF NOT EXISTS idx_subscribers_tenant ON subscribers(tenant_id);
CREATE INDEX IF NOT EXISTS idx_subscribers_plan ON subscribers(plan_id);
CREATE INDEX IF NOT EXISTS idx_fulfillment_batches_tenant ON fulfillment_batches(tenant_id);

-- Enforce Strict Multi-Tenancy via PostgreSQL RLS
DO $$
DECLARE
    t_name text;
    pol_name text;
BEGIN
    FOR t_name IN
        SELECT unnest(ARRAY[
            'subscription_plans',
            'subscribers',
            'fulfillment_batches'
        ])
    LOOP
        IF to_regclass(t_name) IS NOT NULL THEN
            EXECUTE format('ALTER TABLE %I ENABLE ROW LEVEL SECURITY', t_name);

            pol_name := format('tenant_isolation_%s', t_name);
            IF NOT EXISTS (
                SELECT 1
                FROM pg_policies
                WHERE schemaname = current_schema()
                    AND tablename = t_name
                    AND policyname = pol_name
            ) THEN
                EXECUTE format(
                    'CREATE POLICY %I ON %I USING (tenant_id::text = current_setting(''app.current_tenant'', true)) WITH CHECK (tenant_id::text = current_setting(''app.current_tenant'', true))',
                    pol_name,
                    t_name
                );
            END IF;
        END IF;
    END LOOP;
END
$$;
