-- Remove global unique constraints that violate multi-tenant isolation
ALTER TABLE users DROP CONSTRAINT IF EXISTS users_username_key;
ALTER TABLE users DROP CONSTRAINT IF EXISTS users_email_key;
ALTER TABLE users DROP CONSTRAINT IF EXISTS users_oidc_subject_key;

-- In SQLite (modernc/sqlite), dropping constraints directly using ALTER TABLE ... DROP CONSTRAINT is not fully supported or is complex.
-- We will use index manipulation for SQLite compatibility, while keeping the Postgres constraints dropped if they exist.
DROP INDEX IF EXISTS idx_users_username;
DROP INDEX IF EXISTS idx_users_email;
DROP INDEX IF EXISTS idx_users_oidc;

-- Add composite unique constraints scoped to the organization
CREATE UNIQUE INDEX IF NOT EXISTS idx_users_org_username ON users(organization_id, username);
CREATE UNIQUE INDEX IF NOT EXISTS idx_users_org_email ON users(organization_id, email);
CREATE UNIQUE INDEX IF NOT EXISTS idx_users_org_oidc ON users(organization_id, oidc_subject) WHERE oidc_subject IS NOT NULL;
