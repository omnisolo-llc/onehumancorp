-- +goose Up

DO $$
BEGIN
    IF to_regclass('agent_actions') IS NOT NULL THEN
        ALTER TABLE agent_actions ENABLE ROW LEVEL SECURITY;
        IF NOT EXISTS (SELECT 1 FROM pg_policies WHERE tablename = 'agent_actions' AND policyname = 'tenant_isolation_agent_actions') THEN
            CREATE POLICY tenant_isolation_agent_actions ON agent_actions USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
        END IF;
    END IF;

    IF to_regclass('agent_session_data') IS NOT NULL THEN
        ALTER TABLE agent_session_data ENABLE ROW LEVEL SECURITY;
        IF NOT EXISTS (SELECT 1 FROM pg_policies WHERE tablename = 'agent_session_data' AND policyname = 'tenant_isolation_agent_session_data') THEN
            CREATE POLICY tenant_isolation_agent_session_data ON agent_session_data USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
        END IF;
    END IF;

    IF to_regclass('ai_memories') IS NOT NULL THEN
        ALTER TABLE ai_memories ENABLE ROW LEVEL SECURITY;
        IF NOT EXISTS (SELECT 1 FROM pg_policies WHERE tablename = 'ai_memories' AND policyname = 'tenant_isolation_ai_memories') THEN
            CREATE POLICY tenant_isolation_ai_memories ON ai_memories USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
        END IF;
    END IF;

    IF to_regclass('bom_items') IS NOT NULL THEN
        ALTER TABLE bom_items ENABLE ROW LEVEL SECURITY;
        IF NOT EXISTS (SELECT 1 FROM pg_policies WHERE tablename = 'bom_items' AND policyname = 'tenant_isolation_bom_items') THEN
            CREATE POLICY tenant_isolation_bom_items ON bom_items USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
        END IF;
    END IF;

    IF to_regclass('customer_timeline') IS NOT NULL THEN
        ALTER TABLE customer_timeline ENABLE ROW LEVEL SECURITY;
        IF NOT EXISTS (SELECT 1 FROM pg_policies WHERE tablename = 'customer_timeline' AND policyname = 'tenant_isolation_customer_timeline') THEN
            CREATE POLICY tenant_isolation_customer_timeline ON customer_timeline USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
        END IF;
    END IF;

    IF to_regclass('depletion_logs') IS NOT NULL THEN
        ALTER TABLE depletion_logs ENABLE ROW LEVEL SECURITY;
        IF NOT EXISTS (SELECT 1 FROM pg_policies WHERE tablename = 'depletion_logs' AND policyname = 'tenant_isolation_depletion_logs') THEN
            CREATE POLICY tenant_isolation_depletion_logs ON depletion_logs USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
        END IF;
    END IF;

    IF to_regclass('interactions') IS NOT NULL THEN
        ALTER TABLE interactions ENABLE ROW LEVEL SECURITY;
        IF NOT EXISTS (SELECT 1 FROM pg_policies WHERE tablename = 'interactions' AND policyname = 'tenant_isolation_interactions') THEN
            CREATE POLICY tenant_isolation_interactions ON interactions USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
        END IF;
    END IF;

    IF to_regclass('order_line_items') IS NOT NULL THEN
        ALTER TABLE order_line_items ENABLE ROW LEVEL SECURITY;
        IF NOT EXISTS (SELECT 1 FROM pg_policies WHERE tablename = 'order_line_items' AND policyname = 'tenant_isolation_order_line_items') THEN
            CREATE POLICY tenant_isolation_order_line_items ON order_line_items USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
        END IF;
    END IF;

    IF to_regclass('po_line_items') IS NOT NULL THEN
        ALTER TABLE po_line_items ENABLE ROW LEVEL SECURITY;
        IF NOT EXISTS (SELECT 1 FROM pg_policies WHERE tablename = 'po_line_items' AND policyname = 'tenant_isolation_po_line_items') THEN
            CREATE POLICY tenant_isolation_po_line_items ON po_line_items USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
        END IF;
    END IF;

    IF to_regclass('raw_materials') IS NOT NULL THEN
        ALTER TABLE raw_materials ENABLE ROW LEVEL SECURITY;
        IF NOT EXISTS (SELECT 1 FROM pg_policies WHERE tablename = 'raw_materials' AND policyname = 'tenant_isolation_raw_materials') THEN
            CREATE POLICY tenant_isolation_raw_materials ON raw_materials USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
        END IF;
    END IF;

    IF to_regclass('services') IS NOT NULL THEN
        ALTER TABLE services ENABLE ROW LEVEL SECURITY;
        IF NOT EXISTS (SELECT 1 FROM pg_policies WHERE tablename = 'services' AND policyname = 'tenant_isolation_services') THEN
            CREATE POLICY tenant_isolation_services ON services USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
        END IF;
    END IF;

    IF to_regclass('swarm_tasks') IS NOT NULL THEN
        ALTER TABLE swarm_tasks ENABLE ROW LEVEL SECURITY;
        IF NOT EXISTS (SELECT 1 FROM pg_policies WHERE tablename = 'swarm_tasks' AND policyname = 'tenant_isolation_swarm_tasks') THEN
            CREATE POLICY tenant_isolation_swarm_tasks ON swarm_tasks USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
        END IF;
    END IF;

    IF to_regclass('task_dependencies') IS NOT NULL THEN
        ALTER TABLE task_dependencies ENABLE ROW LEVEL SECURITY;
        IF NOT EXISTS (SELECT 1 FROM pg_policies WHERE tablename = 'task_dependencies' AND policyname = 'tenant_isolation_task_dependencies') THEN
            CREATE POLICY tenant_isolation_task_dependencies ON task_dependencies USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
        END IF;
    END IF;

    IF to_regclass('task_envelopes') IS NOT NULL THEN
        ALTER TABLE task_envelopes ENABLE ROW LEVEL SECURITY;
        IF NOT EXISTS (SELECT 1 FROM pg_policies WHERE tablename = 'task_envelopes' AND policyname = 'tenant_isolation_task_envelopes') THEN
            CREATE POLICY tenant_isolation_task_envelopes ON task_envelopes USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
        END IF;
    END IF;
END $$;

-- +goose Down
