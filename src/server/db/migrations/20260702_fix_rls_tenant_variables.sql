-- +goose Up
-- Issue: 202
-- Description: Fix RLS tenant variable naming (app.current_tenant_id -> app.current_tenant) and add missing RLS

DO $$
BEGIN
    -- customer_profile
    IF EXISTS (SELECT 1 FROM pg_policies WHERE tablename = 'customer_profile' AND policyname = 'customer_profile_tenant_isolation_policy') THEN
        DROP POLICY customer_profile_tenant_isolation_policy ON customer_profile;
    END IF;
    IF to_regclass('customer_profile') IS NOT NULL THEN
        CREATE POLICY customer_profile_tenant_isolation_policy ON customer_profile FOR ALL USING (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;

    -- work_item
    IF EXISTS (SELECT 1 FROM pg_policies WHERE tablename = 'work_item' AND policyname = 'work_item_tenant_isolation_policy') THEN
        DROP POLICY work_item_tenant_isolation_policy ON work_item;
    END IF;
    IF to_regclass('work_item') IS NOT NULL THEN
        CREATE POLICY work_item_tenant_isolation_policy ON work_item FOR ALL USING (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;

    -- agent_draft
    IF EXISTS (SELECT 1 FROM pg_policies WHERE tablename = 'agent_draft' AND policyname = 'agent_draft_tenant_isolation_policy') THEN
        DROP POLICY agent_draft_tenant_isolation_policy ON agent_draft;
    END IF;
    IF to_regclass('agent_draft') IS NOT NULL THEN
        CREATE POLICY agent_draft_tenant_isolation_policy ON agent_draft FOR ALL USING (EXISTS (SELECT 1 FROM work_item WHERE work_item.id = agent_draft.work_item_id AND work_item.tenant_id::text = current_setting('app.current_tenant', true)));
    END IF;

    -- proposed_bookings
    IF EXISTS (SELECT 1 FROM pg_policies WHERE tablename = 'proposed_bookings' AND policyname = 'proposed_bookings_tenant_isolation') THEN
        DROP POLICY proposed_bookings_tenant_isolation ON proposed_bookings;
    END IF;
    IF to_regclass('proposed_bookings') IS NOT NULL THEN
        CREATE POLICY proposed_bookings_tenant_isolation ON proposed_bookings USING (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;

    -- work_tasks
    IF EXISTS (SELECT 1 FROM pg_policies WHERE tablename = 'work_tasks' AND policyname = 'work_tasks_tenant_isolation') THEN
        DROP POLICY work_tasks_tenant_isolation ON work_tasks;
    END IF;
    IF to_regclass('work_tasks') IS NOT NULL THEN
        CREATE POLICY work_tasks_tenant_isolation ON work_tasks USING (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;

    -- waitlist_campaigns
    IF EXISTS (SELECT 1 FROM pg_policies WHERE tablename = 'waitlist_campaigns' AND policyname = 'Tenant isolation for waitlist_campaigns select') THEN
        DROP POLICY "Tenant isolation for waitlist_campaigns select" ON waitlist_campaigns;
    END IF;
    IF EXISTS (SELECT 1 FROM pg_policies WHERE tablename = 'waitlist_campaigns' AND policyname = 'Tenant isolation for waitlist_campaigns insert') THEN
        DROP POLICY "Tenant isolation for waitlist_campaigns insert" ON waitlist_campaigns;
    END IF;
    IF EXISTS (SELECT 1 FROM pg_policies WHERE tablename = 'waitlist_campaigns' AND policyname = 'Tenant isolation for waitlist_campaigns update') THEN
        DROP POLICY "Tenant isolation for waitlist_campaigns update" ON waitlist_campaigns;
    END IF;
    IF EXISTS (SELECT 1 FROM pg_policies WHERE tablename = 'waitlist_campaigns' AND policyname = 'Tenant isolation for waitlist_campaigns delete') THEN
        DROP POLICY "Tenant isolation for waitlist_campaigns delete" ON waitlist_campaigns;
    END IF;
    IF to_regclass('waitlist_campaigns') IS NOT NULL THEN
        CREATE POLICY "Tenant isolation for waitlist_campaigns select" ON waitlist_campaigns FOR SELECT USING (tenant_id::text = current_setting('app.current_tenant', true));
        CREATE POLICY "Tenant isolation for waitlist_campaigns insert" ON waitlist_campaigns FOR INSERT WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
        CREATE POLICY "Tenant isolation for waitlist_campaigns update" ON waitlist_campaigns FOR UPDATE USING (tenant_id::text = current_setting('app.current_tenant', true));
        CREATE POLICY "Tenant isolation for waitlist_campaigns delete" ON waitlist_campaigns FOR DELETE USING (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;

    -- pre_order_entries
    IF EXISTS (SELECT 1 FROM pg_policies WHERE tablename = 'pre_order_entries' AND policyname = 'Tenant isolation for pre_order_entries select') THEN
        DROP POLICY "Tenant isolation for pre_order_entries select" ON pre_order_entries;
    END IF;
    IF EXISTS (SELECT 1 FROM pg_policies WHERE tablename = 'pre_order_entries' AND policyname = 'Tenant isolation for pre_order_entries insert') THEN
        DROP POLICY "Tenant isolation for pre_order_entries insert" ON pre_order_entries;
    END IF;
    IF EXISTS (SELECT 1 FROM pg_policies WHERE tablename = 'pre_order_entries' AND policyname = 'Tenant isolation for pre_order_entries update') THEN
        DROP POLICY "Tenant isolation for pre_order_entries update" ON pre_order_entries;
    END IF;
    IF EXISTS (SELECT 1 FROM pg_policies WHERE tablename = 'pre_order_entries' AND policyname = 'Tenant isolation for pre_order_entries delete') THEN
        DROP POLICY "Tenant isolation for pre_order_entries delete" ON pre_order_entries;
    END IF;
    IF to_regclass('pre_order_entries') IS NOT NULL THEN
        CREATE POLICY "Tenant isolation for pre_order_entries select" ON pre_order_entries FOR SELECT USING (tenant_id::text = current_setting('app.current_tenant', true));
        CREATE POLICY "Tenant isolation for pre_order_entries insert" ON pre_order_entries FOR INSERT WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
        CREATE POLICY "Tenant isolation for pre_order_entries update" ON pre_order_entries FOR UPDATE USING (tenant_id::text = current_setting('app.current_tenant', true));
        CREATE POLICY "Tenant isolation for pre_order_entries delete" ON pre_order_entries FOR DELETE USING (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;

    -- social_post_proposals
    IF EXISTS (SELECT 1 FROM pg_policies WHERE tablename = 'social_post_proposals' AND policyname = 'tenant_isolation_social_post_proposals') THEN
        DROP POLICY tenant_isolation_social_post_proposals ON social_post_proposals;
    END IF;
    IF to_regclass('social_post_proposals') IS NOT NULL THEN
        CREATE POLICY tenant_isolation_social_post_proposals ON social_post_proposals FOR ALL USING (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;

    -- availability_schedules
    IF EXISTS (SELECT 1 FROM pg_policies WHERE tablename = 'availability_schedules' AND policyname = 'availability_schedules_tenant_isolation') THEN
        DROP POLICY availability_schedules_tenant_isolation ON availability_schedules;
    END IF;
    IF to_regclass('availability_schedules') IS NOT NULL THEN
        CREATE POLICY availability_schedules_tenant_isolation ON availability_schedules USING (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;

    -- calendar_integrations
    IF EXISTS (SELECT 1 FROM pg_policies WHERE tablename = 'calendar_integrations' AND policyname = 'calendar_integrations_tenant_isolation') THEN
        DROP POLICY calendar_integrations_tenant_isolation ON calendar_integrations;
    END IF;
    IF to_regclass('calendar_integrations') IS NOT NULL THEN
        CREATE POLICY calendar_integrations_tenant_isolation ON calendar_integrations USING (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;

END
$$;

ALTER TABLE IF EXISTS embedding_cache ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS epics ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS telemetry_buffer ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS task_dependencies ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS shared_task_dependencies ENABLE ROW LEVEL SECURITY;

-- +goose Down
-- Revert RLS policy updates back to app.current_tenant_id if needed
DO $$
BEGIN
    IF to_regclass('customer_profile') IS NOT NULL THEN
        DROP POLICY IF EXISTS customer_profile_tenant_isolation_policy ON customer_profile;
        CREATE POLICY customer_profile_tenant_isolation_policy ON customer_profile FOR ALL USING (tenant_id::text = current_setting('app.current_tenant_id', true));
    END IF;

    IF to_regclass('work_item') IS NOT NULL THEN
        DROP POLICY IF EXISTS work_item_tenant_isolation_policy ON work_item;
        CREATE POLICY work_item_tenant_isolation_policy ON work_item FOR ALL USING (tenant_id::text = current_setting('app.current_tenant_id', true));
    END IF;

    IF to_regclass('agent_draft') IS NOT NULL THEN
        DROP POLICY IF EXISTS agent_draft_tenant_isolation_policy ON agent_draft;
        CREATE POLICY agent_draft_tenant_isolation_policy ON agent_draft FOR ALL USING (EXISTS (SELECT 1 FROM work_item WHERE work_item.id = agent_draft.work_item_id AND work_item.tenant_id::text = current_setting('app.current_tenant_id', true)));
    END IF;

    IF to_regclass('proposed_bookings') IS NOT NULL THEN
        DROP POLICY IF EXISTS proposed_bookings_tenant_isolation ON proposed_bookings;
        CREATE POLICY proposed_bookings_tenant_isolation ON proposed_bookings USING (tenant_id::text = current_setting('app.current_tenant_id', true));
    END IF;

    IF to_regclass('work_tasks') IS NOT NULL THEN
        DROP POLICY IF EXISTS work_tasks_tenant_isolation ON work_tasks;
        CREATE POLICY work_tasks_tenant_isolation ON work_tasks USING (tenant_id::text = current_setting('app.current_tenant_id', true));
    END IF;

    IF to_regclass('waitlist_campaigns') IS NOT NULL THEN
        DROP POLICY IF EXISTS "Tenant isolation for waitlist_campaigns select" ON waitlist_campaigns;
        DROP POLICY IF EXISTS "Tenant isolation for waitlist_campaigns insert" ON waitlist_campaigns;
        DROP POLICY IF EXISTS "Tenant isolation for waitlist_campaigns update" ON waitlist_campaigns;
        DROP POLICY IF EXISTS "Tenant isolation for waitlist_campaigns delete" ON waitlist_campaigns;

        CREATE POLICY "Tenant isolation for waitlist_campaigns select" ON waitlist_campaigns FOR SELECT USING (tenant_id::text = current_setting('app.current_tenant_id', true));
        CREATE POLICY "Tenant isolation for waitlist_campaigns insert" ON waitlist_campaigns FOR INSERT WITH CHECK (tenant_id::text = current_setting('app.current_tenant_id', true));
        CREATE POLICY "Tenant isolation for waitlist_campaigns update" ON waitlist_campaigns FOR UPDATE USING (tenant_id::text = current_setting('app.current_tenant_id', true));
        CREATE POLICY "Tenant isolation for waitlist_campaigns delete" ON waitlist_campaigns FOR DELETE USING (tenant_id::text = current_setting('app.current_tenant_id', true));
    END IF;

    IF to_regclass('pre_order_entries') IS NOT NULL THEN
        DROP POLICY IF EXISTS "Tenant isolation for pre_order_entries select" ON pre_order_entries;
        DROP POLICY IF EXISTS "Tenant isolation for pre_order_entries insert" ON pre_order_entries;
        DROP POLICY IF EXISTS "Tenant isolation for pre_order_entries update" ON pre_order_entries;
        DROP POLICY IF EXISTS "Tenant isolation for pre_order_entries delete" ON pre_order_entries;

        CREATE POLICY "Tenant isolation for pre_order_entries select" ON pre_order_entries FOR SELECT USING (tenant_id::text = current_setting('app.current_tenant_id', true));
        CREATE POLICY "Tenant isolation for pre_order_entries insert" ON pre_order_entries FOR INSERT WITH CHECK (tenant_id::text = current_setting('app.current_tenant_id', true));
        CREATE POLICY "Tenant isolation for pre_order_entries update" ON pre_order_entries FOR UPDATE USING (tenant_id::text = current_setting('app.current_tenant_id', true));
        CREATE POLICY "Tenant isolation for pre_order_entries delete" ON pre_order_entries FOR DELETE USING (tenant_id::text = current_setting('app.current_tenant_id', true));
    END IF;

    IF to_regclass('social_post_proposals') IS NOT NULL THEN
        DROP POLICY IF EXISTS tenant_isolation_social_post_proposals ON social_post_proposals;
        CREATE POLICY tenant_isolation_social_post_proposals ON social_post_proposals FOR ALL USING (tenant_id::text = current_setting('app.current_tenant_id', true));
    END IF;

    IF to_regclass('availability_schedules') IS NOT NULL THEN
        DROP POLICY IF EXISTS availability_schedules_tenant_isolation ON availability_schedules;
        CREATE POLICY availability_schedules_tenant_isolation ON availability_schedules USING (tenant_id::text = current_setting('app.current_tenant_id', true));
    END IF;

    IF to_regclass('calendar_integrations') IS NOT NULL THEN
        DROP POLICY IF EXISTS calendar_integrations_tenant_isolation ON calendar_integrations;
        CREATE POLICY calendar_integrations_tenant_isolation ON calendar_integrations USING (tenant_id::text = current_setting('app.current_tenant_id', true));
    END IF;
END
$$;

ALTER TABLE IF EXISTS embedding_cache DISABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS epics DISABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS telemetry_buffer DISABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS task_dependencies DISABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS shared_task_dependencies DISABLE ROW LEVEL SECURITY;
