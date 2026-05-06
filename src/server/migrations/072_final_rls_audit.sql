-- 072_final_rls_audit.sql
-- Final hardening of RLS policies to ensure no data leakage between tenants

-- Ensure all tables that SHOULD have RLS actually have it enabled
ALTER TABLE IF EXISTS tenants ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS agents ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS products ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS orders ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS order_items ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS customers ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS bookings ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS pages ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS memories ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS consolidated_memory ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS shared_tasks ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS swarm_tasks ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS agent_missions ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS onboarding_state ENABLE ROW LEVEL SECURITY;

-- Re-apply strict policies (dropping old ones first to avoid duplicates or weaker policies)
-- The naming convention used is tenant_isolation_<table_name>_strict

-- shared_tasks
DROP POLICY IF EXISTS tenant_isolation_shared_tasks_strict ON shared_tasks;
CREATE POLICY tenant_isolation_shared_tasks_strict ON shared_tasks
    USING (organization_id::text = current_setting('app.current_tenant', true));

-- swarm_tasks
DROP POLICY IF EXISTS tenant_isolation_swarm_tasks_strict ON swarm_tasks;
CREATE POLICY tenant_isolation_swarm_tasks_strict ON swarm_tasks
    USING (tenant_id::text = current_setting('app.current_tenant', true));

-- agent_missions
DROP POLICY IF EXISTS tenant_isolation_agent_missions_strict ON agent_missions;
CREATE POLICY tenant_isolation_agent_missions_strict ON agent_missions
    USING (organization_id::text = current_setting('app.current_tenant', true));

-- tenants
DROP POLICY IF EXISTS tenant_isolation_tenants_strict ON tenants;
CREATE POLICY tenant_isolation_tenants_strict ON tenants
    USING (tenant_id::text = current_setting('app.current_tenant', true));

-- onboarding_state
DROP POLICY IF EXISTS tenant_isolation_onboarding_state_strict ON onboarding_state;
CREATE POLICY tenant_isolation_onboarding_state_strict ON onboarding_state
    USING (tenant_id::text = current_setting('app.current_tenant', true));

-- products
DROP POLICY IF EXISTS tenant_isolation_products_strict ON products;
CREATE POLICY tenant_isolation_products_strict ON products
    USING (tenant_id::text = current_setting('app.current_tenant', true));

-- orders
DROP POLICY IF EXISTS tenant_isolation_orders_strict ON orders;
CREATE POLICY tenant_isolation_orders_strict ON orders
    USING (tenant_id::text = current_setting('app.current_tenant', true));
