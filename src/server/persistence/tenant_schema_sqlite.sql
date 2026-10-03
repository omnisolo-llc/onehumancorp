CREATE TABLE IF NOT EXISTS tenants (
    id TEXT PRIMARY KEY,
    owner_id TEXT,
    name TEXT,
    tier TEXT DEFAULT 'free',
    plan_tier TEXT DEFAULT 'free',
    has_claimed_trial_extension BOOLEAN DEFAULT FALSE,
    subdomain TEXT,
    default_currency TEXT DEFAULT 'USD',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    is_subscribable BOOLEAN DEFAULT FALSE,
    subscription_frequency TEXT,
    subscription_discount_percent INTEGER DEFAULT 0,
    _sync_status TEXT DEFAULT 'pending',
    version INTEGER DEFAULT 1
);
