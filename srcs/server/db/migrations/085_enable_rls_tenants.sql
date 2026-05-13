-- +goose postgres
-- +goose StatementBegin
ALTER TABLE tenants ENABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_tenants ON tenants;
CREATE POLICY tenant_isolation_tenants ON tenants
    USING (id::text = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');
-- +goose StatementEnd

-- +goose sqlite3
-- +goose StatementBegin
-- SQLite does not support RLS natively in this way.
SELECT 1;
-- +goose StatementEnd
