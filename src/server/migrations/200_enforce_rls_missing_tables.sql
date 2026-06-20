-- +goose Up
-- Add missing RLS policies to tables that store tenant data but missed them

-- From 002_missing_tables.sql
DO $$
DECLARE
    t_name text;
    pol_name text;
BEGIN
    FOR t_name IN
        SELECT unnest(ARRAY[
            'meeting_rooms',
            'meeting_transcripts'
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

-- From 017_business_milestones.sql
ALTER TABLE business_milestones ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_business_milestones ON business_milestones;
CREATE POLICY tenant_isolation_business_milestones ON business_milestones USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- From 058_shared_tasks_decomposition_table.sql
ALTER TABLE shared_tasks_decomposition ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_shared_tasks_decomposition ON shared_tasks_decomposition;
CREATE POLICY tenant_isolation_shared_tasks_decomposition ON shared_tasks_decomposition USING (organization_id::text = current_setting('app.current_tenant', true)) WITH CHECK (organization_id::text = current_setting('app.current_tenant', true));

-- From 074_missing_tables.sql
DO $$
DECLARE
    t_name text;
    pol_name text;
BEGIN
    FOR t_name IN
        SELECT unnest(ARRAY[
            'interactions',
            'agent_actions',
            'customer_timeline'
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

-- From 008_data_model_architecture.sql
DO $$
DECLARE
    t_name text;
    pol_name text;
BEGIN
    FOR t_name IN
        SELECT unnest(ARRAY[
            'tenants',
            'customers',
            'products',
            'services',
            'orders',
            'order_line_items',
            'bookings',
            'ai_memories'
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
                IF t_name = 'tenants' THEN
                    EXECUTE format(
                        'CREATE POLICY %I ON %I USING (id::text = current_setting(''app.current_tenant'', true)) WITH CHECK (id::text = current_setting(''app.current_tenant'', true))',
                        pol_name,
                        t_name
                    );
                ELSE
                    EXECUTE format(
                        'CREATE POLICY %I ON %I USING (tenant_id::text = current_setting(''app.current_tenant'', true)) WITH CHECK (tenant_id::text = current_setting(''app.current_tenant'', true))',
                        pol_name,
                        t_name
                    );
                END IF;
            END IF;
        END IF;
    END LOOP;
END
$$;

-- +goose Down
-- Revert RLS
ALTER TABLE business_milestones DISABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_business_milestones ON business_milestones;

ALTER TABLE shared_tasks_decomposition DISABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_shared_tasks_decomposition ON shared_tasks_decomposition;

DO $$
DECLARE
    t_name text;
    pol_name text;
BEGIN
    FOR t_name IN
        SELECT unnest(ARRAY[
            'meeting_rooms',
            'meeting_transcripts',
            'interactions',
            'agent_actions',
            'customer_timeline',
            'tenants',
            'customers',
            'products',
            'services',
            'orders',
            'order_line_items',
            'bookings',
            'ai_memories'
        ])
    LOOP
        IF to_regclass(t_name) IS NOT NULL THEN
            EXECUTE format('ALTER TABLE %I DISABLE ROW LEVEL SECURITY', t_name);
            pol_name := format('tenant_isolation_%s', t_name);
            EXECUTE format('DROP POLICY IF EXISTS %I ON %I', pol_name, t_name);
        END IF;
    END LOOP;
END
$$;
