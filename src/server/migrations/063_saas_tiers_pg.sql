-- Postgres specific upgrades
ALTER TABLE saas_tiers ENABLE ROW LEVEL SECURITY;
ALTER TABLE tenant_subscriptions ENABLE ROW LEVEL SECURITY;

CREATE POLICY "Users can view tiers"
    ON saas_tiers FOR SELECT
    USING (true);

CREATE POLICY "Tenants can view own subscription"
    ON tenant_subscriptions FOR SELECT
    USING (tenant_id = current_setting('app.current_tenant', true));
