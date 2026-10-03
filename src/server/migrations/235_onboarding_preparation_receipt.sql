-- Protected preparation receipt: user-writable wizard JSON cannot grant launch.
ALTER TABLE onboarding_state ADD COLUMN IF NOT EXISTS preparation_receipt JSONB;
CREATE UNIQUE INDEX IF NOT EXISTS onboarding_one_preparation_per_tenant
ON onboarding_state(tenant_id) WHERE preparation_receipt IS NOT NULL;
