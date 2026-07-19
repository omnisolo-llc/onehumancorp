-- +goose Up

DO $$
DECLARE
    t_name text;
    pol_name text;
    col_type text;
BEGIN
    FOR t_name IN
        SELECT table_name
        FROM information_schema.tables
        WHERE table_schema = 'public'
          AND table_type = 'BASE TABLE'
    LOOP
        -- Only target tables without RLS enabled
        IF EXISTS (
            SELECT 1
            FROM pg_class c
            JOIN pg_namespace n ON n.oid = c.relnamespace
            WHERE n.nspname = 'public'
              AND c.relname = t_name
              AND c.relrowsecurity = false
        ) THEN
            -- Check if table has tenant_id
            IF EXISTS (SELECT 1 FROM information_schema.columns WHERE table_schema = 'public' AND table_name=t_name AND column_name='tenant_id') THEN
                SELECT data_type INTO col_type FROM information_schema.columns WHERE table_schema = 'public' AND table_name=t_name AND column_name='tenant_id';

                -- Enable RLS
                EXECUTE format('ALTER TABLE %I ENABLE ROW LEVEL SECURITY', t_name);

                pol_name := format('tenant_isolation_%s', t_name);

                -- Create policy if it doesn't exist
                IF NOT EXISTS (
                    SELECT 1
                    FROM pg_policies
                    WHERE schemaname = 'public'
                        AND tablename = t_name
                        AND policyname = pol_name
                ) THEN
                    IF col_type = 'uuid' THEN
                        EXECUTE format(
                            'CREATE POLICY %I ON %I USING (tenant_id = NULLIF(current_setting(''app.current_tenant'', true), '''')::uuid) WITH CHECK (tenant_id = NULLIF(current_setting(''app.current_tenant'', true), '''')::uuid)',
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

            -- Check if table has organization_id
            ELSIF EXISTS (SELECT 1 FROM information_schema.columns WHERE table_schema = 'public' AND table_name=t_name AND column_name='organization_id') THEN
                SELECT data_type INTO col_type FROM information_schema.columns WHERE table_schema = 'public' AND table_name=t_name AND column_name='organization_id';

                -- Enable RLS
                EXECUTE format('ALTER TABLE %I ENABLE ROW LEVEL SECURITY', t_name);

                pol_name := format('tenant_isolation_%s', t_name);

                -- Create policy if it doesn't exist
                IF NOT EXISTS (
                    SELECT 1
                    FROM pg_policies
                    WHERE schemaname = 'public'
                        AND tablename = t_name
                        AND policyname = pol_name
                ) THEN
                    IF col_type = 'uuid' THEN
                        EXECUTE format(
                            'CREATE POLICY %I ON %I USING (organization_id = NULLIF(current_setting(''app.current_tenant'', true), '''')::uuid) WITH CHECK (organization_id = NULLIF(current_setting(''app.current_tenant'', true), '''')::uuid)',
                            pol_name,
                            t_name
                        );
                    ELSE
                        EXECUTE format(
                            'CREATE POLICY %I ON %I USING (organization_id::text = current_setting(''app.current_tenant'', true)) WITH CHECK (organization_id::text = current_setting(''app.current_tenant'', true))',
                            pol_name,
                            t_name
                        );
                    END IF;
                END IF;
            END IF;
        END IF;
    END LOOP;
END
$$;
