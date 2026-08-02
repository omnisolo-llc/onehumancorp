-- Migration 217: Ensure user authentication columns exist regardless of table creation order
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_tables WHERE tablename = 'user_sessions') THEN
        ALTER TABLE user_sessions DROP CONSTRAINT IF EXISTS fk_rails_9fa262d742;
    END IF;
END $$;

DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_tables WHERE tablename = 'users') THEN
        ALTER TABLE users ADD COLUMN IF NOT EXISTS username TEXT;
        ALTER TABLE users ADD COLUMN IF NOT EXISTS password_hash TEXT DEFAULT '';
        ALTER TABLE users ADD COLUMN IF NOT EXISTS active BOOLEAN DEFAULT TRUE;
        ALTER TABLE users ADD COLUMN IF NOT EXISTS tenant_id TEXT DEFAULT '';
        ALTER TABLE users ADD COLUMN IF NOT EXISTS oidc_subject TEXT;
        ALTER TABLE users ADD COLUMN IF NOT EXISTS roles TEXT[] DEFAULT '{}';
    END IF;
END $$;
