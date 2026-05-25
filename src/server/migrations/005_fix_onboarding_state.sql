ALTER TABLE onboarding_state ADD COLUMN IF NOT EXISTS organization_id TEXT NOT NULL DEFAULT 'default';
ALTER TABLE onboarding_state DROP CONSTRAINT IF EXISTS onboarding_state_pkey;
ALTER TABLE onboarding_state ADD PRIMARY KEY (tenant_id, organization_id, user_id);
