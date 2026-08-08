BEGIN;

ALTER TABLE users DISABLE ROW LEVEL SECURITY;
ALTER TABLE products DISABLE ROW LEVEL SECURITY;
ALTER TABLE customers DISABLE ROW LEVEL SECURITY;
ALTER TABLE orders DISABLE ROW LEVEL SECURITY;
ALTER TABLE help_articles DISABLE ROW LEVEL SECURITY;
ALTER TABLE video_tutorials DISABLE ROW LEVEL SECURITY;

INSERT INTO help_articles (tenant_id, category, title, desc_text, link)
VALUES
  ('e2e-tenant', 'Getting Started', 'Getting Started with Your Store', 'Welcome to OneHumanCorp! Let''s get your business online in under 10 minutes.', '/help/getting-started-1'),
  ('e2e-tenant', 'My Store', 'Adding Products', 'Add products, track what''s in stock, and change how your store looks.', '/help/my-store'),
  ('e2e-tenant', 'Payments', 'Accepting Payments', 'Get paid faster with integrated payment processing.', '/help/payments'),
  ('e2e-tenant', 'AI Agents', 'Your AI Helpers', 'Learn how AI can help you manage your business.', '/help/ai-agents'),
  ('e2e-tenant', 'Marketing', 'Finding Customers', 'Send emails, run promos, and find new customers.', '/help/marketing'),
  ('e2e-tenant', 'Account & Billing', 'Managing Your Plan', 'View your bills and manage your team.', '/help/billing');

INSERT INTO video_tutorials (tenant_id, title, duration, video_url)
VALUES
  ('e2e-tenant', 'Connecting a bank account to accept payments', '1:45', '/videos/payments.mp4'),
  ('e2e-tenant', 'How to set up your first store easily', '1:20', '/videos/setup.mp4'),
  ('e2e-tenant', 'Linking your own website name', '0:45', '/videos/domain.mp4');

ALTER TABLE help_articles ENABLE ROW LEVEL SECURITY;
ALTER TABLE video_tutorials ENABLE ROW LEVEL SECURITY;

ALTER TABLE help_articles FORCE ROW LEVEL SECURITY;
ALTER TABLE video_tutorials FORCE ROW LEVEL SECURITY;

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
