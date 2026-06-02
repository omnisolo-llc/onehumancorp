-- +goose Up
-- Migration 058: Autonomous AI Omnichannel Invoicing Engine

CREATE TABLE IF NOT EXISTS invoices (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    customer_id TEXT REFERENCES customers(id) ON DELETE SET NULL,
    status TEXT DEFAULT 'draft', -- draft, pending, paid, canceled
    total_amount DECIMAL DEFAULT 0,
    payment_link TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS invoice_items (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    invoice_id TEXT REFERENCES invoices(id) ON DELETE CASCADE,
    description TEXT NOT NULL,
    quantity INT DEFAULT 1,
    unit_price DECIMAL DEFAULT 0,
    amount DECIMAL DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS payment_intents (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    invoice_id TEXT REFERENCES invoices(id) ON DELETE CASCADE,
    provider_id TEXT NOT NULL,
    status TEXT DEFAULT 'pending',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Enforce Strict Multi-Tenancy via PostgreSQL RLS
DO $$
DECLARE
    t_name text;
    pol_name text;
BEGIN
    FOR t_name IN
        SELECT unnest(ARRAY[
            'invoices',
            'invoice_items',
            'payment_intents'
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

-- +goose Down
-- Reverse Migration 058

DO $$
DECLARE
    t_name text;
    pol_name text;
BEGIN
    FOR t_name IN
        SELECT unnest(ARRAY[
            'payment_intents',
            'invoice_items',
            'invoices'
        ])
    LOOP
        IF to_regclass(t_name) IS NOT NULL THEN
            pol_name := format('tenant_isolation_%s', t_name);
            EXECUTE format('DROP POLICY IF EXISTS %I ON %I', pol_name, t_name);
            EXECUTE format('ALTER TABLE %I DISABLE ROW LEVEL SECURITY', t_name);
        END IF;
    END LOOP;
END
$$;

DROP TABLE IF EXISTS payment_intents CASCADE;
DROP TABLE IF EXISTS invoice_items CASCADE;
DROP TABLE IF EXISTS invoices CASCADE;
