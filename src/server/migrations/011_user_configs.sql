CREATE TABLE IF NOT EXISTS user_configs (
    spiffe_id VARCHAR PRIMARY KEY,
    config_json TEXT NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    hash VARCHAR NOT NULL
);
ALTER TABLE user_configs ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_user_configs ON user_configs USING (spiffe_id::text LIKE 'spiffe://%/' || current_setting('app.current_tenant', true) || '/%');
