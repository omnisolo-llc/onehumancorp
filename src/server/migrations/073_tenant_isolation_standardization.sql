-- 073_tenant_isolation_standardization.sql

-- Standardize organization_id to tenant_id across all relevant tables
DO $$
BEGIN
    -- Rename columns if they exist and target doesn't
    IF EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'shared_tasks_v4' AND column_name = 'organization_id') THEN
        ALTER TABLE shared_tasks_v4 RENAME COLUMN organization_id TO tenant_id;
    END IF;
    IF EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'shared_tasks' AND column_name = 'organization_id') THEN
        ALTER TABLE shared_tasks RENAME COLUMN organization_id TO tenant_id;
    END IF;
    IF EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'autodream_memories' AND column_name = 'organization_id') THEN
        ALTER TABLE autodream_memories RENAME COLUMN organization_id TO tenant_id;
    END IF;
    IF EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'agent_memories' AND column_name = 'organization_id') THEN
        ALTER TABLE agent_memories RENAME COLUMN organization_id TO tenant_id;
    END IF;
    IF EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'agent_missions' AND column_name = 'organization_id') THEN
        ALTER TABLE agent_missions RENAME COLUMN organization_id TO tenant_id;
    END IF;
    IF EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'swarm_tasks' AND column_name = 'organization_id') THEN
        ALTER TABLE swarm_tasks RENAME COLUMN organization_id TO tenant_id;
    END IF;
    IF EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'state_machine_transitions' AND column_name = 'organization_id') THEN
        ALTER TABLE state_machine_transitions RENAME COLUMN organization_id TO tenant_id;
    END IF;
    IF EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'referrals' AND column_name = 'organization_id') THEN
        ALTER TABLE referrals RENAME COLUMN organization_id TO tenant_id;
    END IF;
    IF EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'hybrid_fs_sync_queue' AND column_name = 'organization_id') THEN
        ALTER TABLE hybrid_fs_sync_queue RENAME COLUMN organization_id TO tenant_id;
    END IF;
    IF EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'competitor_metrics' AND column_name = 'organization_id') THEN
        ALTER TABLE competitor_metrics RENAME COLUMN organization_id TO tenant_id;
    END IF;
    IF EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'agent_violations' AND column_name = 'organization_id') THEN
        ALTER TABLE agent_violations RENAME COLUMN organization_id TO tenant_id;
    END IF;

    -- For sub_agent_queue, some previous migrations added tenant_id and kept organization_id.
    -- Drop organization_id if tenant_id exists, else rename.
    IF EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'sub_agent_queue' AND column_name = 'organization_id') THEN
        IF EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'sub_agent_queue' AND column_name = 'tenant_id') THEN
            ALTER TABLE sub_agent_queue DROP COLUMN organization_id;
        ELSE
            ALTER TABLE sub_agent_queue RENAME COLUMN organization_id TO tenant_id;
        END IF;
    END IF;

    -- Add tenant_id if missing
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'sub_agent_queue' AND column_name = 'tenant_id') THEN
        ALTER TABLE sub_agent_queue ADD COLUMN tenant_id TEXT DEFAULT 'system';
    END IF;

    -- For onboarding_state and products which already have both
    IF EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'onboarding_state' AND column_name = 'organization_id') THEN
        ALTER TABLE onboarding_state DROP COLUMN organization_id;
    END IF;
    IF EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'products' AND column_name = 'organization_id') THEN
        ALTER TABLE products DROP COLUMN organization_id;
    END IF;
    IF EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'agents' AND column_name = 'organization_id') THEN
        ALTER TABLE agents DROP COLUMN organization_id;
    END IF;

END $$;


-- Enable and Force Row Level Security on all core tables
ALTER TABLE tenants ENABLE ROW LEVEL SECURITY;
ALTER TABLE tenants FORCE ROW LEVEL SECURITY;

ALTER TABLE shared_tasks ENABLE ROW LEVEL SECURITY;
ALTER TABLE shared_tasks FORCE ROW LEVEL SECURITY;

ALTER TABLE shared_tasks_v4 ENABLE ROW LEVEL SECURITY;
ALTER TABLE shared_tasks_v4 FORCE ROW LEVEL SECURITY;

ALTER TABLE swarm_tasks ENABLE ROW LEVEL SECURITY;
ALTER TABLE swarm_tasks FORCE ROW LEVEL SECURITY;

ALTER TABLE agent_missions ENABLE ROW LEVEL SECURITY;
ALTER TABLE agent_missions FORCE ROW LEVEL SECURITY;

ALTER TABLE state_machine_transitions ENABLE ROW LEVEL SECURITY;
ALTER TABLE state_machine_transitions FORCE ROW LEVEL SECURITY;

ALTER TABLE sub_agent_queue ENABLE ROW LEVEL SECURITY;
ALTER TABLE sub_agent_queue FORCE ROW LEVEL SECURITY;

ALTER TABLE autodream_memories ENABLE ROW LEVEL SECURITY;
ALTER TABLE autodream_memories FORCE ROW LEVEL SECURITY;

ALTER TABLE agent_memories ENABLE ROW LEVEL SECURITY;
ALTER TABLE agent_memories FORCE ROW LEVEL SECURITY;

ALTER TABLE products ENABLE ROW LEVEL SECURITY;
ALTER TABLE products FORCE ROW LEVEL SECURITY;

ALTER TABLE referrals ENABLE ROW LEVEL SECURITY;
ALTER TABLE referrals FORCE ROW LEVEL SECURITY;

ALTER TABLE onboarding_state ENABLE ROW LEVEL SECURITY;
ALTER TABLE onboarding_state FORCE ROW LEVEL SECURITY;

ALTER TABLE customers ENABLE ROW LEVEL SECURITY;
ALTER TABLE customers FORCE ROW LEVEL SECURITY;

ALTER TABLE orders ENABLE ROW LEVEL SECURITY;
ALTER TABLE orders FORCE ROW LEVEL SECURITY;

-- Drop all old organization_id RLS policies
DROP POLICY IF EXISTS tenant_isolation_shared_tasks ON shared_tasks;
DROP POLICY IF EXISTS tenant_isolation_swarm_tasks ON swarm_tasks;
DROP POLICY IF EXISTS tenant_isolation_agent_missions ON agent_missions;
DROP POLICY IF EXISTS tenant_isolation_state_machine_transitions ON state_machine_transitions;
DROP POLICY IF EXISTS tenant_isolation_sub_agent_queue ON sub_agent_queue;
DROP POLICY IF EXISTS tenant_isolation_sub_agent_queue_t ON sub_agent_queue;
DROP POLICY IF EXISTS tenant_isolation_sub_agent_queue_strict ON sub_agent_queue;
DROP POLICY IF EXISTS tenant_isolation_autodream_memories ON autodream_memories;
DROP POLICY IF EXISTS tenant_isolation_agent_memories ON agent_memories;
DROP POLICY IF EXISTS tenant_isolation_products ON products;
DROP POLICY IF EXISTS referrals_isolation_policy ON referrals;
DROP POLICY IF EXISTS tenant_isolation_onboarding_state ON onboarding_state;

-- Recreate strict policies checking tenant_id = app.current_tenant
CREATE POLICY tenant_isolation_shared_tasks ON shared_tasks USING (tenant_id::text = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_swarm_tasks ON swarm_tasks USING (tenant_id::text = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_agent_missions ON agent_missions USING (tenant_id::text = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_state_machine_transitions ON state_machine_transitions USING (tenant_id::text = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_sub_agent_queue ON sub_agent_queue USING (tenant_id::text = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_autodream_memories ON autodream_memories USING (tenant_id::text = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_agent_memories ON agent_memories USING (tenant_id::text = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_products ON products USING (tenant_id::text = current_setting('app.current_tenant', true));
CREATE POLICY referrals_isolation_policy ON referrals USING (tenant_id::text = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_onboarding_state ON onboarding_state USING (tenant_id::text = current_setting('app.current_tenant', true));
