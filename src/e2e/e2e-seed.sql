BEGIN;

ALTER TABLE users DISABLE ROW LEVEL SECURITY;
ALTER TABLE products DISABLE ROW LEVEL SECURITY;
ALTER TABLE customers DISABLE ROW LEVEL SECURITY;
ALTER TABLE orders DISABLE ROW LEVEL SECURITY;

INSERT INTO tenants (id, name, industry, tier, has_claimed_trial_extension)
VALUES
  ('e2e-tenant', 'OHC E2E Bakery', 'Food and beverage', 'Starter', false),
  ('e2e-tenant-free', 'OHC E2E Free Bakery', 'Food and beverage', 'Free', false),
  ('e2e-tenant-business', 'OHC E2E Business Bakery', 'Food and beverage', 'Business', false),
  ('e2e-tenant-unlimited', 'OHC E2E Pro Bakery', 'Food and beverage', 'Pro', false)
ON CONFLICT (id) DO UPDATE
SET name = EXCLUDED.name,
    industry = EXCLUDED.industry,
    tier = EXCLUDED.tier,
    has_claimed_trial_extension = EXCLUDED.has_claimed_trial_extension,
    updated_at = CURRENT_TIMESTAMP;

INSERT INTO users (id, username, email, password_hash, roles, active, tenant_id, created_at, updated_at)
VALUES
  (
    'e2e-admin-user',
    'test@example.com',
    'test@example.com',
    '$2b$10$hmVhunI7Fq2ZzQ0PguAH5OeXUyb/gNAORUpLPD2g44Ik9/Fd9sM7a',
    ARRAY['ADMIN'],
    true,
    'e2e-tenant',
    CURRENT_TIMESTAMP,
    CURRENT_TIMESTAMP
  ),
  (
    'e2e-team-member',
    'member@example.com',
    'member@example.com',
    '$2b$10$DO879TauCkftPAQhaF3wt.34Fd4ntX8KrtpeQCoOa43kwLNxKqkLK',
    ARRAY['OPERATOR'],
    true,
    'e2e-tenant',
    CURRENT_TIMESTAMP,
    CURRENT_TIMESTAMP
  ),
  (
    'e2e-free-user',
    'free@example.com',
    'free@example.com',
    '$2b$10$hmVhunI7Fq2ZzQ0PguAH5OeXUyb/gNAORUpLPD2g44Ik9/Fd9sM7a',
    ARRAY['ADMIN'],
    true,
    'e2e-tenant-free',
    CURRENT_TIMESTAMP,
    CURRENT_TIMESTAMP
  ),
  (
    'e2e-business-user',
    'business@example.com',
    'business@example.com',
    '$2b$10$hmVhunI7Fq2ZzQ0PguAH5OeXUyb/gNAORUpLPD2g44Ik9/Fd9sM7a',
    ARRAY['ADMIN'],
    true,
    'e2e-tenant-business',
    CURRENT_TIMESTAMP,
    CURRENT_TIMESTAMP
  ),
  (
    'e2e-unlimited-admin-user',
    'pro@example.com',
    'pro@example.com',
    '$2b$10$hmVhunI7Fq2ZzQ0PguAH5OeXUyb/gNAORUpLPD2g44Ik9/Fd9sM7a',
    ARRAY['ADMIN'],
    true,
    'e2e-tenant-unlimited',
    CURRENT_TIMESTAMP,
    CURRENT_TIMESTAMP
  )
ON CONFLICT (id) DO UPDATE
SET username = EXCLUDED.username,
    email = EXCLUDED.email,
    password_hash = EXCLUDED.password_hash,
    roles = EXCLUDED.roles,
    active = EXCLUDED.active,
    tenant_id = EXCLUDED.tenant_id,
    updated_at = CURRENT_TIMESTAMP;

INSERT INTO products (id, tenant_id, title, description, type, price, price_cents, currency, inventory_count, metadata)
VALUES
  (
    'e2e-product-cake',
    'e2e-tenant',
    'Vegan Celebration Cake',
    'Plant-based celebration cake for local pickup.',
    'physical',
    39.99,
    3999,
    'USD',
    12,
    '{"seeded_by":"e2e","image_url":"/dashboard_with_charts.png"}'::jsonb
  ),
  (
    'e2e-product-class',
    'e2e-tenant',
    'Cake Decorating Class',
    'Hands-on decorating session for small groups.',
    'booking',
    75.00,
    7500,
    'USD',
    8,
    '{"seeded_by":"e2e"}'::jsonb
  )
ON CONFLICT (id) DO UPDATE
SET tenant_id = EXCLUDED.tenant_id,
    title = EXCLUDED.title,
    description = EXCLUDED.description,
    type = EXCLUDED.type,
    price = EXCLUDED.price,
    price_cents = EXCLUDED.price_cents,
    currency = EXCLUDED.currency,
    inventory_count = EXCLUDED.inventory_count,
    metadata = EXCLUDED.metadata,
    updated_at = CURRENT_TIMESTAMP;

INSERT INTO customers (id, tenant_id, name, email, phone, preferences)
VALUES (
  'e2e-customer-bakery',
  'e2e-tenant',
  'Ada Baker',
  'ada.baker@example.test',
  '+15550101001',
  '{"seeded_by":"e2e","preferred_fulfillment":"pickup"}'::jsonb
)
ON CONFLICT (id) DO UPDATE
SET tenant_id = EXCLUDED.tenant_id,
    name = EXCLUDED.name,
    email = EXCLUDED.email,
    phone = EXCLUDED.phone,
    preferences = EXCLUDED.preferences,
    updated_at = CURRENT_TIMESTAMP;

INSERT INTO orders (id, tenant_id, customer_id, total_amount, status)
VALUES (
  'e2e-seeded-record',
  'e2e-tenant',
  'e2e-customer-bakery',
  39.99,
  'paid'
)
ON CONFLICT (id) DO UPDATE
SET tenant_id = EXCLUDED.tenant_id,
    customer_id = EXCLUDED.customer_id,
    total_amount = EXCLUDED.total_amount,
    status = EXCLUDED.status,
    updated_at = CURRENT_TIMESTAMP;

ALTER TABLE products ENABLE ROW LEVEL SECURITY;
ALTER TABLE users ENABLE ROW LEVEL SECURITY;
ALTER TABLE customers ENABLE ROW LEVEL SECURITY;
ALTER TABLE orders ENABLE ROW LEVEL SECURITY;

ALTER TABLE products FORCE ROW LEVEL SECURITY;
ALTER TABLE users FORCE ROW LEVEL SECURITY;
ALTER TABLE customers FORCE ROW LEVEL SECURITY;
ALTER TABLE orders FORCE ROW LEVEL SECURITY;

COMMIT;

-- Insert Documentation and Help Seed Data
INSERT INTO help_articles (tenant_id, category, title, desc_text, link) VALUES
('e2e-tenant', 'Getting Started', 'Welcome to One Human Corp', 'Let''s get your business online in under 10 minutes.', '/help_article.html?id=getting-started-1'),
('e2e-tenant', 'My Store', 'Setting up your storefront', 'Add products, track what''s in stock, and change how your store looks.', '/help_article.html?id=my-store-1'),
('e2e-tenant', 'Payments', 'Accepting your first payment', 'Learn how to accept credit cards and manage your payouts.', '/help_article.html?id=payments-1'),
('e2e-tenant', 'Advanced', 'API Documentation (for Advanced Users)', 'Interactive API reference for connecting external services.', 'api-docs.html');

INSERT INTO video_tutorials (tenant_id, title, duration, video_url) VALUES
('e2e-tenant', 'How to set up your first store easily', '1:20', 'https://www.w3schools.com/html/mov_bbb.mp4'),
('e2e-tenant', 'Connecting a bank account to accept payments', '0:45', 'https://www.w3schools.com/html/mov_bbb.mp4'),
('e2e-tenant', 'Activating your AI Support Agent', '1:25', 'https://www.w3schools.com/html/mov_bbb.mp4');

INSERT INTO tooltips (tenant_id, id, text) VALUES
('e2e-tenant', 'dashboard-walkthrough-btn', 'Take a tour of the dashboard'),
('e2e-tenant', 'api-docs-tooltip', 'Direct API access is only for custom integrations.'),
('e2e-tenant', 'kairos-nav-link-tooltip', 'Click here to see what your AI helpers are working on and how they plan.'),
('e2e-tenant', 'voice-assistant-tooltip', 'Hold to speak a command to your AI Assistant.'),
('e2e-tenant', 'rate-limit-close-tooltip', 'Dismiss this warning.'),
('e2e-tenant', 'dashboard-tooltip', 'View your daily sales and overall business health.'),
('e2e-tenant', 'generate-link-btn', 'Click here to share access with a team member.'),
('e2e-tenant', 'ask-ai-tooltip', 'Open AI Help Chat to get answers instantly.'),
('e2e-tenant', 'settings-delivery-tooltip', 'Turn this on to offer local delivery to your customers.'),
('e2e-tenant', 'help-btn-tooltip', 'Need help? Click here to access our Help Center, Ask AI, Video Tutorials, and Release Notes.'),
('e2e-tenant', 'help-search-tooltip', 'Search for help articles and videos...'),
('e2e-tenant', 'inventory-tooltip', 'Manage your inventory, prices, and stock levels.'),
('e2e-tenant', 'orders-tooltip', 'See what customers bought and track order fulfillment.'),
('e2e-tenant', 'total-sales-tooltip', 'Total revenue generated from your orders.'),
('e2e-tenant', 'recent-orders-tooltip', 'View the latest orders placed by your customers.'),
('e2e-tenant', 'inbox-activity-tooltip', 'Keep track of recent customer messages.'),
('e2e-tenant', 'help-advanced-toggle-tooltip', 'Show advanced developer options.'),
('e2e-tenant', 'help-btn-tooltip-appshell', 'Need help? Click here to access our Help Center, Ask AI, Video Tutorials, and Release Notes.'),
('e2e-tenant', 'checkout-pay-tooltip', 'Click to process your payment.'),
('e2e-tenant', 'leaderboard-link', 'Generate a gamified leaderboard for your website.') ON CONFLICT DO NOTHING;

INSERT INTO walkthrough_steps (tenant_id, page, step_order, selector, title, text) VALUES
('e2e-tenant', 'dashboard', 1, '#dashboard-title', 'Welcome', 'Business Analytics'),
('e2e-tenant', 'dashboard', 2, '#operations-map', 'Operations Map', 'Operations Map'),
('e2e-tenant', 'dashboard', 3, '#wrapped-summary', 'AI Savings', 'Here you can see the time and effort your agents have saved you.');
