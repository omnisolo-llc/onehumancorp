-- +goose Up

-- Fix customer_profile to ensure WITH CHECK and app.current_tenant instead of app.current_tenant_id
DROP POLICY IF EXISTS customer_profile_tenant_isolation_policy ON customer_profile;
CREATE POLICY customer_profile_tenant_isolation_policy ON customer_profile FOR ALL
    USING (tenant_id = current_setting('app.current_tenant', true)::uuid)
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true)::uuid);

-- Fix work_item to ensure WITH CHECK and app.current_tenant instead of app.current_tenant_id
DROP POLICY IF EXISTS work_item_tenant_isolation_policy ON work_item;
CREATE POLICY work_item_tenant_isolation_policy ON work_item FOR ALL
    USING (tenant_id = current_setting('app.current_tenant', true)::uuid)
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true)::uuid);

-- Fix agent_draft to ensure WITH CHECK and app.current_tenant instead of app.current_tenant_id
DROP POLICY IF EXISTS agent_draft_tenant_isolation_policy ON agent_draft;
CREATE POLICY agent_draft_tenant_isolation_policy ON agent_draft FOR ALL
    USING (
        EXISTS (
            SELECT 1 FROM work_item WHERE work_item.id = agent_draft.work_item_id AND work_item.tenant_id = current_setting('app.current_tenant', true)::uuid
        )
    )
    WITH CHECK (
        EXISTS (
            SELECT 1 FROM work_item WHERE work_item.id = agent_draft.work_item_id AND work_item.tenant_id = current_setting('app.current_tenant', true)::uuid
        )
    );

-- +goose Down
-- Intentionally blank
