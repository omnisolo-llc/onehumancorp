BEGIN;

ALTER TABLE users DISABLE ROW LEVEL SECURITY;
ALTER TABLE products DISABLE ROW LEVEL SECURITY;
ALTER TABLE customers DISABLE ROW LEVEL SECURITY;
ALTER TABLE orders DISABLE ROW LEVEL SECURITY;
ALTER TABLE orders ADD COLUMN IF NOT EXISTS notes TEXT;
ALTER TABLE orders ADD COLUMN IF NOT EXISTS translated_notes TEXT;
ALTER TABLE agent_feed_items DISABLE ROW LEVEL SECURITY;
ALTER TABLE agent_approvals DISABLE ROW LEVEL SECURITY;
ALTER TABLE job_templates DISABLE ROW LEVEL SECURITY;
ALTER TABLE appointments DISABLE ROW LEVEL SECURITY;
ALTER TABLE subscription_plans DISABLE ROW LEVEL SECURITY;
ALTER TABLE subscriptions DISABLE ROW LEVEL SECURITY;
ALTER TABLE fulfillment_schedules DISABLE ROW LEVEL SECURITY;
ALTER TABLE bookings DISABLE ROW LEVEL SECURITY;
ALTER TABLE omni_inbox_messages DISABLE ROW LEVEL SECURITY;
ALTER TABLE service_routes DISABLE ROW LEVEL SECURITY;
ALTER TABLE job_locations DISABLE ROW LEVEL SECURITY;

INSERT INTO tenants (id, name, industry, tier, plan_tier, has_claimed_trial_extension)
VALUES
  ('e2e-tenant', 'OmniSolo E2E Bakery', 'Food and beverage', 'Free', 'Free', false),
  ('e2e-tenant-free', 'OmniSolo E2E Free Bakery', 'Food and beverage', 'Free', 'Free', false),
  ('e2e-tenant-starter', 'OmniSolo E2E Starter Bakery', 'Food and beverage', 'Starter', 'Starter', false),
  ('e2e-tenant-business', 'OmniSolo E2E Business Bakery', 'Food and beverage', 'Business', 'Business', false),
  ('e2e-tenant-unlimited', 'OmniSolo E2E Pro Bakery', 'Food and beverage', 'Pro', 'Pro', false)
ON CONFLICT (id) DO UPDATE
SET name = EXCLUDED.name,
    industry = EXCLUDED.industry,
    tier = EXCLUDED.tier,
    plan_tier = EXCLUDED.plan_tier,
    has_claimed_trial_extension = EXCLUDED.has_claimed_trial_extension,
    updated_at = CURRENT_TIMESTAMP;

UPDATE tenants
SET base_currency = 'USD',
    enabled_currencies = '["USD", "EUR"]'::jsonb
WHERE id IN (
  'e2e-tenant',
  'e2e-tenant-free',
  'e2e-tenant-starter',
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
    'e2e-leo-user',
    'leo@example.com',
    'leo@example.com',
    '$2b$10$hmVhunI7Fq2ZzQ0PguAH5OeXUyb/gNAORUpLPD2g44Ik9/Fd9sM7a',
    ARRAY['ADMIN'],
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
  ),
  (
    'e2e-starter-user',
    'starter@example.com',
    'starter@example.com',
    '$2b$10$hmVhunI7Fq2ZzQ0PguAH5OeXUyb/gNAORUpLPD2g44Ik9/Fd9sM7a',
    ARRAY['ADMIN'],
    true,
    'e2e-tenant-starter',
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

-- Authentication reads normalized identity_user_roles, not the compatibility
-- users.roles array. Seed both representations in this same isolated transaction.
DELETE FROM identity_user_roles WHERE user_id IN (
  'e2e-admin-user', 'e2e-team-member', 'e2e-leo-user',
  'e2e-free-user', 'e2e-business-user', 'e2e-unlimited-admin-user',
  'e2e-starter-user'
);
INSERT INTO identity_user_roles (user_id, role_name, tenant_id, position)
SELECT u.id, role.role_name, u.tenant_id, (role.position - 1)::integer
FROM users u
CROSS JOIN LATERAL unnest(u.roles) WITH ORDINALITY AS role(role_name, position)
WHERE u.id IN (
  'e2e-admin-user', 'e2e-team-member', 'e2e-leo-user',
  'e2e-free-user', 'e2e-business-user', 'e2e-unlimited-admin-user',
  'e2e-starter-user'
);

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

INSERT INTO availability_blocks (
  id, tenant_id, service_id, start_time, end_time, is_available
)
VALUES (
  'e2e-availability-class-next-day',
  'e2e-tenant',
  'e2e-product-class',
  CURRENT_DATE + INTERVAL '1 day 09:00',
  CURRENT_DATE + INTERVAL '1 day 10:00',
  TRUE
)
ON CONFLICT (id) DO UPDATE
SET tenant_id = EXCLUDED.tenant_id,
    service_id = EXCLUDED.service_id,
    start_time = EXCLUDED.start_time,
    end_time = EXCLUDED.end_time,
    is_available = TRUE,
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

INSERT INTO bookings (id, tenant_id, customer_id, product_id, service_id, start_time, end_time, status)
VALUES (
  'e2e-booking-class-next-day',
  'e2e-tenant',
  'e2e-customer-bakery',
  'e2e-product-class',
  'e2e-product-class',
  CURRENT_DATE + INTERVAL '1 day 09:00',
  CURRENT_DATE + INTERVAL '1 day 10:00',
  'confirmed'
)
ON CONFLICT (id) DO UPDATE
SET tenant_id = EXCLUDED.tenant_id,
    customer_id = EXCLUDED.customer_id,
    product_id = EXCLUDED.product_id,
    service_id = EXCLUDED.service_id,
    start_time = EXCLUDED.start_time,
    end_time = EXCLUDED.end_time,
    status = EXCLUDED.status,
    updated_at = CURRENT_TIMESTAMP;

INSERT INTO job_templates (id, tenant_id, name)
VALUES
  ('e2e-job-template', 'e2e-tenant', 'E2E Service Visit'),
  ('e2e-template-1', 'e2e-tenant', 'Fix leaking sink'),
  ('e2e-template-2', 'e2e-tenant', 'HVAC Filter Replacement')
ON CONFLICT (id) DO UPDATE
SET tenant_id = EXCLUDED.tenant_id,
    name = EXCLUDED.name,
    updated_at = CURRENT_TIMESTAMP;

INSERT INTO appointments (
  id,
  tenant_id,
  customer_id,
  job_template_id,
  status,
  scheduled_start_time,
  scheduled_end_time,
  location_address,
  notes
)
VALUES
  (
    'e2e-appointment',
    'e2e-tenant',
    'e2e-customer-bakery',
    'e2e-job-template',
    'Scheduled',
    CURRENT_TIMESTAMP + INTERVAL '1 day',
    CURRENT_TIMESTAMP + INTERVAL '1 day 1 hour',
    '123 OmniSolo Way',
    'Seeded browser regression appointment'
  ),
  (
    'e2e-appt-1',
    'e2e-tenant',
    'e2e-customer-bakery',
    'e2e-template-1',
    'Scheduled',
    CURRENT_TIMESTAMP,
    CURRENT_TIMESTAMP + INTERVAL '1 hour',
    '123 Main St, Austin, TX',
    'Fix leaking sink appointment'
  ),
  (
    'e2e-appt-2',
    'e2e-tenant',
    'e2e-customer-bakery',
    'e2e-template-2',
    'Scheduled',
    CURRENT_TIMESTAMP + INTERVAL '2 hours',
    CURRENT_TIMESTAMP + INTERVAL '3 hours',
    '456 Oak Ave, Austin, TX',
    'HVAC replacement appointment'
  )
ON CONFLICT (id) DO UPDATE
SET tenant_id = EXCLUDED.tenant_id,
    customer_id = EXCLUDED.customer_id,
    job_template_id = EXCLUDED.job_template_id,
    status = EXCLUDED.status,
    scheduled_start_time = EXCLUDED.scheduled_start_time,
    scheduled_end_time = EXCLUDED.scheduled_end_time,
    location_address = EXCLUDED.location_address,
    notes = EXCLUDED.notes,
    updated_at = CURRENT_TIMESTAMP;

INSERT INTO service_routes (id, tenant_id, agent_id, route_date, status)
VALUES
  ('e2e-route-today', 'e2e-tenant', 'e2e-staff-carlos', CURRENT_DATE, 'active')
ON CONFLICT (id) DO UPDATE
SET tenant_id = EXCLUDED.tenant_id,
    agent_id = EXCLUDED.agent_id,
    route_date = EXCLUDED.route_date,
    status = EXCLUDED.status,
    updated_at = CURRENT_TIMESTAMP;

INSERT INTO job_locations (id, tenant_id, service_route_id, appointment_id, sequence_order, status)
VALUES
  ('e2e-job-1', 'e2e-tenant', 'e2e-route-today', 'e2e-appt-1', 1, 'pending'),
  ('e2e-job-2', 'e2e-tenant', 'e2e-route-today', 'e2e-appt-2', 2, 'pending')
ON CONFLICT (id) DO UPDATE
SET tenant_id = EXCLUDED.tenant_id,
    service_route_id = EXCLUDED.service_route_id,
    appointment_id = EXCLUDED.appointment_id,
    sequence_order = EXCLUDED.sequence_order,
    status = EXCLUDED.status,
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

INSERT INTO agent_feed_items (
  id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state
)
VALUES (
  'e2e-feed-churn',
  'e2e-tenant',
  'CustomerSuccess',
  '{"feature_type":"subscription_churn_risk","customer_id":"e2e-customer-bakery","description":"A subscriber is at risk of churning","reason":"No recent activity in 30 days and renewal is approaching"}'::jsonb,
  '{"feature_type":"subscription_churn_risk","action_type":"DraftForReview","generated_response":"We miss you. Book a complimentary catch-up session and keep your momentum going."}'::jsonb,
  'PENDING_APPROVAL'
),
(
  'e2e-feed-inbox-quote-1',
  'e2e-tenant',
  'Sales',
  '{"description":"Vegan pastry box quote approval","customer_id":"maya_bakes"}'::jsonb,
  '{"inbox_message_id":"e2e-inbox-msg-1","action_type":"Draft Quote","feature_type":"quote_draft","total_amount":75.00,"total_amount_cents":7500,"scope":"Vegan options for Saturday","line_items":[{"description":"Vegan Pastry Box","unit_price_cents":7500,"quantity":1}]}'::jsonb,
  'PENDING_APPROVAL'
),
(
  'e2e-feed-ambassador-reply',
  'e2e-tenant',
  'Ambassador',
  '{"feature_type":"ambassador_reply","source":"Instagram","past_orders":"Returning Customer (2 past orders).","context_used":"Customer prefers vegan options.","original_message":"Do you have vegan options?"}'::jsonb,
  '{"feature_type":"ambassador_reply","action_type":"DraftForReview","source":"Instagram","past_orders":"Returning Customer (2 past orders).","context_used":"Customer prefers vegan options.","original_message":"Do you have vegan options?","generated_response":"Yes! We have a full vegan pastry selection."}'::jsonb,
  'PENDING_APPROVAL'
)
ON CONFLICT (id) DO UPDATE
SET tenant_id = EXCLUDED.tenant_id,
    event_source = EXCLUDED.event_source,
    context_payload = EXCLUDED.context_payload,
    proposed_action = EXCLUDED.proposed_action,
    lifecycle_state = 'PENDING_APPROVAL',
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

INSERT INTO agent_approvals (
  id,
  tenant_id,
  department,
  description,
  status,
  action_risk,
  payload,
  created_at,
  updated_at
)
VALUES (
  'e2e-approval-quote-sink',
  'e2e-tenant',
  'Field Operations',
  'Fix leaking sink for John Doe',
  'DRAFT',
  'low',
  '{"feature_type": "quote_draft", "scope": "Fix leaking sink for John Doe", "service": "Plumbing Repair", "suggested_price": 250, "line_items": [{"description": "Fix leaking sink for John Doe", "unit_price_cents": 25000, "quantity": 1}]}'::jsonb,
  CURRENT_TIMESTAMP,
  CURRENT_TIMESTAMP
)
ON CONFLICT (id) DO UPDATE
SET tenant_id = EXCLUDED.tenant_id,
    department = EXCLUDED.department,
    description = EXCLUDED.description,
    status = EXCLUDED.status,
    action_risk = EXCLUDED.action_risk,
    payload = EXCLUDED.payload,
    updated_at = CURRENT_TIMESTAMP;

INSERT INTO agent_approvals (
  id,
  tenant_id,
  department,
  description,
  status,
  action_risk,
  payload,
  created_at,
  updated_at
)
VALUES (
  'e2e-approval-inbox-quote-1',
  'e2e-tenant',
  'Sales',
  'Vegan pastry box quote approval',
  'PENDING',
  'low',
  '{"inbox_message_id": "e2e-inbox-msg-1", "action_type": "Draft Quote", "feature_type": "quote_draft", "total_amount": 75.00, "total_amount_cents": 7500, "scope": "Vegan options for Saturday", "line_items": [{"description": "Vegan Pastry Box", "unit_price_cents": 7500, "quantity": 1}]}'::jsonb,
  CURRENT_TIMESTAMP,
  CURRENT_TIMESTAMP
)
ON CONFLICT (id) DO UPDATE
SET tenant_id = EXCLUDED.tenant_id,
    department = EXCLUDED.department,
    description = EXCLUDED.description,
    status = EXCLUDED.status,
    action_risk = EXCLUDED.action_risk,
    payload = EXCLUDED.payload,
    updated_at = CURRENT_TIMESTAMP;

INSERT INTO omni_inbox_messages (
  id,
  tenant_id,
  source,
  sender_id,
  customer_id,
  original_content,
  translated_content,
  target_language,
  draft_reply,
  status,
  created_at,
  updated_at
)
VALUES (
  'e2e-inbox-msg-1',
  'e2e-tenant',
  'instagram',
  'maya_bakes',
  'e2e-customer-bakery',
  'Do you have vegan options for Saturday?',
  'Do you have vegan options for Saturday?',
  'en',
  'Yes! We have several delicious vegan pastries available this Saturday.',
  'pending',
  CURRENT_TIMESTAMP,
  CURRENT_TIMESTAMP
)
ON CONFLICT (id) DO UPDATE
SET tenant_id = EXCLUDED.tenant_id,
    source = EXCLUDED.source,
    sender_id = EXCLUDED.sender_id,
    customer_id = EXCLUDED.customer_id,
    original_content = EXCLUDED.original_content,
    translated_content = EXCLUDED.translated_content,
    target_language = EXCLUDED.target_language,
    draft_reply = EXCLUDED.draft_reply,
    status = EXCLUDED.status,
    updated_at = CURRENT_TIMESTAMP;

CREATE TABLE IF NOT EXISTS applied_client_mutations (
    client_mutation_id VARCHAR PRIMARY KEY,
    tenant_id VARCHAR NOT NULL,
    applied_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_applied_client_mutations_tenant ON applied_client_mutations(tenant_id);
ALTER TABLE applied_client_mutations ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_applied_client_mutations ON applied_client_mutations;
CREATE POLICY tenant_isolation_applied_client_mutations ON applied_client_mutations
    FOR ALL
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

CREATE TABLE IF NOT EXISTS agent_feed (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    source TEXT NOT NULL,
    priority TEXT,
    title TEXT,
    description TEXT,
    payload JSONB,
    state TEXT NOT NULL DEFAULT 'PENDING_APPROVAL',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_agent_feed_tenant ON agent_feed(tenant_id);
ALTER TABLE agent_feed ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_agent_feed ON agent_feed;
CREATE POLICY tenant_isolation_agent_feed ON agent_feed
    FOR ALL
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

CREATE OR REPLACE FUNCTION sync_agent_feed_to_items()
RETURNS TRIGGER AS $$
BEGIN
    INSERT INTO agent_feed_items (id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state, created_at, updated_at)
    VALUES (
        NEW.id,
        NEW.tenant_id,
        NEW.source,
        jsonb_build_object('description', NEW.description, 'title', NEW.title, 'priority', NEW.priority),
        CASE
            WHEN NEW.payload IS NULL THEN '{}'::jsonb
            WHEN jsonb_typeof(NEW.payload) = 'object' THEN NEW.payload
            ELSE jsonb_build_object('raw', NEW.payload)
        END,
        NEW.state,
        COALESCE(NEW.created_at, CURRENT_TIMESTAMP),
        COALESCE(NEW.updated_at, CURRENT_TIMESTAMP)
    )
    ON CONFLICT (id) DO UPDATE SET
        lifecycle_state = EXCLUDED.lifecycle_state,
        context_payload = EXCLUDED.context_payload,
        proposed_action = EXCLUDED.proposed_action,
        updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trg_agent_feed_sync_items ON agent_feed;
CREATE TRIGGER trg_agent_feed_sync_items
AFTER INSERT OR UPDATE ON agent_feed
FOR EACH ROW
EXECUTE FUNCTION sync_agent_feed_to_items();

CREATE OR REPLACE FUNCTION sync_agent_feed_delete()
RETURNS TRIGGER AS $$
BEGIN
    DELETE FROM agent_feed_items WHERE id = OLD.id;
    RETURN OLD;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trg_agent_feed_sync_delete ON agent_feed;
CREATE TRIGGER trg_agent_feed_sync_delete
AFTER DELETE ON agent_feed
FOR EACH ROW
EXECUTE FUNCTION sync_agent_feed_delete();

CREATE OR REPLACE FUNCTION sync_items_to_agent_feed()
RETURNS TRIGGER AS $$
BEGIN
    UPDATE agent_feed
    SET state = NEW.lifecycle_state,
        updated_at = CURRENT_TIMESTAMP
    WHERE id = NEW.id;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trg_items_sync_agent_feed ON agent_feed_items;
CREATE TRIGGER trg_items_sync_agent_feed
AFTER UPDATE OF lifecycle_state ON agent_feed_items
FOR EACH ROW
EXECUTE FUNCTION sync_items_to_agent_feed();

CREATE TABLE IF NOT EXISTS staff_tasks (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    staff_id TEXT NOT NULL,
    title TEXT NOT NULL DEFAULT '',
    description TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending',
    priority TEXT NOT NULL DEFAULT 'medium',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_staff_tasks_tenant_id ON staff_tasks(tenant_id);
CREATE INDEX IF NOT EXISTS idx_staff_tasks_staff_id ON staff_tasks(staff_id);
ALTER TABLE staff_tasks ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_staff_tasks ON staff_tasks;
CREATE POLICY tenant_isolation_staff_tasks
ON staff_tasks
USING (tenant_id::text = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

CREATE TABLE IF NOT EXISTS shift_summaries (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    shift_date DATE NOT NULL,
    summary_text TEXT NOT NULL,
    escalations TEXT,
    metrics JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
ALTER TABLE shift_summaries ADD COLUMN IF NOT EXISTS escalations TEXT;
CREATE INDEX IF NOT EXISTS idx_shift_summaries_tenant_id ON shift_summaries(tenant_id);
ALTER TABLE shift_summaries ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_shift_summaries ON shift_summaries;
CREATE POLICY tenant_isolation_shift_summaries
ON shift_summaries
USING (tenant_id::text = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE products ENABLE ROW LEVEL SECURITY;
ALTER TABLE users ENABLE ROW LEVEL SECURITY;
ALTER TABLE customers ENABLE ROW LEVEL SECURITY;
ALTER TABLE orders ENABLE ROW LEVEL SECURITY;
ALTER TABLE agent_feed_items ENABLE ROW LEVEL SECURITY;
ALTER TABLE agent_approvals ENABLE ROW LEVEL SECURITY;
ALTER TABLE job_templates ENABLE ROW LEVEL SECURITY;
ALTER TABLE appointments ENABLE ROW LEVEL SECURITY;
ALTER TABLE subscription_plans ENABLE ROW LEVEL SECURITY;
ALTER TABLE subscriptions ENABLE ROW LEVEL SECURITY;
ALTER TABLE fulfillment_schedules ENABLE ROW LEVEL SECURITY;
ALTER TABLE bookings ENABLE ROW LEVEL SECURITY;
ALTER TABLE omni_inbox_messages ENABLE ROW LEVEL SECURITY;
ALTER TABLE service_routes ENABLE ROW LEVEL SECURITY;
ALTER TABLE job_locations ENABLE ROW LEVEL SECURITY;
ALTER TABLE applied_client_mutations ENABLE ROW LEVEL SECURITY;
ALTER TABLE agent_feed ENABLE ROW LEVEL SECURITY;
ALTER TABLE staff_tasks ENABLE ROW LEVEL SECURITY;
ALTER TABLE shift_summaries ENABLE ROW LEVEL SECURITY;

ALTER TABLE products FORCE ROW LEVEL SECURITY;
ALTER TABLE users FORCE ROW LEVEL SECURITY;
ALTER TABLE customers FORCE ROW LEVEL SECURITY;
ALTER TABLE orders FORCE ROW LEVEL SECURITY;
ALTER TABLE agent_feed_items FORCE ROW LEVEL SECURITY;
ALTER TABLE agent_approvals FORCE ROW LEVEL SECURITY;
ALTER TABLE job_templates FORCE ROW LEVEL SECURITY;
ALTER TABLE appointments FORCE ROW LEVEL SECURITY;
ALTER TABLE subscription_plans FORCE ROW LEVEL SECURITY;
ALTER TABLE subscriptions FORCE ROW LEVEL SECURITY;
ALTER TABLE fulfillment_schedules FORCE ROW LEVEL SECURITY;
ALTER TABLE bookings FORCE ROW LEVEL SECURITY;
ALTER TABLE omni_inbox_messages FORCE ROW LEVEL SECURITY;
ALTER TABLE service_routes FORCE ROW LEVEL SECURITY;
ALTER TABLE job_locations FORCE ROW LEVEL SECURITY;
ALTER TABLE applied_client_mutations FORCE ROW LEVEL SECURITY;
ALTER TABLE agent_feed FORCE ROW LEVEL SECURITY;
ALTER TABLE staff_tasks FORCE ROW LEVEL SECURITY;
ALTER TABLE shift_summaries FORCE ROW LEVEL SECURITY;

COMMIT;
