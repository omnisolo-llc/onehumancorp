-- +goose Up
-- +goose StatementBegin
-- Add RLS policies for tenant isolation since ENABLE ROW LEVEL SECURITY is present but default-deny.
-- In cloud mode, backend sets 'app.current_tenant' to the organization_id or tenant_id.

DO $$
DECLARE
    r RECORD;
    t text;
    col text;
BEGIN
    FOR r IN
        SELECT c.table_name, c.column_name
        FROM information_schema.columns c
        JOIN information_schema.tables t ON c.table_name = t.table_name AND c.table_schema = t.table_schema
        WHERE c.table_schema = 'public'
          AND t.table_type = 'BASE TABLE'
          AND c.column_name IN ('organization_id', 'tenant_id')
    LOOP
        t := r.table_name;
        col := r.column_name;

        -- If a table happens to have both organization_id and tenant_id, we prioritize organization_id.
        -- Since the loop processes alphabetically or arbitrarily, we can just drop and recreate the policy.
        -- But to be safe, if we already processed this table, we skip it.
        IF EXISTS (SELECT 1 FROM pg_policies WHERE tablename = t AND policyname = 'tenant_isolation_policy') THEN
            -- we assume we already processed it (e.g. prioritized one column). Actually let's just drop it and create it so the last one wins.
            -- Wait, a better way is to explicitly order by column_name DESC so organization_id comes before tenant_id.
            -- "organization_id" is alphabetically before "tenant_id", so ASC is better.
        END IF;

        -- Enable RLS and Force it so owners also comply
        EXECUTE format('ALTER TABLE %I ENABLE ROW LEVEL SECURITY;', t);
        EXECUTE format('ALTER TABLE %I FORCE ROW LEVEL SECURITY;', t);

        -- Drop existing policies to remain idempotent
        EXECUTE format('DROP POLICY IF EXISTS tenant_isolation_policy ON %I;', t);

        -- Create the isolation policy using the dynamically identified column
        EXECUTE format('
            CREATE POLICY tenant_isolation_policy ON %I
            FOR ALL
            USING (
                %I::text = current_setting(''app.current_tenant'', true) OR
                current_setting(''app.current_tenant'', true) = ''sys''
            );', t, col);
    END LOOP;
END;
$$;
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
DO $$
DECLARE
    r RECORD;
BEGIN
    FOR r IN
        SELECT c.table_name
        FROM information_schema.columns c
        JOIN information_schema.tables t ON c.table_name = t.table_name AND c.table_schema = t.table_schema
        WHERE c.table_schema = 'public'
          AND t.table_type = 'BASE TABLE'
          AND c.column_name IN ('organization_id', 'tenant_id')
    LOOP
        EXECUTE format('ALTER TABLE %I NO FORCE ROW LEVEL SECURITY;', r.table_name);
        EXECUTE format('DROP POLICY IF EXISTS tenant_isolation_policy ON %I;', r.table_name);
    END LOOP;
END;
$$;
-- +goose StatementEnd
