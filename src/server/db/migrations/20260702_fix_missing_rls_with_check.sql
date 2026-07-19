-- +goose Up

-- Fix availability_schedules RLS policy
DO $$
BEGIN
    IF to_regclass('availability_schedules') IS NOT NULL THEN
        DROP POLICY IF EXISTS availability_schedules_tenant_isolation ON availability_schedules;
        DROP POLICY IF EXISTS tenant_isolation_availability_schedules ON availability_schedules;
        CREATE POLICY tenant_isolation_availability_schedules ON availability_schedules FOR ALL
            USING (tenant_id::text = current_setting('app.current_tenant', true))
            WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END $$;

-- Fix calendar_integrations RLS policy
DO $$
BEGIN
    IF to_regclass('calendar_integrations') IS NOT NULL THEN
        DROP POLICY IF EXISTS calendar_integrations_tenant_isolation ON calendar_integrations;
        DROP POLICY IF EXISTS tenant_isolation_calendar_integrations ON calendar_integrations;
        CREATE POLICY tenant_isolation_calendar_integrations ON calendar_integrations FOR ALL
            USING (tenant_id::text = current_setting('app.current_tenant', true))
            WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END $$;

-- Fix customer_profile RLS policy
DO $$
BEGIN
    IF to_regclass('customer_profile') IS NOT NULL THEN
        DROP POLICY IF EXISTS customer_profile_tenant_isolation_policy ON customer_profile;
        DROP POLICY IF EXISTS tenant_isolation_customer_profile ON customer_profile;
        CREATE POLICY tenant_isolation_customer_profile ON customer_profile FOR ALL
            USING (tenant_id::text = current_setting('app.current_tenant', true))
            WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END $$;

-- Fix work_item RLS policy
DO $$
BEGIN
    IF to_regclass('work_item') IS NOT NULL THEN
        DROP POLICY IF EXISTS work_item_tenant_isolation_policy ON work_item;
        DROP POLICY IF EXISTS tenant_isolation_work_item ON work_item;
        CREATE POLICY tenant_isolation_work_item ON work_item FOR ALL
            USING (tenant_id::text = current_setting('app.current_tenant', true))
            WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END $$;

-- Fix agent_draft RLS policy
DO $$
BEGIN
    IF to_regclass('agent_draft') IS NOT NULL THEN
        DROP POLICY IF EXISTS agent_draft_tenant_isolation_policy ON agent_draft;
        DROP POLICY IF EXISTS tenant_isolation_agent_draft ON agent_draft;
        CREATE POLICY tenant_isolation_agent_draft ON agent_draft FOR ALL
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
    END IF;
END $$;

-- +goose Down
-- Intentionally empty
