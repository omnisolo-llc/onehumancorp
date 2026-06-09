BEGIN;

ALTER TABLE IF EXISTS agent_approvals DISABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS meeting_transcripts DISABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS meeting_rooms DISABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS agent_inbox DISABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS inbox_messages DISABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS shared_tasks DISABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS orders DISABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS customers DISABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS products DISABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS ohc_fx_rates DISABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS loyalty_ledger DISABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS customer360 DISABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS bookings DISABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS agents DISABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS ohc_staff_member DISABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS users DISABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS tenants DISABLE ROW LEVEL SECURITY;

INSERT INTO tenants (id, name, industry, tier)
VALUES
  ('e2e-tenant', 'OHC E2E Bakery', 'Food and beverage', 'starter'),
  ('e2e-tenant-unlimited', 'OHC E2E Pro Bakery', 'Food and beverage', 'Pro')
ON CONFLICT (id) DO UPDATE
SET name = EXCLUDED.name,
    industry = EXCLUDED.industry,
    tier = EXCLUDED.tier,
    updated_at = CURRENT_TIMESTAMP;

-- Ensure RLS allows us to insert ledger data
ALTER TABLE IF EXISTS ledger_accounts DISABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS ledger_transactions DISABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS ledger_entries DISABLE ROW LEVEL SECURITY;

-- Seed Ledger Data
INSERT INTO ledger_accounts (id, tenant_id, name, type, balance, currency, created_at, updated_at)
VALUES ('acct-1', 'e2e-tenant', 'main', 'asset', 1500.00, 'USD', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
ON CONFLICT (id) DO UPDATE
SET balance = EXCLUDED.balance,
    updated_at = EXCLUDED.updated_at;

INSERT INTO ledger_transactions (id, tenant_id, description, status, metadata, created_at, updated_at)
VALUES ('txn-1', 'e2e-tenant', 'Initial deposit', 'completed', '{}', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
ON CONFLICT (id) DO UPDATE
SET status = EXCLUDED.status,
    updated_at = EXCLUDED.updated_at;

INSERT INTO ledger_entries (id, tenant_id, transaction_id, account_id, amount, currency, direction, type, created_at)
VALUES ('entry-1', 'e2e-tenant', 'txn-1', 'acct-1', 1500.00, 'USD', 'credit', 'payment', CURRENT_TIMESTAMP)
ON CONFLICT (id) DO NOTHING;

ALTER TABLE IF EXISTS ledger_accounts ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS ledger_transactions ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS ledger_entries ENABLE ROW LEVEL SECURITY;

-- Ensure RLS allows us to insert ledger data
ALTER TABLE IF EXISTS ledger_accounts DISABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS ledger_transactions DISABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS ledger_entries DISABLE ROW LEVEL SECURITY;

-- Seed Ledger Data
INSERT INTO ledger_accounts (id, tenant_id, name, type, balance, currency, created_at, updated_at)
VALUES ('acct-1', 'e2e-tenant', 'main', 'asset', 1500.00, 'USD', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
ON CONFLICT (id) DO UPDATE
SET balance = EXCLUDED.balance,
    updated_at = EXCLUDED.updated_at;

INSERT INTO ledger_transactions (id, tenant_id, description, status, metadata, created_at, updated_at)
VALUES ('txn-1', 'e2e-tenant', 'Initial deposit', 'completed', '{}', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
ON CONFLICT (id) DO UPDATE
SET status = EXCLUDED.status,
    updated_at = EXCLUDED.updated_at;

INSERT INTO ledger_entries (id, tenant_id, transaction_id, account_id, amount, currency, direction, type, created_at)
VALUES ('entry-1', 'e2e-tenant', 'txn-1', 'acct-1', 1500.00, 'USD', 'credit', 'payment', CURRENT_TIMESTAMP)
ON CONFLICT (id) DO NOTHING;

ALTER TABLE IF EXISTS ledger_accounts ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS ledger_transactions ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS ledger_entries ENABLE ROW LEVEL SECURITY;

INSERT INTO users (id, username, email, password_hash, roles, active, tenant_id, created_at, updated_at)
VALUES
  (
    'e2e-admin-user',
    'test@example.com',
    'test@example.com',
    '$2b$10$hmVhunI7Fq2ZzQ0PguAH5OeXUyb/gNAORUpLPD2g44Ik9/Fd9sM7a',
    ARRAY['ADMIN'],
    TRUE,
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
    TRUE,
    'e2e-tenant',
    CURRENT_TIMESTAMP,
    CURRENT_TIMESTAMP
  ),
  (
    'e2e-unlimited-admin-user',
    'pro@example.com',
    'pro@example.com',
    '$2b$10$hmVhunI7Fq2ZzQ0PguAH5OeXUyb/gNAORUpLPD2g44Ik9/Fd9sM7a',
    ARRAY['ADMIN'],
    TRUE,
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


INSERT INTO ohc_staff_member (id, tenant_id, name, phone_number, role, pin_hash)
VALUES ('staff_1', 'e2e-tenant', 'Carlos', '+15550101010', 'Manager', '1234')
ON CONFLICT (id) DO UPDATE
SET name = EXCLUDED.name,
    phone_number = EXCLUDED.phone_number,
    role = EXCLUDED.role,
    pin_hash = EXCLUDED.pin_hash,
    updated_at = CURRENT_TIMESTAMP;


INSERT INTO agent_approvals (id, tenant_id, department, description, status, action_risk, payload, created_at, updated_at)
VALUES
('e2e-approval-1', 'e2e-tenant', 'customer_success', 'Draft email for review', 'DRAFT', 'HIGH', '{"feature_type": "ambassador_reply", "original_message": "Do you have vegan options for birthday cakes?", "generated_response": "Yes, we have several vegan options for birthday cakes. We would love to help you plan your special day!"}'::jsonb, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
,
('e2e-approval-social', 'e2e-tenant', 'marketing', 'Generated 7-day social media plan for Vegan Celebration Cake', 'DRAFT', 'LOW', '{"feature_type": "social_calendar"}'::jsonb, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
('e2e-approval-cart', 'e2e-tenant', 'sales', 'Abandoned cart recovery: 10% discount for Sarah', 'DRAFT', 'HIGH', '{"feature_type": "abandoned_cart", "context": {"abandoned_carts_count": 3, "potential_revenue": 120.00}}'::jsonb, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
('e2e-approval-review', 'e2e-tenant', 'customer_success', '3 customers haven''t reviewed their orders. Request reviews?', 'DRAFT', 'HIGH', '{"feature_type": "automated_review_request", "target": "recent_unreviewed_orders", "count": 3}'::jsonb, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
('e2e-approval-pricing', 'e2e-tenant', 'business_advisory', 'Smart Price Suggestion: Vegan Celebration Cake', 'PENDING', 'HIGH', '{"context": {"smart_pricing": true, "product_id": "e2e-product-cake", "product_name": "Vegan Celebration Cake", "old_price": 39.99, "new_price": 45.00, "discount_amount": -5.01, "sales_projection": "+$150", "stagnant_days": 10, "margin_percent": 45}}'::jsonb, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
,('e2e-approval-quote-draft', 'e2e-tenant', 'sales', 'Draft Quote Ready: Fix leaking sink for John Doe', 'PENDING', 'HIGH', '{"feature_type": "quote_draft", "customer_inquiry": "How much to fix a leaking sink? Here is a picture", "suggested_price": 150.0, "scope": "Fix leaking sink including labor and standard materials.", "suggested_time": "Tomorrow at 2 PM", "generated_response": "Based on our past projects, I can offer Fix leaking sink starting at 50.00. Should I send over the formal agreement?", "service": "Fix leaking sink", "price": 150.0}'::jsonb, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
('e2e-approval-social-draft', 'e2e-tenant', 'marketing', 'The Promoter generated social media captions for your new product. Review and schedule.', 'DRAFT', 'HIGH', '{"feature_type": "social_post_draft", "tiktok": "Check out our new product on TikTok!", "instagram": "New arrival! Link in bio.", "facebook": "We just added a new product to our store."}'::jsonb, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
ON CONFLICT (id) DO UPDATE
SET status = EXCLUDED.status,
    updated_at = CURRENT_TIMESTAMP;

INSERT INTO agents (id, tenant_id, name, role, status, provider_type, region)
VALUES
  ('e2e-agent-marketing', 'e2e-tenant', 'Marketing Pro', 'Marketing assistant', 'Active', 'minimax', 'us'),
  ('e2e-agent-ops', 'e2e-tenant', 'Ops Helper', 'Operations assistant', 'Active', 'minimax', 'us')
ON CONFLICT (id) DO UPDATE
SET name = EXCLUDED.name,
    role = EXCLUDED.role,
    status = EXCLUDED.status,
    provider_type = EXCLUDED.provider_type,
    region = EXCLUDED.region;

INSERT INTO customers (id, tenant_id, name, email, phone, preferences)
VALUES
  ('e2e-customer-ava', 'e2e-tenant', 'Ava Customer', 'ava@example.com', '+15550101010', '{"diet":"vegan"}'::jsonb),
  ('e2e-customer-ben', 'e2e-tenant', 'Ben Buyer', 'ben@example.com', '+15550101011', '{}'::jsonb)
ON CONFLICT (id) DO UPDATE
SET name = EXCLUDED.name,
    email = EXCLUDED.email,
    phone = EXCLUDED.phone,
    preferences = EXCLUDED.preferences,
    updated_at = CURRENT_TIMESTAMP;

INSERT INTO products (id, tenant_id, title, description, type, price, price_cents, currency, inventory_count, metadata)
VALUES
  ('e2e-product-cake', 'e2e-tenant', 'Vegan Celebration Cake', 'Plant-based celebration cake for local pickup.', 'physical', 39.99, 3999, 'USD', 12, '{"seeded_by":"e2e"}'::jsonb),
  ('e2e-product-class', 'e2e-tenant', 'Cake Decorating Class', 'Hands-on decorating session for small groups.', 'booking', 75.00, 7500, 'USD', 8, '{"seeded_by":"e2e"}'::jsonb)
ON CONFLICT (id) DO UPDATE
SET title = EXCLUDED.title,
    description = EXCLUDED.description,
    type = EXCLUDED.type,
    price = EXCLUDED.price,
    price_cents = EXCLUDED.price_cents,
    currency = EXCLUDED.currency,
    inventory_count = EXCLUDED.inventory_count,
    metadata = EXCLUDED.metadata,
    updated_at = CURRENT_TIMESTAMP;

INSERT INTO orders (id, tenant_id, customer_id, total_amount, status)
VALUES
  ('e2e-order-1', 'e2e-tenant', 'e2e-customer-ava', 39.99, 'ready'),
  ('e2e-order-2', 'e2e-tenant', 'e2e-customer-ben', 75.00, 'pending')
ON CONFLICT (id) DO UPDATE
SET customer_id = EXCLUDED.customer_id,
    total_amount = EXCLUDED.total_amount,
    status = EXCLUDED.status,
    updated_at = CURRENT_TIMESTAMP;

INSERT INTO bookings (id, tenant_id, customer_id, product_id, start_time, end_time, status)
VALUES
  ('e2e-booking-1', 'e2e-tenant', 'e2e-customer-ava', 'e2e-product-class', CURRENT_TIMESTAMP + interval '1 day', CURRENT_TIMESTAMP + interval '1 day 1 hour', 'confirmed')
ON CONFLICT (id) DO UPDATE
SET customer_id = EXCLUDED.customer_id,
    product_id = EXCLUDED.product_id,
    start_time = EXCLUDED.start_time,
    end_time = EXCLUDED.end_time,
    status = EXCLUDED.status,
    updated_at = CURRENT_TIMESTAMP;

INSERT INTO shared_tasks (id, tenant_id, title, description, status, agent_id, priority, payload)
VALUES
  ('e2e-task-restock', 'e2e-tenant', 'Prepare weekend inventory', 'Review seeded orders and prep ingredients.', 'PENDING', 'e2e-agent-ops', 'P1', '{"source":"database_seed"}'),
  ('e2e-task-social', 'e2e-tenant', 'Draft weekly promotion', 'Create a promotion for vegan celebration cakes.', 'PENDING', 'e2e-agent-marketing', 'P2', '{"source":"database_seed"}')
ON CONFLICT (id) DO UPDATE
SET title = EXCLUDED.title,
    description = EXCLUDED.description,
    status = EXCLUDED.status,
    agent_id = EXCLUDED.agent_id,
    priority = EXCLUDED.priority,
    payload = EXCLUDED.payload,
    updated_at = CURRENT_TIMESTAMP;

INSERT INTO meeting_rooms (id, tenant_id, agenda, participants)
VALUES ('e2e-room-ops', 'e2e-tenant', 'Daily operations check-in', '["Marketing Pro","Ops Helper"]')
ON CONFLICT (id) DO UPDATE
SET agenda = EXCLUDED.agenda,
    participants = EXCLUDED.participants;

INSERT INTO agent_inbox (agent_id, tenant_id, message_id, from_agent, to_agent, type, content, meeting_id)
VALUES
  ('e2e-agent-marketing', 'e2e-tenant', 'e2e-message-vegan-options', 'customer', 'e2e-agent-marketing', 'customer_question', 'Do you have vegan options for birthday cakes?', 'e2e-room-ops')
ON CONFLICT DO NOTHING;

INSERT INTO inbox_messages (id, tenant_id, source, content, draft_reply, status)
VALUES
  ('e2e-inbox-msg-1', 'e2e-tenant', 'Instagram DM', 'Do you have vegan options for birthday cakes?', 'Hi there! Yes, we do offer vegan birthday cakes. They start at $45. Would you like to see our menu?', 'pending'),
  ('e2e-inbox-msg-2', 'e2e-tenant', 'WhatsApp', 'Can I schedule a consultation for my wedding?', 'Hi! Absolutely. I have availability this Thursday at 2pm or Friday at 10am. Which works best for you?', 'pending')
ON CONFLICT DO NOTHING;

ALTER TABLE IF EXISTS tenants ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS users ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS agents ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS ohc_staff_member ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS products ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS customers ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS orders ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS shared_tasks ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS agent_inbox ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS inbox_messages ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS meeting_rooms ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS meeting_transcripts ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS customer360 ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS loyalty_ledger ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS ohc_fx_rates ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS bookings ENABLE ROW LEVEL SECURITY;

ALTER TABLE IF EXISTS tenants FORCE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS users FORCE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS agents FORCE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS ohc_staff_member FORCE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS products FORCE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS customers FORCE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS orders FORCE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS shared_tasks FORCE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS agent_inbox FORCE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS inbox_messages FORCE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS meeting_rooms FORCE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS meeting_transcripts FORCE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS customer360 FORCE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS loyalty_ledger FORCE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS ohc_fx_rates FORCE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS bookings FORCE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS ledger_accounts FORCE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS ledger_transactions FORCE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS ledger_entries FORCE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS ledger_accounts FORCE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS ledger_transactions FORCE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS ledger_entries FORCE ROW LEVEL SECURITY;

ALTER TABLE IF EXISTS agent_actions ENABLE ROW LEVEL SECURITY;

ALTER TABLE IF EXISTS purchase_orders ENABLE ROW LEVEL SECURITY;

ALTER TABLE IF EXISTS services ENABLE ROW LEVEL SECURITY;

ALTER TABLE IF EXISTS interactions ENABLE ROW LEVEL SECURITY;

ALTER TABLE IF EXISTS bom_items ENABLE ROW LEVEL SECURITY;

ALTER TABLE IF EXISTS vendors ENABLE ROW LEVEL SECURITY;

ALTER TABLE IF EXISTS ai_memories ENABLE ROW LEVEL SECURITY;

ALTER TABLE IF EXISTS po_line_items ENABLE ROW LEVEL SECURITY;

ALTER TABLE IF EXISTS order_line_items ENABLE ROW LEVEL SECURITY;

ALTER TABLE IF EXISTS raw_materials ENABLE ROW LEVEL SECURITY;

ALTER TABLE IF EXISTS customer_timeline ENABLE ROW LEVEL SECURITY;

ALTER TABLE IF EXISTS depletion_logs ENABLE ROW LEVEL SECURITY;

ALTER TABLE IF EXISTS agent_actions FORCE ROW LEVEL SECURITY;

ALTER TABLE IF EXISTS purchase_orders FORCE ROW LEVEL SECURITY;

ALTER TABLE IF EXISTS services FORCE ROW LEVEL SECURITY;

ALTER TABLE IF EXISTS interactions FORCE ROW LEVEL SECURITY;

ALTER TABLE IF EXISTS bom_items FORCE ROW LEVEL SECURITY;

ALTER TABLE IF EXISTS vendors FORCE ROW LEVEL SECURITY;

ALTER TABLE IF EXISTS ai_memories FORCE ROW LEVEL SECURITY;

ALTER TABLE IF EXISTS po_line_items FORCE ROW LEVEL SECURITY;

ALTER TABLE IF EXISTS order_line_items FORCE ROW LEVEL SECURITY;

ALTER TABLE IF EXISTS raw_materials FORCE ROW LEVEL SECURITY;

ALTER TABLE IF EXISTS customer_timeline FORCE ROW LEVEL SECURITY;

ALTER TABLE IF EXISTS depletion_logs FORCE ROW LEVEL SECURITY;

COMMIT;

-- Triage seed data
INSERT INTO triage_items (id, tenant_id, source, priority, context, status)
VALUES
  ('triage-test-1', 'test-tenant', 'Instagram DM', 'Urgent', 'Maya requested a custom cake for Friday', 'pending'),
  ('triage-test-2', 'test-tenant', 'WhatsApp', 'Medium', 'Question about delivery times', 'pending')
ON CONFLICT (id) DO NOTHING;

INSERT INTO triage_proposed_actions (id, triage_item_id, tenant_id, action_type, payload)
VALUES
  ('action-test-1', 'triage-test-1', 'test-tenant', 'Draft Reply', 'Hi Maya! I can definitely help with the custom cake. It will be $50.'),
  ('action-test-2', 'triage-test-2', 'test-tenant', 'Draft Reply', 'We deliver between 9 AM and 5 PM on weekdays.')
ON CONFLICT (id) DO NOTHING;
