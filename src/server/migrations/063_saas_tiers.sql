CREATE TABLE IF NOT EXISTS saas_tiers (
    id VARCHAR(50) PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    price_cents INTEGER NOT NULL,
    max_products INTEGER,
    max_agents INTEGER,
    monthly_action_limit INTEGER,
    storage_limit_mb INTEGER,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS tenant_subscriptions (
    tenant_id VARCHAR(50) PRIMARY KEY,
    tier_id VARCHAR(50) REFERENCES saas_tiers(id),
    stripe_customer_id VARCHAR(100),
    stripe_subscription_id VARCHAR(100),
    status VARCHAR(50),
    current_period_end TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO saas_tiers (id, name, price_cents, max_products, max_agents, monthly_action_limit, storage_limit_mb)
VALUES
    ('free', 'Free', 0, 10, 1, 100, 500),
    ('starter', 'Starter', 900, 100, 3, 1000, 5000),
    ('pro', 'Pro', 2900, NULL, 10, NULL, 50000),
    ('business', 'Business', 7900, NULL, NULL, NULL, NULL)
ON CONFLICT (id) DO NOTHING;
