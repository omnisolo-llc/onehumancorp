-- +goose Up

-- 1. Fix interactive_proposal_line_items
ALTER TABLE IF EXISTS interactive_proposal_line_items ADD COLUMN IF NOT EXISTS tenant_id TEXT;
UPDATE interactive_proposal_line_items SET tenant_id = (SELECT tenant_id FROM interactive_proposals WHERE interactive_proposals.id = interactive_proposal_line_items.proposal_id) WHERE tenant_id IS NULL;
UPDATE interactive_proposal_line_items SET tenant_id = 'default_tenant' WHERE tenant_id IS NULL;
ALTER TABLE IF EXISTS interactive_proposal_line_items ALTER COLUMN tenant_id SET DEFAULT current_setting('app.current_tenant', true);
ALTER TABLE IF EXISTS interactive_proposal_line_items ALTER COLUMN tenant_id SET NOT NULL;

-- 2. Fix agent_draft
ALTER TABLE IF EXISTS agent_draft ADD COLUMN IF NOT EXISTS tenant_id TEXT;
UPDATE agent_draft SET tenant_id = 'default_tenant' WHERE tenant_id IS NULL;
ALTER TABLE IF EXISTS agent_draft ALTER COLUMN tenant_id SET DEFAULT current_setting('app.current_tenant', true);
ALTER TABLE IF EXISTS agent_draft ALTER COLUMN tenant_id SET NOT NULL;

-- 3. Fix delivery_zones, route_plans, delivery_tasks
ALTER TABLE IF EXISTS delivery_zones ADD COLUMN IF NOT EXISTS tenant_id TEXT;
UPDATE delivery_zones SET tenant_id = 'default_tenant' WHERE tenant_id IS NULL;
ALTER TABLE IF EXISTS delivery_zones ALTER COLUMN tenant_id SET DEFAULT current_setting('app.current_tenant', true);
ALTER TABLE IF EXISTS delivery_zones ALTER COLUMN tenant_id SET NOT NULL;

ALTER TABLE IF EXISTS route_plans ADD COLUMN IF NOT EXISTS tenant_id TEXT;
UPDATE route_plans SET tenant_id = 'default_tenant' WHERE tenant_id IS NULL;
ALTER TABLE IF EXISTS route_plans ALTER COLUMN tenant_id SET DEFAULT current_setting('app.current_tenant', true);
ALTER TABLE IF EXISTS route_plans ALTER COLUMN tenant_id SET NOT NULL;

ALTER TABLE IF EXISTS delivery_tasks ADD COLUMN IF NOT EXISTS tenant_id TEXT;
UPDATE delivery_tasks SET tenant_id = 'default_tenant' WHERE tenant_id IS NULL;
ALTER TABLE IF EXISTS delivery_tasks ALTER COLUMN tenant_id SET DEFAULT current_setting('app.current_tenant', true);
ALTER TABLE IF EXISTS delivery_tasks ALTER COLUMN tenant_id SET NOT NULL;

-- 4. Fix quote_line_items
ALTER TABLE IF EXISTS quote_line_items ADD COLUMN IF NOT EXISTS tenant_id TEXT;
UPDATE quote_line_items SET tenant_id = (SELECT tenant_id FROM quotes WHERE quotes.id = quote_line_items.quote_id) WHERE tenant_id IS NULL;
UPDATE quote_line_items SET tenant_id = 'default_tenant' WHERE tenant_id IS NULL;
ALTER TABLE IF EXISTS quote_line_items ALTER COLUMN tenant_id SET DEFAULT current_setting('app.current_tenant', true);
ALTER TABLE IF EXISTS quote_line_items ALTER COLUMN tenant_id SET NOT NULL;

-- 5. Fix proposal_line_items
ALTER TABLE IF EXISTS proposal_line_items ADD COLUMN IF NOT EXISTS tenant_id TEXT;
UPDATE proposal_line_items SET tenant_id = (SELECT tenant_id FROM proposals WHERE proposals.id = proposal_line_items.proposal_id) WHERE tenant_id IS NULL;
UPDATE proposal_line_items SET tenant_id = 'default_tenant' WHERE tenant_id IS NULL;
ALTER TABLE IF EXISTS proposal_line_items ALTER COLUMN tenant_id SET DEFAULT current_setting('app.current_tenant', true);
ALTER TABLE IF EXISTS proposal_line_items ALTER COLUMN tenant_id SET NOT NULL;

-- 6. Fix currencies, product_prices
ALTER TABLE IF EXISTS currencies ADD COLUMN IF NOT EXISTS tenant_id TEXT;
UPDATE currencies SET tenant_id = 'default_tenant' WHERE tenant_id IS NULL;
ALTER TABLE IF EXISTS currencies ALTER COLUMN tenant_id SET DEFAULT current_setting('app.current_tenant', true);
ALTER TABLE IF EXISTS currencies ALTER COLUMN tenant_id SET NOT NULL;

ALTER TABLE IF EXISTS product_prices ADD COLUMN IF NOT EXISTS tenant_id TEXT;
UPDATE product_prices SET tenant_id = 'default_tenant' WHERE tenant_id IS NULL;
ALTER TABLE IF EXISTS product_prices ALTER COLUMN tenant_id SET DEFAULT current_setting('app.current_tenant', true);
ALTER TABLE IF EXISTS product_prices ALTER COLUMN tenant_id SET NOT NULL;

-- 7. Fix shared_tasks, shared_task_dependencies
ALTER TABLE IF EXISTS shared_tasks ADD COLUMN IF NOT EXISTS tenant_id TEXT;
UPDATE shared_tasks SET tenant_id = 'default_tenant' WHERE tenant_id IS NULL;
ALTER TABLE IF EXISTS shared_tasks ALTER COLUMN tenant_id SET DEFAULT current_setting('app.current_tenant', true);
ALTER TABLE IF EXISTS shared_tasks ALTER COLUMN tenant_id SET NOT NULL;

ALTER TABLE IF EXISTS shared_task_dependencies ADD COLUMN IF NOT EXISTS tenant_id TEXT;
UPDATE shared_task_dependencies SET tenant_id = 'default_tenant' WHERE tenant_id IS NULL;
ALTER TABLE IF EXISTS shared_task_dependencies ALTER COLUMN tenant_id SET DEFAULT current_setting('app.current_tenant', true);
ALTER TABLE IF EXISTS shared_task_dependencies ALTER COLUMN tenant_id SET NOT NULL;


DO $$
DECLARE
    t TEXT;
BEGIN
    FOR t IN
        SELECT unnest(ARRAY['interactive_proposal_line_items', 'agent_draft', 'delivery_zones', 'route_plans', 'delivery_tasks', 'quote_line_items', 'proposal_line_items', 'currencies', 'product_prices', 'shared_tasks', 'shared_task_dependencies'])
    LOOP
        IF to_regclass(t) IS NOT NULL THEN

            -- Enable RLS
            EXECUTE format('ALTER TABLE %I ENABLE ROW LEVEL SECURITY', t);

            -- Create policy if not exists
            IF NOT EXISTS (
                SELECT 1 FROM pg_policies
                WHERE schemaname = current_schema()
                AND tablename = t
                AND policyname = 'tenant_isolation_' || t
            ) THEN
                EXECUTE format(
                    'CREATE POLICY tenant_isolation_%I ON %I USING (tenant_id = current_setting(''app.current_tenant'', true)) WITH CHECK (tenant_id = current_setting(''app.current_tenant'', true))',
                    t, t
                );
            END IF;
        END IF;
    END LOOP;
END
$$;

-- +goose Down
-- Revert the changes made
DO $$
DECLARE
    t TEXT;
BEGIN
    FOR t IN
        SELECT unnest(ARRAY['interactive_proposal_line_items', 'agent_draft', 'delivery_zones', 'route_plans', 'delivery_tasks', 'quote_line_items', 'proposal_line_items', 'currencies', 'product_prices', 'shared_tasks', 'shared_task_dependencies'])
    LOOP
        IF to_regclass(t) IS NOT NULL THEN
            -- Drop policy
            EXECUTE format('DROP POLICY IF EXISTS tenant_isolation_%I ON %I', t, t);
            -- Disable RLS
            EXECUTE format('ALTER TABLE %I DISABLE ROW LEVEL SECURITY', t);
        END IF;
    END LOOP;
END
$$;

ALTER TABLE IF EXISTS product_prices DROP COLUMN IF EXISTS tenant_id;
ALTER TABLE IF EXISTS currencies DROP COLUMN IF EXISTS tenant_id;
ALTER TABLE IF EXISTS proposal_line_items DROP COLUMN IF EXISTS tenant_id;
ALTER TABLE IF EXISTS quote_line_items DROP COLUMN IF EXISTS tenant_id;
ALTER TABLE IF EXISTS delivery_tasks DROP COLUMN IF EXISTS tenant_id;
ALTER TABLE IF EXISTS route_plans DROP COLUMN IF EXISTS tenant_id;
ALTER TABLE IF EXISTS delivery_zones DROP COLUMN IF EXISTS tenant_id;
ALTER TABLE IF EXISTS agent_draft DROP COLUMN IF EXISTS tenant_id;
ALTER TABLE IF EXISTS interactive_proposal_line_items DROP COLUMN IF EXISTS tenant_id;
ALTER TABLE IF EXISTS shared_task_dependencies DROP COLUMN IF EXISTS tenant_id;
ALTER TABLE IF EXISTS shared_tasks DROP COLUMN IF EXISTS tenant_id;
