-- +goose Up
-- Migration 068: Predictive Supply Chain

CREATE TABLE IF NOT EXISTS inventory_predictions (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    raw_material_id TEXT REFERENCES raw_materials(id) ON DELETE CASCADE,
    predicted_stockout_date TIMESTAMPTZ,
    predicted_daily_velocity DECIMAL DEFAULT 0,
    current_inventory INT DEFAULT 0,
    status TEXT DEFAULT 'PENDING', -- "PENDING | DRAFTED_PO | DISMISSED"
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

DO $$
DECLARE
    t_name text;
    pol_name text;
BEGIN
    FOR t_name IN
        SELECT unnest(ARRAY[
            'inventory_predictions'
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
DO $$
DECLARE
    t_name text;
    pol_name text;
BEGIN
    FOR t_name IN
        SELECT unnest(ARRAY[
            'inventory_predictions'
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

DROP TABLE IF EXISTS inventory_predictions CASCADE;
