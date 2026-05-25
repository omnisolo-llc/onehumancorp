-- Migration 013: Unified Multi-Tenant Data Model Architecture for Small Business Operations
-- Enforcing exact schema from design doc

CREATE TABLE IF NOT EXISTS products (
    product_id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    type TEXT NOT NULL, -- Physical, Digital, Service
    price_cents INT NOT NULL DEFAULT 0,
    in_stock BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS conversations (
    conversation_id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    customer_id TEXT REFERENCES customers(id) ON DELETE CASCADE,
    channel TEXT NOT NULL, -- IG, Web, Email
    content TEXT NOT NULL,
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
            'products',
            'conversations'
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
                    'CREATE POLICY %I ON %I USING (tenant_id::text = current_setting(''app.current_tenant'', true))',
                    pol_name,
                    t_name
                );
            END IF;
        END IF;
    END LOOP;
END
$$;
