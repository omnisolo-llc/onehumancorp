-- +goose Up

DO $$
BEGIN
    ALTER TABLE IF EXISTS agent_actions ENABLE ROW LEVEL SECURITY;
    IF NOT EXISTS (SELECT 1 FROM pg_policies WHERE tablename = 'agent_actions' AND policyname = 'tenant_isolation_agent_actions') THEN
        CREATE POLICY tenant_isolation_agent_actions ON agent_actions USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;

    ALTER TABLE IF EXISTS agent_session_data ENABLE ROW LEVEL SECURITY;
    IF NOT EXISTS (SELECT 1 FROM pg_policies WHERE tablename = 'agent_session_data' AND policyname = 'tenant_isolation_agent_session_data') THEN
        CREATE POLICY tenant_isolation_agent_session_data ON agent_session_data USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;

    ALTER TABLE IF EXISTS ai_memories ENABLE ROW LEVEL SECURITY;
    IF NOT EXISTS (SELECT 1 FROM pg_policies WHERE tablename = 'ai_memories' AND policyname = 'tenant_isolation_ai_memories') THEN
        CREATE POLICY tenant_isolation_ai_memories ON ai_memories USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;

    ALTER TABLE IF EXISTS bom_items ENABLE ROW LEVEL SECURITY;
    IF NOT EXISTS (SELECT 1 FROM pg_policies WHERE tablename = 'bom_items' AND policyname = 'tenant_isolation_bom_items') THEN
        CREATE POLICY tenant_isolation_bom_items ON bom_items USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;

    ALTER TABLE IF EXISTS customer_timeline ENABLE ROW LEVEL SECURITY;
    IF NOT EXISTS (SELECT 1 FROM pg_policies WHERE tablename = 'customer_timeline' AND policyname = 'tenant_isolation_customer_timeline') THEN
        CREATE POLICY tenant_isolation_customer_timeline ON customer_timeline USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;

    ALTER TABLE IF EXISTS depletion_logs ENABLE ROW LEVEL SECURITY;
    IF NOT EXISTS (SELECT 1 FROM pg_policies WHERE tablename = 'depletion_logs' AND policyname = 'tenant_isolation_depletion_logs') THEN
        CREATE POLICY tenant_isolation_depletion_logs ON depletion_logs USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;

    ALTER TABLE IF EXISTS interactions ENABLE ROW LEVEL SECURITY;
    IF NOT EXISTS (SELECT 1 FROM pg_policies WHERE tablename = 'interactions' AND policyname = 'tenant_isolation_interactions') THEN
        CREATE POLICY tenant_isolation_interactions ON interactions USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;

    ALTER TABLE IF EXISTS order_line_items ENABLE ROW LEVEL SECURITY;
    IF NOT EXISTS (SELECT 1 FROM pg_policies WHERE tablename = 'order_line_items' AND policyname = 'tenant_isolation_order_line_items') THEN
        CREATE POLICY tenant_isolation_order_line_items ON order_line_items USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;

    ALTER TABLE IF EXISTS po_line_items ENABLE ROW LEVEL SECURITY;
    IF NOT EXISTS (SELECT 1 FROM pg_policies WHERE tablename = 'po_line_items' AND policyname = 'tenant_isolation_po_line_items') THEN
        CREATE POLICY tenant_isolation_po_line_items ON po_line_items USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;

    ALTER TABLE IF EXISTS raw_materials ENABLE ROW LEVEL SECURITY;
    IF NOT EXISTS (SELECT 1 FROM pg_policies WHERE tablename = 'raw_materials' AND policyname = 'tenant_isolation_raw_materials') THEN
        CREATE POLICY tenant_isolation_raw_materials ON raw_materials USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;

    ALTER TABLE IF EXISTS services ENABLE ROW LEVEL SECURITY;
    IF NOT EXISTS (SELECT 1 FROM pg_policies WHERE tablename = 'services' AND policyname = 'tenant_isolation_services') THEN
        CREATE POLICY tenant_isolation_services ON services USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;

    ALTER TABLE IF EXISTS swarm_tasks ENABLE ROW LEVEL SECURITY;
    IF NOT EXISTS (SELECT 1 FROM pg_policies WHERE tablename = 'swarm_tasks' AND policyname = 'tenant_isolation_swarm_tasks') THEN
        CREATE POLICY tenant_isolation_swarm_tasks ON swarm_tasks USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;

    ALTER TABLE IF EXISTS task_dependencies ENABLE ROW LEVEL SECURITY;
    IF NOT EXISTS (SELECT 1 FROM pg_policies WHERE tablename = 'task_dependencies' AND policyname = 'tenant_isolation_task_dependencies') THEN
        CREATE POLICY tenant_isolation_task_dependencies ON task_dependencies USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;

    ALTER TABLE IF EXISTS task_envelopes ENABLE ROW LEVEL SECURITY;
    IF NOT EXISTS (SELECT 1 FROM pg_policies WHERE tablename = 'task_envelopes' AND policyname = 'tenant_isolation_task_envelopes') THEN
        CREATE POLICY tenant_isolation_task_envelopes ON task_envelopes USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END $$;

-- +goose Down
