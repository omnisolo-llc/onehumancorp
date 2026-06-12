-- +goose Up
-- Enable missing Row Level Security for tables that relied purely on dynamic blocks

ALTER TABLE IF EXISTS services ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS vendors ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS purchase_orders ENABLE ROW LEVEL SECURITY;

DO $$
DECLARE
    t_name text;
    pol_name text;
BEGIN
    FOR t_name IN
        SELECT unnest(ARRAY[
            'services',
            'vendors',
            'purchase_orders'
        ])
    LOOP
        IF to_regclass(t_name) IS NOT NULL THEN
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
DO $$
DECLARE
    t_name text;
    pol_name text;
BEGIN
    FOR t_name IN
        SELECT unnest(ARRAY[
            'services',
            'vendors',
            'purchase_orders'
        ])
    LOOP
        IF to_regclass(t_name) IS NOT NULL THEN
            pol_name := format('tenant_isolation_%s', t_name);
            EXECUTE format('DROP POLICY IF EXISTS %I ON %I', pol_name, t_name);
        END IF;
    END LOOP;
END
$$;

ALTER TABLE IF EXISTS services DISABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS vendors DISABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS purchase_orders DISABLE ROW LEVEL SECURITY;
