CREATE TABLE IF NOT EXISTS loyalty_settings (
    tenant_id TEXT PRIMARY KEY,
    point_ratio INTEGER NOT NULL DEFAULT 1,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE loyalty_settings ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_loyalty_settings ON loyalty_settings;
CREATE POLICY tenant_isolation_loyalty_settings
ON loyalty_settings
USING (tenant_id = current_setting('app.current_tenant', true));
