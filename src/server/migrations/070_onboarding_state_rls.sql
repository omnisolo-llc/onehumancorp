
ALTER TABLE onboarding_state ENABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_onboarding_state ON onboarding_state;
DROP POLICY IF EXISTS tenant_isolation_onboarding_state_strict ON onboarding_state;

CREATE POLICY tenant_isolation_onboarding_state_strict ON onboarding_state USING (organization_id::text = current_setting('app.current_tenant', true));
