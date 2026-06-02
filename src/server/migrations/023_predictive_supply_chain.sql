-- +goose Up
-- Migration 023: Predictive Supply Chain

CREATE TABLE IF NOT EXISTS inventory_predictions (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    product_id TEXT REFERENCES products(id) ON DELETE CASCADE,
    predicted_depletion_date TIMESTAMPTZ,
    confidence_score DECIMAL DEFAULT 0.0,
    daily_velocity DECIMAL DEFAULT 0.0,
    status TEXT DEFAULT 'PENDING',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

DO $$
DECLARE
    pol_name text;
BEGIN
    IF to_regclass('inventory_predictions') IS NOT NULL THEN
        EXECUTE 'ALTER TABLE inventory_predictions ENABLE ROW LEVEL SECURITY';
        pol_name := 'tenant_isolation_inventory_predictions';
        IF NOT EXISTS (
            SELECT 1
            FROM pg_policies
            WHERE schemaname = current_schema()
                AND tablename = 'inventory_predictions'
                AND policyname = pol_name
        ) THEN
            EXECUTE format(
                'CREATE POLICY %I ON inventory_predictions USING (tenant_id::text = current_setting(''app.current_tenant'', true)) WITH CHECK (tenant_id::text = current_setting(''app.current_tenant'', true))',
                pol_name
            );
        END IF;
    END IF;
END
$$;

-- +goose Down
DO $$
DECLARE
    pol_name text;
BEGIN
    IF to_regclass('inventory_predictions') IS NOT NULL THEN
        pol_name := 'tenant_isolation_inventory_predictions';
        EXECUTE format('DROP POLICY IF EXISTS %I ON inventory_predictions', pol_name);
        EXECUTE 'ALTER TABLE inventory_predictions DISABLE ROW LEVEL SECURITY';
    END IF;
END
$$;

DROP TABLE IF EXISTS inventory_predictions CASCADE;
