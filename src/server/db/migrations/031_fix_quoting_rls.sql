-- Fix quoting RLS policies that incorrectly referenced ohc.current_tenant

DROP POLICY IF EXISTS tenant_isolation_quotes ON quotes;
CREATE POLICY tenant_isolation_quotes ON quotes USING (tenant_id = current_setting('app.current_tenant', true));

DROP POLICY IF EXISTS tenant_isolation_quote_line_items ON quote_line_items;
CREATE POLICY tenant_isolation_quote_line_items ON quote_line_items USING (
    quote_id IN (SELECT id FROM quotes WHERE tenant_id = current_setting('app.current_tenant', true))
);

DROP POLICY IF EXISTS tenant_isolation_pricing_heuristics ON pricing_heuristics;
CREATE POLICY tenant_isolation_pricing_heuristics ON pricing_heuristics USING (tenant_id = current_setting('app.current_tenant', true));
