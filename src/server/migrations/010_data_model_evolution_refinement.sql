-- Migration 010: Data Model Evolution Refinement

-- Harmonize tenant_id naming
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM INFORMATION_SCHEMA.COLUMNS WHERE TABLE_NAME = 'tasks' AND COLUMN_NAME = 'organization_id') THEN
        ALTER TABLE tasks RENAME COLUMN organization_id TO tenant_id;
    END IF;

    IF EXISTS (SELECT 1 FROM INFORMATION_SCHEMA.COLUMNS WHERE TABLE_NAME = 'shared_tasks_v4' AND COLUMN_NAME = 'organization_id') THEN
        ALTER TABLE shared_tasks_v4 RENAME COLUMN organization_id TO tenant_id;
    END IF;

    IF EXISTS (SELECT 1 FROM INFORMATION_SCHEMA.COLUMNS WHERE TABLE_NAME = 'shared_tasks_decomposition' AND COLUMN_NAME = 'organization_id') THEN
        ALTER TABLE shared_tasks_decomposition RENAME COLUMN organization_id TO tenant_id;
    END IF;
END
$$;

-- Add priority to tasks if not exists
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM INFORMATION_SCHEMA.COLUMNS WHERE TABLE_NAME = 'tasks' AND COLUMN_NAME = 'priority') THEN
        ALTER TABLE tasks ADD COLUMN priority TEXT DEFAULT 'P2';
    END IF;
END
$$;

-- Ensure RLS is enabled and policies are applied for all tables mentioned in research brief
DO $$
DECLARE
    t_name text;
    pol_name text;
BEGIN
    FOR t_name IN
        SELECT unnest(ARRAY[
            'catalog_items',
            'item_variants',
            'inventory_ledger',
            'payments',
            'fulfillments',
            'bookings',
            'interactions',
            'agent_actions',
            'tasks',
            'shared_tasks_v4',
            'shared_tasks_decomposition'
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
