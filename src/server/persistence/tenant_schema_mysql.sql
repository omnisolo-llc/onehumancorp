CREATE TABLE IF NOT EXISTS tenants (
    id VARCHAR(255) PRIMARY KEY,
    owner_id VARCHAR(255),
    name VARCHAR(255),
    tier VARCHAR(64) NOT NULL DEFAULT 'free',
    plan_tier VARCHAR(64) NOT NULL DEFAULT 'free',
    has_claimed_trial_extension BOOLEAN NOT NULL DEFAULT FALSE,
    subdomain VARCHAR(255),
    default_currency VARCHAR(16) NOT NULL DEFAULT 'USD',
    base_currency VARCHAR(16) NOT NULL DEFAULT 'USD',
    enabled_currencies JSON NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);
