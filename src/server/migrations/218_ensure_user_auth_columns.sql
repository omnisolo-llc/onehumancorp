-- Migration 217: Ensure user authentication columns and TEXT id format exist regardless of table creation order
-- ALTER TABLE user_sessions DROP CONSTRAINT IF EXISTS fk_rails_9fa262d742;


ALTER TABLE users ADD COLUMN IF NOT EXISTS username TEXT;
ALTER TABLE users ADD COLUMN IF NOT EXISTS password_hash TEXT DEFAULT '';
ALTER TABLE users ADD COLUMN IF NOT EXISTS active BOOLEAN DEFAULT TRUE;
ALTER TABLE users ADD COLUMN IF NOT EXISTS tenant_id TEXT DEFAULT '';
ALTER TABLE users ADD COLUMN IF NOT EXISTS oidc_subject TEXT;
ALTER TABLE users ADD COLUMN IF NOT EXISTS roles TEXT[] DEFAULT '{}';
