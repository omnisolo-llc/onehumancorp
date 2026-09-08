BEGIN;

ALTER TABLE users DISABLE ROW LEVEL SECURITY;
ALTER TABLE products DISABLE ROW LEVEL SECURITY;
ALTER TABLE customers DISABLE ROW LEVEL SECURITY;
ALTER TABLE orders DISABLE ROW LEVEL SECURITY;
ALTER TABLE subscription_plans DISABLE ROW LEVEL SECURITY;
ALTER TABLE subscriptions DISABLE ROW LEVEL SECURITY;
ALTER TABLE fulfillment_schedules DISABLE ROW LEVEL SECURITY;

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

UPDATE tenants
SET base_currency = 'USD',
    enabled_currencies = '["USD", "EUR"]'::jsonb
WHERE id IN (
  'e2e-tenant',
  'e2e-tenant-free',
  'e2e-tenant-business',
  'e2e-tenant-unlimited'
);

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

INSERT INTO subscription_plans (
  id,
  tenant_id,
  product_id,
  name,
  description,
  price_cents,
  currency,
  frequency,
  interval,
  status
)
VALUES (
  'e2e-plan-cake-club',
  'e2e-tenant',
  'e2e-product-cake',
  'Celebration Cake Club',
  'Monthly bakery subscription seeded for browser verification.',
  3999,
  'USD',
  'month',
  'month',
  'active'
)
ON CONFLICT (id) DO UPDATE
SET tenant_id = EXCLUDED.tenant_id,
    product_id = EXCLUDED.product_id,
    name = EXCLUDED.name,
    description = EXCLUDED.description,
    price_cents = EXCLUDED.price_cents,
    currency = EXCLUDED.currency,
    frequency = EXCLUDED.frequency,
    interval = EXCLUDED.interval,
    status = EXCLUDED.status,
    updated_at = CURRENT_TIMESTAMP;

INSERT INTO subscriptions (
  id,
  tenant_id,
  customer_id,
  plan_id,
  status,
  health_score,
  current_period_end
)
VALUES (
  'e2e-subscription-cake-club',
  'e2e-tenant',
  'e2e-customer-bakery',
  'e2e-plan-cake-club',
  'active',
  98,
  CURRENT_TIMESTAMP + INTERVAL '30 days'
)
ON CONFLICT (id) DO UPDATE
SET tenant_id = EXCLUDED.tenant_id,
    customer_id = EXCLUDED.customer_id,
    plan_id = EXCLUDED.plan_id,
    status = EXCLUDED.status,
    health_score = EXCLUDED.health_score,
    current_period_end = EXCLUDED.current_period_end,
    updated_at = CURRENT_TIMESTAMP;

INSERT INTO fulfillment_schedules (
  id,
  tenant_id,
  subscription_plan_id,
  fulfillment_date,
  subscriber_count,
  status
)
VALUES (
  'e2e-fulfillment-cake-club',
  'e2e-tenant',
  'e2e-plan-cake-club',
  CURRENT_DATE + 7,
  1,
  'PENDING'
)
ON CONFLICT (id) DO UPDATE
SET tenant_id = EXCLUDED.tenant_id,
    subscription_plan_id = EXCLUDED.subscription_plan_id,
    fulfillment_date = EXCLUDED.fulfillment_date,
    subscriber_count = EXCLUDED.subscriber_count,
    status = EXCLUDED.status,
    updated_at = CURRENT_TIMESTAMP;

ALTER TABLE products ENABLE ROW LEVEL SECURITY;
ALTER TABLE users ENABLE ROW LEVEL SECURITY;
ALTER TABLE customers ENABLE ROW LEVEL SECURITY;
ALTER TABLE orders ENABLE ROW LEVEL SECURITY;
ALTER TABLE subscription_plans ENABLE ROW LEVEL SECURITY;
ALTER TABLE subscriptions ENABLE ROW LEVEL SECURITY;
ALTER TABLE fulfillment_schedules ENABLE ROW LEVEL SECURITY;

ALTER TABLE products FORCE ROW LEVEL SECURITY;
ALTER TABLE users FORCE ROW LEVEL SECURITY;
ALTER TABLE customers FORCE ROW LEVEL SECURITY;
ALTER TABLE orders FORCE ROW LEVEL SECURITY;
ALTER TABLE subscription_plans FORCE ROW LEVEL SECURITY;
ALTER TABLE subscriptions FORCE ROW LEVEL SECURITY;
ALTER TABLE fulfillment_schedules FORCE ROW LEVEL SECURITY;

COMMIT;
