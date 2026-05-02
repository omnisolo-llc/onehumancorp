-- 058_onboarding_state.sql

CREATE TABLE IF NOT EXISTS onboarding_state (
    tenant_id VARCHAR(255) NOT NULL,
    organization_id VARCHAR(255) NOT NULL,
    user_id VARCHAR(255) NOT NULL,
    current_step INT NOT NULL DEFAULT 0,
    state_json JSONB NOT NULL DEFAULT '{}',
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (tenant_id, organization_id)
);

ALTER TABLE onboarding_state ENABLE ROW LEVEL SECURITY;
