
CREATE TABLE IF NOT EXISTS onboarding_state (
    organization_id VARCHAR(255) PRIMARY KEY,
    user_id VARCHAR(255) NOT NULL,
    current_step INT NOT NULL DEFAULT 0,
    state_json JSONB NOT NULL DEFAULT '{}',
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
