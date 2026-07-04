-- +goose Up

-- 024_a_interactive_quoting.sql - quote_line_items
ALTER TABLE quote_line_items ADD COLUMN IF NOT EXISTS tenant_id TEXT;
UPDATE quote_line_items SET tenant_id = q.tenant_id FROM quotes q WHERE quote_line_items.quote_id = q.id AND quote_line_items.tenant_id IS NULL;
UPDATE quote_line_items SET tenant_id = 'default_tenant' WHERE tenant_id IS NULL;
ALTER TABLE quote_line_items ALTER COLUMN tenant_id SET NOT NULL;
ALTER TABLE IF EXISTS quote_line_items ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_quote_line_items ON quote_line_items;
CREATE POLICY tenant_isolation_quote_line_items ON quote_line_items USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));


-- 138_a_autonomous_proposals.sql & 138_c_proposals.sql - proposal_line_items
ALTER TABLE proposal_line_items ADD COLUMN IF NOT EXISTS tenant_id TEXT;
UPDATE proposal_line_items SET tenant_id = p.tenant_id FROM proposals p WHERE proposal_line_items.proposal_id::text = p.id::text AND proposal_line_items.tenant_id IS NULL;
UPDATE proposal_line_items SET tenant_id = 'default_tenant' WHERE tenant_id IS NULL;
ALTER TABLE proposal_line_items ALTER COLUMN tenant_id SET NOT NULL;
ALTER TABLE IF EXISTS proposal_line_items ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_proposal_line_items ON proposal_line_items;
CREATE POLICY tenant_isolation_proposal_line_items ON proposal_line_items USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));


-- 146_interactive_proposals.sql - interactive_proposal_line_items
ALTER TABLE interactive_proposal_line_items ADD COLUMN IF NOT EXISTS tenant_id TEXT;
UPDATE interactive_proposal_line_items SET tenant_id = p.tenant_id FROM interactive_proposals p WHERE interactive_proposal_line_items.proposal_id = p.id AND interactive_proposal_line_items.tenant_id IS NULL;
UPDATE interactive_proposal_line_items SET tenant_id = 'default_tenant' WHERE tenant_id IS NULL;
ALTER TABLE interactive_proposal_line_items ALTER COLUMN tenant_id SET NOT NULL;
ALTER TABLE IF EXISTS interactive_proposal_line_items ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_interactive_proposal_line_items ON interactive_proposal_line_items;
CREATE POLICY tenant_isolation_interactive_proposal_line_items ON interactive_proposal_line_items USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));


-- +goose Down
-- Dropping policies to revert to previous state where they relied on parent relationships
ALTER TABLE IF EXISTS quote_line_items ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_quote_line_items ON quote_line_items;
CREATE POLICY tenant_isolation_quote_line_items ON quote_line_items USING (
    quote_id IN (SELECT id FROM quotes WHERE tenant_id = current_setting('app.current_tenant', true))
) WITH CHECK (
    quote_id IN (SELECT id FROM quotes WHERE tenant_id = current_setting('app.current_tenant', true))
);
ALTER TABLE quote_line_items DROP COLUMN IF EXISTS tenant_id;


ALTER TABLE IF EXISTS proposal_line_items ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_proposal_line_items ON proposal_line_items;
CREATE POLICY tenant_isolation_proposal_line_items ON proposal_line_items USING (
    proposal_id::text IN (SELECT id::text FROM proposals WHERE tenant_id::text = current_setting('app.current_tenant', true))
) WITH CHECK (
    proposal_id::text IN (SELECT id::text FROM proposals WHERE tenant_id::text = current_setting('app.current_tenant', true))
);
ALTER TABLE proposal_line_items DROP COLUMN IF EXISTS tenant_id;


ALTER TABLE IF EXISTS interactive_proposal_line_items ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_interactive_proposal_line_items ON interactive_proposal_line_items;
CREATE POLICY tenant_isolation_interactive_proposal_line_items ON interactive_proposal_line_items USING (
    proposal_id IN (SELECT id FROM interactive_proposals WHERE tenant_id = current_setting('app.current_tenant', true))
) WITH CHECK (
    proposal_id IN (SELECT id FROM interactive_proposals WHERE tenant_id = current_setting('app.current_tenant', true))
);
ALTER TABLE interactive_proposal_line_items DROP COLUMN IF EXISTS tenant_id;
