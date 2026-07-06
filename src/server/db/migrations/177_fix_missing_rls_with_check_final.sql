-- +goose Up

-- Fix operation_intents RLS policy to have WITH CHECK
DROP POLICY IF EXISTS tenant_isolation_operation_intents ON operation_intents;
CREATE POLICY tenant_isolation_operation_intents ON operation_intents
    USING (tenant_id = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- Fix customer_profile RLS policy
DROP POLICY IF EXISTS customer_profile_tenant_isolation_policy ON customer_profile;
CREATE POLICY customer_profile_tenant_isolation_policy ON customer_profile FOR ALL
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Fix work_item RLS policy
DROP POLICY IF EXISTS work_item_tenant_isolation_policy ON work_item;
CREATE POLICY work_item_tenant_isolation_policy ON work_item FOR ALL
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Fix agent_draft RLS policy
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

-- Fix applied_client_mutations RLS policy
DROP POLICY IF EXISTS applied_client_mutations_tenant_isolation_policy ON applied_client_mutations;
CREATE POLICY applied_client_mutations_tenant_isolation_policy ON applied_client_mutations FOR ALL
    USING (tenant_id = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- Fix ohc_shared_offer RLS policy
DROP POLICY IF EXISTS tenant_isolation_ohc_shared_offer ON ohc_shared_offer;
CREATE POLICY tenant_isolation_ohc_shared_offer ON ohc_shared_offer FOR ALL
    USING (originating_tenant_id = current_setting('app.current_tenant', true) OR target_tenant_id = current_setting('app.current_tenant', true))
    WITH CHECK (originating_tenant_id = current_setting('app.current_tenant', true) OR target_tenant_id = current_setting('app.current_tenant', true));

-- Fix entity_versions RLS policy
DROP POLICY IF EXISTS tenant_isolation_entity_versions ON entity_versions;
CREATE POLICY tenant_isolation_entity_versions ON entity_versions FOR ALL
    USING (tenant_id = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- Fix sync_events RLS policy
DROP POLICY IF EXISTS tenant_isolation_sync_events ON sync_events;
CREATE POLICY tenant_isolation_sync_events ON sync_events FOR ALL
    USING (tenant_id = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- Fix conflict_queue RLS policy
DROP POLICY IF EXISTS tenant_isolation_conflict_queue ON conflict_queue;
CREATE POLICY tenant_isolation_conflict_queue ON conflict_queue FOR ALL
    USING (tenant_id = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));


-- +goose Down
-- Reverting operation_intents
DROP POLICY IF EXISTS tenant_isolation_operation_intents ON operation_intents;
CREATE POLICY tenant_isolation_operation_intents ON operation_intents
    USING (tenant_id = current_setting('app.current_tenant', true));

-- Reverting customer_profile
DROP POLICY IF EXISTS customer_profile_tenant_isolation_policy ON customer_profile;
CREATE POLICY customer_profile_tenant_isolation_policy ON customer_profile FOR ALL
    USING (tenant_id::text = current_setting('app.current_tenant', true));

-- Reverting work_item
DROP POLICY IF EXISTS work_item_tenant_isolation_policy ON work_item;
CREATE POLICY work_item_tenant_isolation_policy ON work_item FOR ALL
    USING (tenant_id::text = current_setting('app.current_tenant', true));

-- Reverting agent_draft
DROP POLICY IF EXISTS agent_draft_tenant_isolation_policy ON agent_draft;
CREATE POLICY agent_draft_tenant_isolation_policy ON agent_draft FOR ALL
    USING (
        EXISTS (
            SELECT 1 FROM work_item WHERE work_item.id = agent_draft.work_item_id AND work_item.tenant_id::text = current_setting('app.current_tenant', true)
        )
    );

-- Reverting applied_client_mutations
DROP POLICY IF EXISTS applied_client_mutations_tenant_isolation_policy ON applied_client_mutations;
CREATE POLICY applied_client_mutations_tenant_isolation_policy ON applied_client_mutations
    USING (tenant_id = current_setting('app.current_tenant', true));

-- Reverting ohc_shared_offer
DROP POLICY IF EXISTS tenant_isolation_ohc_shared_offer ON ohc_shared_offer;
CREATE POLICY tenant_isolation_ohc_shared_offer ON ohc_shared_offer
    USING (originating_tenant_id = current_setting('app.current_tenant', true) OR target_tenant_id = current_setting('app.current_tenant', true));

-- Reverting entity_versions
DROP POLICY IF EXISTS tenant_isolation_entity_versions ON entity_versions;
CREATE POLICY tenant_isolation_entity_versions ON entity_versions
    USING (tenant_id = current_setting('app.current_tenant', true));

-- Reverting sync_events
DROP POLICY IF EXISTS tenant_isolation_sync_events ON sync_events;
CREATE POLICY tenant_isolation_sync_events ON sync_events
    USING (tenant_id = current_setting('app.current_tenant', true));

-- Reverting conflict_queue
DROP POLICY IF EXISTS tenant_isolation_conflict_queue ON conflict_queue;
CREATE POLICY tenant_isolation_conflict_queue ON conflict_queue
    USING (tenant_id = current_setting('app.current_tenant', true));
