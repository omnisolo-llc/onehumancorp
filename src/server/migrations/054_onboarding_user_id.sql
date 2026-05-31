-- Ensure onboarding_state works based on tenant_id + user_id instead of organization_id

ALTER TABLE onboarding_state DROP CONSTRAINT IF EXISTS onboarding_state_pkey;
ALTER TABLE onboarding_state ADD PRIMARY KEY (tenant_id, user_id);

ALTER TABLE onboarding_state DROP COLUMN IF EXISTS organization_id;
