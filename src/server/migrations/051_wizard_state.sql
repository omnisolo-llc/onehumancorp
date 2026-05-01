CREATE TABLE IF NOT EXISTS wizard_state (
    organization_id VARCHAR(255) NOT NULL,
    state JSONB NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    PRIMARY KEY (organization_id)
);
ALTER TABLE wizard_state ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_wizard_state ON wizard_state USING (
    organization_id = current_setting('app.current_tenant', true)
    OR current_setting('app.current_tenant', true) = 'system'
    OR current_setting('app.current_tenant', true) = ''
);
