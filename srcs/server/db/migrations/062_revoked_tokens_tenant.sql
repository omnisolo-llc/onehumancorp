-- Add organization_id column to revoked_tokens for tenant isolation
ALTER TABLE revoked_tokens ADD COLUMN organization_id TEXT NOT NULL DEFAULT '';

-- Drop the old primary key constraint
ALTER TABLE revoked_tokens DROP CONSTRAINT IF EXISTS revoked_tokens_pkey;

-- Try to drop any unique constraint that might have been automatically created on jti
ALTER TABLE revoked_tokens DROP CONSTRAINT IF EXISTS revoked_tokens_jti_key;

-- Add a new composite primary key
ALTER TABLE revoked_tokens ADD PRIMARY KEY (organization_id, jti);
