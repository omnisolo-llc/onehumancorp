-- Fix revoked_tokens primary key to include tenant_id to prevent cross-tenant IDOR/DoS
ALTER TABLE revoked_tokens DROP CONSTRAINT IF EXISTS revoked_tokens_pkey;
ALTER TABLE revoked_tokens ADD PRIMARY KEY (jti, tenant_id);
