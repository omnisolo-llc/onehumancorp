CREATE TABLE IF NOT EXISTS pricing_plans (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    monthly_price_usd DECIMAL NOT NULL,
    max_products INTEGER,
    max_ai_actions INTEGER,
    features JSONB DEFAULT '[]',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO pricing_plans (id, name, monthly_price_usd, max_products, max_ai_actions, features)
VALUES
('Free', 'Free', 0.0, 10, 100, '["10 products", "100 AI actions", "OHC Subdomain"]'),
('Starter', 'Starter', 9.0, -1, 1000, '["Unlimited products", "1,000 AI actions", "Custom domain"]'),
('Pro', 'Pro', 29.0, -1, -1, '["Unlimited products", "Unlimited AI actions", "Custom domain"]'),
('Business', 'Business', 79.0, -1, -1, '["Unlimited products", "Unlimited AI actions", "Multi-domain"]')
ON CONFLICT (id) DO NOTHING;
