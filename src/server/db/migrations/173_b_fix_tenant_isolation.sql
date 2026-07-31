-- +goose Up

-- Fix loyalty_ledgers RLS policy to use app.current_tenant
DROP POLICY IF EXISTS tenant_isolation_loyalty_ledgers ON loyalty_ledgers;
CREATE POLICY tenant_isolation_loyalty_ledgers ON loyalty_ledgers
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Fix operation_intents RLS policy to have WITH CHECK
DROP POLICY IF EXISTS tenant_isolation_operation_intents ON operation_intents;
CREATE POLICY tenant_isolation_operation_intents ON operation_intents
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Fix reward_claims RLS policy to use app.current_tenant
DROP POLICY IF EXISTS tenant_isolation_reward_claims ON reward_claims;
CREATE POLICY tenant_isolation_reward_claims ON reward_claims
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Fix shift_swap_requests RLS policy to use app.current_tenant
DROP POLICY IF EXISTS tenant_isolation_shift_swap_requests ON shift_swap_requests;
CREATE POLICY tenant_isolation_shift_swap_requests ON shift_swap_requests
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Fix agent_draft to ensure WITH CHECK
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

-- Fix customer_profile to ensure WITH CHECK
DROP POLICY IF EXISTS customer_profile_tenant_isolation_policy ON customer_profile;
CREATE POLICY customer_profile_tenant_isolation_policy ON customer_profile FOR ALL
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Fix work_item to ensure WITH CHECK
DROP POLICY IF EXISTS work_item_tenant_isolation_policy ON work_item;
CREATE POLICY work_item_tenant_isolation_policy ON work_item FOR ALL
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Fix proposed_bookings to ensure WITH CHECK
DROP POLICY IF EXISTS proposed_bookings_tenant_isolation ON proposed_bookings;
CREATE POLICY proposed_bookings_tenant_isolation ON proposed_bookings FOR ALL
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Fix work_tasks to ensure WITH CHECK
DROP POLICY IF EXISTS work_tasks_tenant_isolation ON work_tasks;
CREATE POLICY work_tasks_tenant_isolation ON work_tasks FOR ALL
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- +goose Down
-- Intentionally empty
