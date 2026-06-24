-- +goose Up
-- Apply missing RLS to tables identified by check_rls_all2.sh

-- ai_memories
ALTER TABLE IF EXISTS ai_memories ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ai_memories ON ai_memories;
CREATE POLICY tenant_isolation_ai_memories ON ai_memories USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- agent_actions
ALTER TABLE IF EXISTS agent_actions ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_agent_actions ON agent_actions;
CREATE POLICY tenant_isolation_agent_actions ON agent_actions USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- bom_items
ALTER TABLE IF EXISTS bom_items ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_bom_items ON bom_items;
CREATE POLICY tenant_isolation_bom_items ON bom_items USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- customer_timeline
ALTER TABLE IF EXISTS customer_timeline ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_customer_timeline ON customer_timeline;
CREATE POLICY tenant_isolation_customer_timeline ON customer_timeline USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- depletion_logs
ALTER TABLE IF EXISTS depletion_logs ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_depletion_logs ON depletion_logs;
CREATE POLICY tenant_isolation_depletion_logs ON depletion_logs USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- interactions
ALTER TABLE IF EXISTS interactions ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_interactions ON interactions;
CREATE POLICY tenant_isolation_interactions ON interactions USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- order_line_items
ALTER TABLE IF EXISTS order_line_items ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_order_line_items ON order_line_items;
CREATE POLICY tenant_isolation_order_line_items ON order_line_items USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- po_line_items
ALTER TABLE IF EXISTS po_line_items ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_po_line_items ON po_line_items;
CREATE POLICY tenant_isolation_po_line_items ON po_line_items USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- raw_materials
ALTER TABLE IF EXISTS raw_materials ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_raw_materials ON raw_materials;
CREATE POLICY tenant_isolation_raw_materials ON raw_materials USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- services
ALTER TABLE IF EXISTS services ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_services ON services;
CREATE POLICY tenant_isolation_services ON services USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- +goose Down
-- Reverting RLS changes is potentially destructive, omitting or disable if strictly needed.
