-- +goose Up
-- Fix incorrect RLS policies using `app.current_tenant_id`

-- Fix customer_profile
DROP POLICY IF EXISTS customer_profile_tenant_isolation_policy ON customer_profile;
CREATE POLICY customer_profile_tenant_isolation_policy ON customer_profile FOR ALL USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Fix work_item
DROP POLICY IF EXISTS work_item_tenant_isolation_policy ON work_item;
CREATE POLICY work_item_tenant_isolation_policy ON work_item FOR ALL USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Fix agent_draft (omnichannel_tables)
DROP POLICY IF EXISTS agent_draft_tenant_isolation_policy ON agent_draft;
CREATE POLICY agent_draft_tenant_isolation_policy ON agent_draft FOR ALL
USING (EXISTS (SELECT 1 FROM work_item WHERE work_item.id = agent_draft.work_item_id AND work_item.tenant_id::text = current_setting('app.current_tenant', true)))
WITH CHECK (EXISTS (SELECT 1 FROM work_item WHERE work_item.id = agent_draft.work_item_id AND work_item.tenant_id::text = current_setting('app.current_tenant', true)));

-- Fix staff_shifts
DROP POLICY IF EXISTS staff_shifts_tenant_isolation ON staff_shifts;
CREATE POLICY staff_shifts_tenant_isolation ON staff_shifts FOR ALL USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Fix staff_members
DROP POLICY IF EXISTS staff_members_tenant_isolation ON staff_members;
CREATE POLICY staff_members_tenant_isolation ON staff_members FOR ALL USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Fix staff_tasks
DROP POLICY IF EXISTS staff_tasks_tenant_isolation ON staff_tasks;
CREATE POLICY staff_tasks_tenant_isolation ON staff_tasks FOR ALL USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Fix staff_task_assignments
DROP POLICY IF EXISTS staff_task_assignments_tenant_isolation ON staff_task_assignments;
CREATE POLICY staff_task_assignments_tenant_isolation ON staff_task_assignments FOR ALL USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Fix multi_platform_syndication
DROP POLICY IF EXISTS tenant_isolation_syndication_campaigns ON syndication_campaigns;
CREATE POLICY tenant_isolation_syndication_campaigns ON syndication_campaigns FOR ALL USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

DROP POLICY IF EXISTS tenant_isolation_syndication_posts ON syndication_posts;
CREATE POLICY tenant_isolation_syndication_posts ON syndication_posts FOR ALL USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- Fix proposed_bookings
DROP POLICY IF EXISTS tenant_isolation_proposed_bookings ON proposed_bookings;
CREATE POLICY tenant_isolation_proposed_bookings ON proposed_bookings FOR ALL USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

DROP POLICY IF EXISTS tenant_isolation_proposed_booking_options ON proposed_booking_options;
CREATE POLICY tenant_isolation_proposed_booking_options ON proposed_booking_options FOR ALL USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Fix pre_order_waitlist_campaigns
DROP POLICY IF EXISTS "Tenant isolation for waitlist_campaigns select" ON waitlist_campaigns;
DROP POLICY IF EXISTS "Tenant isolation for waitlist_campaigns insert" ON waitlist_campaigns;
DROP POLICY IF EXISTS "Tenant isolation for waitlist_campaigns update" ON waitlist_campaigns;
DROP POLICY IF EXISTS "Tenant isolation for waitlist_campaigns delete" ON waitlist_campaigns;

CREATE POLICY "Tenant isolation for waitlist_campaigns select" ON waitlist_campaigns FOR SELECT USING (tenant_id::text = current_setting('app.current_tenant', true));
CREATE POLICY "Tenant isolation for waitlist_campaigns insert" ON waitlist_campaigns FOR INSERT WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
CREATE POLICY "Tenant isolation for waitlist_campaigns update" ON waitlist_campaigns FOR UPDATE USING (tenant_id::text = current_setting('app.current_tenant', true));
CREATE POLICY "Tenant isolation for waitlist_campaigns delete" ON waitlist_campaigns FOR DELETE USING (tenant_id::text = current_setting('app.current_tenant', true));

DROP POLICY IF EXISTS "Tenant isolation for pre_order_entries select" ON pre_order_entries;
DROP POLICY IF EXISTS "Tenant isolation for pre_order_entries insert" ON pre_order_entries;
DROP POLICY IF EXISTS "Tenant isolation for pre_order_entries update" ON pre_order_entries;
DROP POLICY IF EXISTS "Tenant isolation for pre_order_entries delete" ON pre_order_entries;

CREATE POLICY "Tenant isolation for pre_order_entries select" ON pre_order_entries FOR SELECT USING (tenant_id::text = current_setting('app.current_tenant', true));
CREATE POLICY "Tenant isolation for pre_order_entries insert" ON pre_order_entries FOR INSERT WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
CREATE POLICY "Tenant isolation for pre_order_entries update" ON pre_order_entries FOR UPDATE USING (tenant_id::text = current_setting('app.current_tenant', true));
CREATE POLICY "Tenant isolation for pre_order_entries delete" ON pre_order_entries FOR DELETE USING (tenant_id::text = current_setting('app.current_tenant', true));

-- +goose Down
