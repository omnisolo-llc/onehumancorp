-- +goose Up

-- Fix missing RLS policies on tables that have tenant_id

-- 1. interactive_proposals
ALTER TABLE IF EXISTS interactive_proposals ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_interactive_proposals ON interactive_proposals;
CREATE POLICY tenant_isolation_interactive_proposals ON interactive_proposals USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- 2. interactive_proposal_line_items
ALTER TABLE IF EXISTS interactive_proposal_line_items ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_interactive_proposal_line_items ON interactive_proposal_line_items;
CREATE POLICY tenant_isolation_interactive_proposal_line_items ON interactive_proposal_line_items USING (proposal_id IN (SELECT id FROM interactive_proposals WHERE tenant_id = current_setting('app.current_tenant', true))) WITH CHECK (proposal_id IN (SELECT id FROM interactive_proposals WHERE tenant_id = current_setting('app.current_tenant', true)));

-- 3. subscription_plans
ALTER TABLE IF EXISTS subscription_plans ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_subscription_plans ON subscription_plans;
CREATE POLICY tenant_isolation_subscription_plans ON subscription_plans USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- 4. subscribers
ALTER TABLE IF EXISTS subscribers ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_subscribers ON subscribers;
CREATE POLICY tenant_isolation_subscribers ON subscribers USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- 5. fulfillment_batches
ALTER TABLE IF EXISTS fulfillment_batches ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_fulfillment_batches ON fulfillment_batches;
CREATE POLICY tenant_isolation_fulfillment_batches ON fulfillment_batches USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- 6. unified_threads
ALTER TABLE IF EXISTS unified_threads ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_unified_threads ON unified_threads;
CREATE POLICY tenant_isolation_unified_threads ON unified_threads USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- 7. unified_messages
ALTER TABLE IF EXISTS unified_messages ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_unified_messages ON unified_messages;
CREATE POLICY tenant_isolation_unified_messages ON unified_messages USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- 8. unified_triage_actions
ALTER TABLE IF EXISTS unified_triage_actions ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_unified_triage_actions ON unified_triage_actions;
CREATE POLICY tenant_isolation_unified_triage_actions ON unified_triage_actions USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- +goose Down
-- Revert RLS

DROP POLICY IF EXISTS tenant_isolation_interactive_proposals ON interactive_proposals;
ALTER TABLE IF EXISTS interactive_proposals DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_interactive_proposal_line_items ON interactive_proposal_line_items;
ALTER TABLE IF EXISTS interactive_proposal_line_items DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_subscription_plans ON subscription_plans;
ALTER TABLE IF EXISTS subscription_plans DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_subscribers ON subscribers;
ALTER TABLE IF EXISTS subscribers DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_fulfillment_batches ON fulfillment_batches;
ALTER TABLE IF EXISTS fulfillment_batches DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_unified_threads ON unified_threads;
ALTER TABLE IF EXISTS unified_threads DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_unified_messages ON unified_messages;
ALTER TABLE IF EXISTS unified_messages DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_unified_triage_actions ON unified_triage_actions;
ALTER TABLE IF EXISTS unified_triage_actions DISABLE ROW LEVEL SECURITY;
