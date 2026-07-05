-- +goose Up

-- Fix customer_profile
DROP POLICY IF EXISTS customer_profile_tenant_isolation_policy ON customer_profile;
CREATE POLICY customer_profile_tenant_isolation_policy ON customer_profile FOR ALL
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Fix work_item
DROP POLICY IF EXISTS work_item_tenant_isolation_policy ON work_item;
CREATE POLICY work_item_tenant_isolation_policy ON work_item FOR ALL
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Fix agent_draft
DROP POLICY IF EXISTS agent_draft_tenant_isolation_policy ON agent_draft;
CREATE POLICY agent_draft_tenant_isolation_policy ON agent_draft FOR ALL
    USING (
        EXISTS (
            SELECT 1 FROM work_item WHERE work_item.id = agent_draft.work_item_id AND work_item.tenant_id::text = current_setting('app.current_tenant', true)
        )
    )
    WITH CHECK (
        EXISTS (
            SELECT 1 FROM work_item WHERE work_item.id = agent_draft.work_item_id AND work_item.tenant_id::text = current_setting('app.current_tenant', true)
        )
    );

-- Fix proposed_bookings
DROP POLICY IF EXISTS proposed_bookings_tenant_isolation ON proposed_bookings;
CREATE POLICY proposed_bookings_tenant_isolation ON proposed_bookings FOR ALL
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Fix waitlist_campaigns
DROP POLICY IF EXISTS "Tenant isolation for waitlist_campaigns select" ON waitlist_campaigns;
CREATE POLICY "Tenant isolation for waitlist_campaigns select"
    ON waitlist_campaigns FOR SELECT
    USING (tenant_id::text = current_setting('app.current_tenant', true));

DROP POLICY IF EXISTS "Tenant isolation for waitlist_campaigns insert" ON waitlist_campaigns;
CREATE POLICY "Tenant isolation for waitlist_campaigns insert"
    ON waitlist_campaigns FOR INSERT
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

DROP POLICY IF EXISTS "Tenant isolation for waitlist_campaigns update" ON waitlist_campaigns;
CREATE POLICY "Tenant isolation for waitlist_campaigns update"
    ON waitlist_campaigns FOR UPDATE
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

DROP POLICY IF EXISTS "Tenant isolation for waitlist_campaigns delete" ON waitlist_campaigns;
CREATE POLICY "Tenant isolation for waitlist_campaigns delete"
    ON waitlist_campaigns FOR DELETE
    USING (tenant_id::text = current_setting('app.current_tenant', true));

-- Fix pre_order_entries
DROP POLICY IF EXISTS "Tenant isolation for pre_order_entries select" ON pre_order_entries;
CREATE POLICY "Tenant isolation for pre_order_entries select"
    ON pre_order_entries FOR SELECT
    USING (tenant_id::text = current_setting('app.current_tenant', true));

DROP POLICY IF EXISTS "Tenant isolation for pre_order_entries insert" ON pre_order_entries;
CREATE POLICY "Tenant isolation for pre_order_entries insert"
    ON pre_order_entries FOR INSERT
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

DROP POLICY IF EXISTS "Tenant isolation for pre_order_entries update" ON pre_order_entries;
CREATE POLICY "Tenant isolation for pre_order_entries update"
    ON pre_order_entries FOR UPDATE
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

DROP POLICY IF EXISTS "Tenant isolation for pre_order_entries delete" ON pre_order_entries;
CREATE POLICY "Tenant isolation for pre_order_entries delete"
    ON pre_order_entries FOR DELETE
    USING (tenant_id::text = current_setting('app.current_tenant', true));

-- Fix applied_client_mutations
DROP POLICY IF EXISTS applied_client_mutations_tenant_isolation_policy ON applied_client_mutations;
CREATE POLICY applied_client_mutations_tenant_isolation_policy ON applied_client_mutations FOR ALL
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Fix entity_versions
DROP POLICY IF EXISTS tenant_isolation_entity_versions ON entity_versions;
CREATE POLICY tenant_isolation_entity_versions ON entity_versions FOR ALL
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Fix sync_events
DROP POLICY IF EXISTS tenant_isolation_sync_events ON sync_events;
CREATE POLICY tenant_isolation_sync_events ON sync_events FOR ALL
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Fix conflict_queue
DROP POLICY IF EXISTS tenant_isolation_conflict_queue ON conflict_queue;
CREATE POLICY tenant_isolation_conflict_queue ON conflict_queue FOR ALL
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Fix loyalty_ledgers
DROP POLICY IF EXISTS tenant_isolation_loyalty_ledgers ON loyalty_ledgers;
CREATE POLICY tenant_isolation_loyalty_ledgers ON loyalty_ledgers FOR ALL
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Fix operation_intents
DROP POLICY IF EXISTS tenant_isolation_operation_intents ON operation_intents;
CREATE POLICY tenant_isolation_operation_intents ON operation_intents FOR ALL
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Fix reward_claims
DROP POLICY IF EXISTS tenant_isolation_reward_claims ON reward_claims;
CREATE POLICY tenant_isolation_reward_claims ON reward_claims FOR ALL
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Fix shift_swap_requests
DROP POLICY IF EXISTS tenant_isolation_shift_swap_requests ON shift_swap_requests;
CREATE POLICY tenant_isolation_shift_swap_requests ON shift_swap_requests FOR ALL
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Fix work_tasks
DROP POLICY IF EXISTS work_tasks_tenant_isolation ON work_tasks;
CREATE POLICY work_tasks_tenant_isolation ON work_tasks FOR ALL
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Fix interactive_proposals
DROP POLICY IF EXISTS tenant_isolation_interactive_proposals ON interactive_proposals;
CREATE POLICY tenant_isolation_interactive_proposals ON interactive_proposals FOR ALL
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Fix interactive_proposal_line_items
DROP POLICY IF EXISTS tenant_isolation_interactive_proposal_line_items ON interactive_proposal_line_items;
CREATE POLICY tenant_isolation_interactive_proposal_line_items ON interactive_proposal_line_items FOR ALL
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Fix subscription_plans
DROP POLICY IF EXISTS tenant_isolation_subscription_plans ON subscription_plans;
CREATE POLICY tenant_isolation_subscription_plans ON subscription_plans FOR ALL
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Fix subscribers
DROP POLICY IF EXISTS tenant_isolation_subscribers ON subscribers;
CREATE POLICY tenant_isolation_subscribers ON subscribers FOR ALL
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Fix fulfillment_batches
DROP POLICY IF EXISTS tenant_isolation_fulfillment_batches ON fulfillment_batches;
CREATE POLICY tenant_isolation_fulfillment_batches ON fulfillment_batches FOR ALL
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Fix consolidated_memory
DROP POLICY IF EXISTS tenant_isolation_consolidated_memory ON consolidated_memory;
CREATE POLICY tenant_isolation_consolidated_memory ON consolidated_memory FOR ALL
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Fix quote_line_items
DROP POLICY IF EXISTS tenant_isolation_quote_line_items ON quote_line_items;
CREATE POLICY tenant_isolation_quote_line_items ON quote_line_items FOR ALL
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Fix proposal_line_items
DROP POLICY IF EXISTS tenant_isolation_proposal_line_items ON proposal_line_items;
CREATE POLICY tenant_isolation_proposal_line_items ON proposal_line_items FOR ALL
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Fix job_templates
DROP POLICY IF EXISTS tenant_isolation_job_templates ON job_templates;
CREATE POLICY tenant_isolation_job_templates ON job_templates FOR ALL
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Fix staff_profiles
DROP POLICY IF EXISTS tenant_isolation_staff_profiles ON staff_profiles;
CREATE POLICY tenant_isolation_staff_profiles ON staff_profiles FOR ALL
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Fix appointments
DROP POLICY IF EXISTS tenant_isolation_appointments ON appointments;
CREATE POLICY tenant_isolation_appointments ON appointments FOR ALL
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Fix booking_resources
DROP POLICY IF EXISTS tenant_isolation_booking_resources ON booking_resources;
CREATE POLICY tenant_isolation_booking_resources ON booking_resources FOR ALL
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Fix service_resource_requirements
DROP POLICY IF EXISTS tenant_isolation_service_resource_requirements ON service_resource_requirements;
CREATE POLICY tenant_isolation_service_resource_requirements ON service_resource_requirements FOR ALL
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Fix booking_resource_reservations
DROP POLICY IF EXISTS tenant_isolation_booking_resource_reservations ON booking_resource_reservations;
CREATE POLICY tenant_isolation_booking_resource_reservations ON booking_resource_reservations FOR ALL
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Fix pos_offline_transactions
DROP POLICY IF EXISTS tenant_isolation_pos_offline_transactions ON pos_offline_transactions;
CREATE POLICY tenant_isolation_pos_offline_transactions ON pos_offline_transactions FOR ALL
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Fix task_dependencies
DROP POLICY IF EXISTS tenant_isolation_task_dependencies ON task_dependencies;
CREATE POLICY tenant_isolation_task_dependencies ON task_dependencies FOR ALL
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Fix swarm_tasks
DROP POLICY IF EXISTS tenant_isolation_swarm_tasks ON swarm_tasks;
CREATE POLICY tenant_isolation_swarm_tasks ON swarm_tasks FOR ALL
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Fix bookings
DROP POLICY IF EXISTS tenant_isolation_bookings ON bookings;
CREATE POLICY tenant_isolation_bookings ON bookings FOR ALL
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- +goose Down
-- Intentionally left empty
