-- +goose Up
-- Migration 217: Ensure user authentication columns and TEXT id format exist regardless of table creation order
ALTER TABLE IF EXISTS user_sessions DROP CONSTRAINT IF EXISTS fk_rails_9fa262d742;

CREATE TABLE IF NOT EXISTS users (
    id TEXT PRIMARY KEY
);

ALTER TABLE users ALTER COLUMN id TYPE TEXT USING id::text;

ALTER TABLE users ADD COLUMN IF NOT EXISTS username TEXT UNIQUE;
ALTER TABLE users ADD COLUMN IF NOT EXISTS password_hash TEXT DEFAULT '';
ALTER TABLE users ADD COLUMN IF NOT EXISTS active BOOLEAN DEFAULT TRUE;
ALTER TABLE users ADD COLUMN IF NOT EXISTS tenant_id TEXT DEFAULT '';
ALTER TABLE users ADD COLUMN IF NOT EXISTS oidc_subject TEXT UNIQUE;
ALTER TABLE users ADD COLUMN IF NOT EXISTS roles TEXT[] DEFAULT '{}';

CREATE TABLE IF NOT EXISTS user_sessions (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    tenant_id TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    expires_at TIMESTAMPTZ NOT NULL
);

ALTER TABLE user_sessions ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS user_sessions_tenant_isolation_policy ON user_sessions;
CREATE POLICY user_sessions_tenant_isolation_policy ON user_sessions FOR ALL USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- +goose Down
DROP TABLE IF EXISTS user_sessions CASCADE;
