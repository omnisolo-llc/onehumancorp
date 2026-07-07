ALTER TABLE IF EXISTS checkout_sessions ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_checkout_sessions ON checkout_sessions
    FOR ALL
    USING (tenant_id = current_setting('app.current_tenant_id', true));

ALTER TABLE IF EXISTS ledger_reserves ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_ledger_reserves ON ledger_reserves
    FOR ALL
    USING (tenant_id = current_setting('app.current_tenant_id', true));

ALTER TABLE IF EXISTS loyalty_ledgers ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_loyalty_ledgers ON loyalty_ledgers
    FOR ALL
    USING (tenant_id = current_setting('app.current_tenant_id', true));

ALTER TABLE IF EXISTS shift_swap_requests ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_shift_swap_requests ON shift_swap_requests
    FOR ALL
    USING (tenant_id = current_setting('app.current_tenant_id', true));

ALTER TABLE IF EXISTS work_intents ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_work_intents ON work_intents
    FOR ALL
    USING (tenant_id = current_setting('app.current_tenant_id', true));
