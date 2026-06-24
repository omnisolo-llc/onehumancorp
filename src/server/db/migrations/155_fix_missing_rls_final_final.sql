-- +goose Up

ALTER TABLE interactive_proposal_line_items ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_interactive_proposal_line_items ON interactive_proposal_line_items;
CREATE POLICY tenant_isolation_interactive_proposal_line_items ON interactive_proposal_line_items
    USING (proposal_id IN (SELECT id FROM interactive_proposals WHERE tenant_id::text = current_setting('app.current_tenant', true)))
    WITH CHECK (proposal_id IN (SELECT id FROM interactive_proposals WHERE tenant_id::text = current_setting('app.current_tenant', true)));

ALTER TABLE quote_line_items ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_quote_line_items ON quote_line_items;
CREATE POLICY tenant_isolation_quote_line_items ON quote_line_items
    USING (quote_id IN (SELECT id FROM quotes WHERE tenant_id::text = current_setting('app.current_tenant', true)))
    WITH CHECK (quote_id IN (SELECT id FROM quotes WHERE tenant_id::text = current_setting('app.current_tenant', true)));

ALTER TABLE proposal_line_items ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_proposal_line_items ON proposal_line_items;
CREATE POLICY tenant_isolation_proposal_line_items ON proposal_line_items
    USING (proposal_id IN (SELECT id FROM proposals WHERE tenant_id::text = current_setting('app.current_tenant', true)))
    WITH CHECK (proposal_id IN (SELECT id FROM proposals WHERE tenant_id::text = current_setting('app.current_tenant', true)));

ALTER TABLE shared_task_dependencies ADD COLUMN IF NOT EXISTS organization_id TEXT;
CREATE INDEX IF NOT EXISTS idx_shared_task_dependencies_org_id ON shared_task_dependencies(organization_id);
ALTER TABLE shared_task_dependencies ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_shared_task_dependencies ON shared_task_dependencies;
CREATE POLICY tenant_isolation_shared_task_dependencies ON shared_task_dependencies
    USING (organization_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (organization_id::text = current_setting('app.current_tenant', true));

ALTER TABLE delivery_zones ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_delivery_zones ON delivery_zones;
CREATE POLICY tenant_isolation_delivery_zones ON delivery_zones
    USING (organization_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (organization_id::text = current_setting('app.current_tenant', true));

ALTER TABLE route_plans ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_route_plans ON route_plans;
CREATE POLICY tenant_isolation_route_plans ON route_plans
    USING (organization_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (organization_id::text = current_setting('app.current_tenant', true));

ALTER TABLE delivery_tasks ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_delivery_tasks ON delivery_tasks;
CREATE POLICY tenant_isolation_delivery_tasks ON delivery_tasks
    USING (organization_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (organization_id::text = current_setting('app.current_tenant', true));

-- +goose Down
