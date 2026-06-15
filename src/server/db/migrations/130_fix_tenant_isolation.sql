-- +goose Up
-- Migration 130: Add missing RLS and Tenant ID Columns

DO $$
BEGIN
    -- Add tenant_id if missing. Default to 'default_tenant' to avoid data loss.
    IF to_regclass('delivery_tasks') IS NOT NULL AND NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name='delivery_tasks' AND column_name='tenant_id') THEN
        ALTER TABLE delivery_tasks ADD COLUMN tenant_id TEXT DEFAULT 'default_tenant';
        UPDATE delivery_tasks SET tenant_id = 'default_tenant' WHERE tenant_id IS NULL;
        ALTER TABLE delivery_tasks ALTER COLUMN tenant_id SET NOT NULL;
        CREATE INDEX IF NOT EXISTS idx_delivery_tasks_tenant_id ON delivery_tasks(tenant_id);
    END IF;

    IF to_regclass('delivery_zones') IS NOT NULL AND NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name='delivery_zones' AND column_name='tenant_id') THEN
        ALTER TABLE delivery_zones ADD COLUMN tenant_id TEXT DEFAULT 'default_tenant';
        UPDATE delivery_zones SET tenant_id = 'default_tenant' WHERE tenant_id IS NULL;
        ALTER TABLE delivery_zones ALTER COLUMN tenant_id SET NOT NULL;
        CREATE INDEX IF NOT EXISTS idx_delivery_zones_tenant_id ON delivery_zones(tenant_id);
    END IF;

    IF to_regclass('ohc_shared_offer') IS NOT NULL AND NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name='ohc_shared_offer' AND column_name='tenant_id') THEN
        ALTER TABLE ohc_shared_offer ADD COLUMN tenant_id TEXT DEFAULT 'default_tenant';
        UPDATE ohc_shared_offer SET tenant_id = 'default_tenant' WHERE tenant_id IS NULL;
        ALTER TABLE ohc_shared_offer ALTER COLUMN tenant_id SET NOT NULL;
        CREATE INDEX IF NOT EXISTS idx_ohc_shared_offer_tenant_id ON ohc_shared_offer(tenant_id);
    END IF;

    IF to_regclass('quote_line_items') IS NOT NULL AND NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name='quote_line_items' AND column_name='tenant_id') THEN
        ALTER TABLE quote_line_items ADD COLUMN tenant_id TEXT DEFAULT 'default_tenant';
        UPDATE quote_line_items SET tenant_id = 'default_tenant' WHERE tenant_id IS NULL;
        ALTER TABLE quote_line_items ALTER COLUMN tenant_id SET NOT NULL;
        CREATE INDEX IF NOT EXISTS idx_quote_line_items_tenant_id ON quote_line_items(tenant_id);
    END IF;

    IF to_regclass('route_plans') IS NOT NULL AND NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name='route_plans' AND column_name='tenant_id') THEN
        ALTER TABLE route_plans ADD COLUMN tenant_id TEXT DEFAULT 'default_tenant';
        UPDATE route_plans SET tenant_id = 'default_tenant' WHERE tenant_id IS NULL;
        ALTER TABLE route_plans ALTER COLUMN tenant_id SET NOT NULL;
        CREATE INDEX IF NOT EXISTS idx_route_plans_tenant_id ON route_plans(tenant_id);
    END IF;

    IF to_regclass('shared_task_dependencies') IS NOT NULL AND NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name='shared_task_dependencies' AND column_name='tenant_id') THEN
        ALTER TABLE shared_task_dependencies ADD COLUMN tenant_id TEXT DEFAULT 'default_tenant';
        UPDATE shared_task_dependencies SET tenant_id = 'default_tenant' WHERE tenant_id IS NULL;
        ALTER TABLE shared_task_dependencies ALTER COLUMN tenant_id SET NOT NULL;
        CREATE INDEX IF NOT EXISTS idx_shared_task_dependencies_tenant_id ON shared_task_dependencies(tenant_id);
    END IF;

    IF to_regclass('shared_tasks') IS NOT NULL AND NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name='shared_tasks' AND column_name='tenant_id') THEN
        ALTER TABLE shared_tasks ADD COLUMN tenant_id TEXT DEFAULT 'default_tenant';
        UPDATE shared_tasks SET tenant_id = 'default_tenant' WHERE tenant_id IS NULL;
        ALTER TABLE shared_tasks ALTER COLUMN tenant_id SET NOT NULL;
        CREATE INDEX IF NOT EXISTS idx_shared_tasks_tenant_id ON shared_tasks(tenant_id);
    END IF;

    -- Ensure RLS is enabled and policies are created for all public tables with tenant_id
    DECLARE
        t_name text;
    BEGIN
        FOR t_name IN
            SELECT tablename
            FROM pg_tables
            WHERE schemaname = 'public'
        LOOP
            IF EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name=t_name AND column_name='tenant_id') THEN
                -- Enable RLS
                EXECUTE format('ALTER TABLE %I ENABLE ROW LEVEL SECURITY;', t_name);

                -- Create policy if it doesn't exist
                IF NOT EXISTS (
                    SELECT 1 FROM pg_policies
                    WHERE schemaname = 'public'
                      AND tablename = t_name
                      AND policyname = 'tenant_isolation_' || t_name
                ) THEN
                    EXECUTE format('CREATE POLICY tenant_isolation_%I ON %I USING (tenant_id::text = current_setting(''app.current_tenant'', true)) WITH CHECK (tenant_id::text = current_setting(''app.current_tenant'', true));', t_name, t_name);
                END IF;
            END IF;
        END LOOP;
    END;
END $$;
