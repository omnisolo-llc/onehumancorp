-- +goose Up

-- Fix customer_profile RLS policy
ALTER TABLE IF EXISTS customer_profile ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS customer_profile_tenant_isolation_policy ON customer_profile;
CREATE POLICY customer_profile_tenant_isolation_policy ON customer_profile FOR ALL USING (tenant_id = current_setting('app.current_tenant', true)::uuid);

-- Fix work_item RLS policy
ALTER TABLE IF EXISTS work_item ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS work_item_tenant_isolation_policy ON work_item;
CREATE POLICY work_item_tenant_isolation_policy ON work_item FOR ALL USING (tenant_id = current_setting('app.current_tenant', true)::uuid);

-- Fix agent_draft RLS policy
ALTER TABLE IF EXISTS agent_draft ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS agent_draft_tenant_isolation_policy ON agent_draft;
CREATE POLICY agent_draft_tenant_isolation_policy ON agent_draft FOR ALL USING (
    EXISTS (
        SELECT 1 FROM work_item WHERE work_item.id = agent_draft.work_item_id AND work_item.tenant_id = current_setting('app.current_tenant', true)::uuid
    )
);

-- Fix proposed_bookings RLS policy
ALTER TABLE IF EXISTS proposed_bookings ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS proposed_bookings_tenant_isolation ON proposed_bookings;
CREATE POLICY proposed_bookings_tenant_isolation ON proposed_bookings USING (tenant_id = current_setting('app.current_tenant', true)::uuid);

-- Fix work_tasks RLS policy
ALTER TABLE IF EXISTS work_tasks ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS work_tasks_tenant_isolation ON work_tasks;
CREATE POLICY work_tasks_tenant_isolation ON work_tasks USING (tenant_id = current_setting('app.current_tenant', true)::uuid);

-- Fix availability_schedules RLS policy
ALTER TABLE IF EXISTS availability_schedules ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS availability_schedules_tenant_isolation ON availability_schedules;
CREATE POLICY availability_schedules_tenant_isolation ON availability_schedules
    USING (tenant_id = current_setting('app.current_tenant', true));

-- Fix calendar_integrations RLS policy
ALTER TABLE IF EXISTS calendar_integrations ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS calendar_integrations_tenant_isolation ON calendar_integrations;
CREATE POLICY calendar_integrations_tenant_isolation ON calendar_integrations
    USING (tenant_id = current_setting('app.current_tenant', true));

-- Fix waitlist_campaigns RLS policy
DROP POLICY IF EXISTS "Tenant isolation for waitlist_campaigns select" ON waitlist_campaigns;
DROP POLICY IF EXISTS "Tenant isolation for waitlist_campaigns insert" ON waitlist_campaigns;
DROP POLICY IF EXISTS "Tenant isolation for waitlist_campaigns update" ON waitlist_campaigns;
DROP POLICY IF EXISTS "Tenant isolation for waitlist_campaigns delete" ON waitlist_campaigns;

CREATE POLICY "Tenant isolation for waitlist_campaigns select"
    ON waitlist_campaigns FOR SELECT
    USING (tenant_id::text = current_setting('app.current_tenant', true));

CREATE POLICY "Tenant isolation for waitlist_campaigns insert"
    ON waitlist_campaigns FOR INSERT
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

CREATE POLICY "Tenant isolation for waitlist_campaigns update"
    ON waitlist_campaigns FOR UPDATE
    USING (tenant_id::text = current_setting('app.current_tenant', true));

CREATE POLICY "Tenant isolation for waitlist_campaigns delete"
    ON waitlist_campaigns FOR DELETE
    USING (tenant_id::text = current_setting('app.current_tenant', true));

-- Fix pre_order_entries RLS policy
DROP POLICY IF EXISTS "Tenant isolation for pre_order_entries select" ON pre_order_entries;
DROP POLICY IF EXISTS "Tenant isolation for pre_order_entries insert" ON pre_order_entries;
DROP POLICY IF EXISTS "Tenant isolation for pre_order_entries update" ON pre_order_entries;
DROP POLICY IF EXISTS "Tenant isolation for pre_order_entries delete" ON pre_order_entries;

CREATE POLICY "Tenant isolation for pre_order_entries select"
    ON pre_order_entries FOR SELECT
    USING (tenant_id::text = current_setting('app.current_tenant', true));

CREATE POLICY "Tenant isolation for pre_order_entries insert"
    ON pre_order_entries FOR INSERT
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

CREATE POLICY "Tenant isolation for pre_order_entries update"
    ON pre_order_entries FOR UPDATE
    USING (tenant_id::text = current_setting('app.current_tenant', true));

CREATE POLICY "Tenant isolation for pre_order_entries delete"
    ON pre_order_entries FOR DELETE
    USING (tenant_id::text = current_setting('app.current_tenant', true));


-- +goose Down
-- Intentionally blank. Down migration should not revert to a broken state.
