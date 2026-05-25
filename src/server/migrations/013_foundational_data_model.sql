-- Migration 013: Foundational Data Model Architecture

CREATE TABLE IF NOT EXISTS tenant (
    id UUID PRIMARY KEY,
    business_name TEXT NOT NULL,
    owner_email TEXT NOT NULL,
    subscription_tier TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    _sync_status TEXT DEFAULT 'synced',
    version BIGINT DEFAULT 1
);

CREATE TABLE IF NOT EXISTS product (
    id UUID NOT NULL,
    tenant_id UUID NOT NULL REFERENCES tenant(id) ON DELETE CASCADE,
    type TEXT NOT NULL,
    title TEXT NOT NULL,
    price DECIMAL NOT NULL,
    stock_level INT NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    _sync_status TEXT DEFAULT 'synced',
    version BIGINT DEFAULT 1,
    PRIMARY KEY (tenant_id, id)
);

CREATE TABLE IF NOT EXISTS customer (
    id UUID NOT NULL,
    tenant_id UUID NOT NULL REFERENCES tenant(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    email TEXT,
    phone TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    _sync_status TEXT DEFAULT 'synced',
    version BIGINT DEFAULT 1,
    PRIMARY KEY (tenant_id, id)
);

CREATE TABLE IF NOT EXISTS order_booking (
    id UUID NOT NULL,
    tenant_id UUID NOT NULL REFERENCES tenant(id) ON DELETE CASCADE,
    customer_id UUID NOT NULL,
    status TEXT NOT NULL,
    total_amount DECIMAL NOT NULL,
    scheduled_for TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    _sync_status TEXT DEFAULT 'synced',
    version BIGINT DEFAULT 1,
    PRIMARY KEY (tenant_id, id),
    FOREIGN KEY (tenant_id, customer_id) REFERENCES customer(tenant_id, id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS order_item (
    id UUID NOT NULL,
    tenant_id UUID NOT NULL REFERENCES tenant(id) ON DELETE CASCADE,
    order_id UUID NOT NULL,
    product_id UUID NOT NULL,
    quantity INT NOT NULL,
    unit_price DECIMAL NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    _sync_status TEXT DEFAULT 'synced',
    version BIGINT DEFAULT 1,
    PRIMARY KEY (tenant_id, id),
    FOREIGN KEY (tenant_id, order_id) REFERENCES order_booking(tenant_id, id) ON DELETE CASCADE,
    FOREIGN KEY (tenant_id, product_id) REFERENCES product(tenant_id, id) ON DELETE CASCADE
);

-- Enforce Strict Multi-Tenancy via PostgreSQL RLS
DO $$
DECLARE
    t_name text;
    pol_name text;
BEGIN
    FOR t_name IN
        SELECT unnest(ARRAY[
            'tenant',
            'product',
            'customer',
            'order_booking',
            'order_item'
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
                IF t_name = 'tenant' THEN
                    EXECUTE format(
                        'CREATE POLICY %I ON %I USING (id::text = current_setting(''app.current_tenant'', true))',
                        pol_name,
                        t_name
                    );
                ELSE
                    EXECUTE format(
                        'CREATE POLICY %I ON %I USING (tenant_id::text = current_setting(''app.current_tenant'', true))',
                        pol_name,
                        t_name
                    );
                END IF;
            END IF;
        END IF;
    END LOOP;
END
$$;
