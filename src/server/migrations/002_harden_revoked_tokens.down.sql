-- Down migration for RLS on revoked_tokens
ALTER TABLE revoked_tokens DISABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_revoked_tokens ON revoked_tokens;
