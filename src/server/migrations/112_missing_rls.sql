-- Add missing RLS to tables

-- customer_timeline
ALTER TABLE IF EXISTS customer_timeline ENABLE ROW LEVEL SECURITY;
DO $$ BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_policies WHERE policyname = 'tenant_isolation_customer_timeline') THEN
        CREATE POLICY tenant_isolation_customer_timeline ON customer_timeline FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::text);
    END IF;
END $$;

-- agent_actions
ALTER TABLE IF EXISTS agent_actions ENABLE ROW LEVEL SECURITY;
DO $$ BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_policies WHERE policyname = 'tenant_isolation_agent_actions') THEN
        CREATE POLICY tenant_isolation_agent_actions ON agent_actions FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::text);
    END IF;
END $$;

-- raw_materials
ALTER TABLE IF EXISTS raw_materials ENABLE ROW LEVEL SECURITY;
DO $$ BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_policies WHERE policyname = 'tenant_isolation_raw_materials') THEN
        CREATE POLICY tenant_isolation_raw_materials ON raw_materials FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::text);
    END IF;
END $$;

-- bom_items
ALTER TABLE IF EXISTS bom_items ENABLE ROW LEVEL SECURITY;
DO $$ BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_policies WHERE policyname = 'tenant_isolation_bom_items') THEN
        CREATE POLICY tenant_isolation_bom_items ON bom_items FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::text);
    END IF;
END $$;

-- ai_memories
ALTER TABLE IF EXISTS ai_memories ENABLE ROW LEVEL SECURITY;
DO $$ BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_policies WHERE policyname = 'tenant_isolation_ai_memories') THEN
        CREATE POLICY tenant_isolation_ai_memories ON ai_memories FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::text);
    END IF;
END $$;

-- po_line_items
ALTER TABLE IF EXISTS po_line_items ENABLE ROW LEVEL SECURITY;
DO $$ BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_policies WHERE policyname = 'tenant_isolation_po_line_items') THEN
        CREATE POLICY tenant_isolation_po_line_items ON po_line_items FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::text);
    END IF;
END $$;

-- order_line_items
ALTER TABLE IF EXISTS order_line_items ENABLE ROW LEVEL SECURITY;
DO $$ BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_policies WHERE policyname = 'tenant_isolation_order_line_items') THEN
        CREATE POLICY tenant_isolation_order_line_items ON order_line_items FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::text);
    END IF;
END $$;

-- depletion_logs
ALTER TABLE IF EXISTS depletion_logs ENABLE ROW LEVEL SECURITY;
DO $$ BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_policies WHERE policyname = 'tenant_isolation_depletion_logs') THEN
        CREATE POLICY tenant_isolation_depletion_logs ON depletion_logs FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::text);
    END IF;
END $$;

-- interactions
ALTER TABLE IF EXISTS interactions ENABLE ROW LEVEL SECURITY;
DO $$ BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_policies WHERE policyname = 'tenant_isolation_interactions') THEN
        CREATE POLICY tenant_isolation_interactions ON interactions FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::text);
    END IF;
END $$;
