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
ALTER TABLE IF EXISTS business_milestones DISABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS tenants DISABLE ROW LEVEL SECURITY;

INSERT INTO tenants (id, name, industry, tier, has_claimed_trial_extension, default_currency)
VALUES
  ('e2e-tenant', 'OHC E2E Bakery', 'Food and beverage', 'Starter', false, 'USD'),
  ('e2e-tenant-free', 'OHC E2E Free Bakery', 'Food and beverage', 'Free', false, 'USD'),
  ('e2e-tenant-business', 'OHC E2E Business Bakery', 'Food and beverage', 'Business', false, 'USD'),
  ('e2e-tenant-unlimited', 'OHC E2E Pro Bakery', 'Food and beverage', 'Pro', false, 'USD')
ON CONFLICT (id) DO UPDATE
SET name = EXCLUDED.name,
    industry = EXCLUDED.industry,
    tier = EXCLUDED.tier,
    has_claimed_trial_extension = EXCLUDED.has_claimed_trial_extension,
    updated_at = CURRENT_TIMESTAMP;

-- Ensure RLS allows us to insert ledger data
ALTER TABLE IF EXISTS ledger_accounts DISABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS ledger_transactions DISABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS ledger_entries DISABLE ROW LEVEL SECURITY;


-- Seed Milestones
INSERT INTO business_milestones (id, tenant_id, milestone_type, reached_at)
VALUES ('m-1', 'e2e-tenant', '10th_order', CURRENT_TIMESTAMP)
ON CONFLICT DO NOTHING;

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
    'e2e-free-user',
    'free@example.com',
    'free@example.com',
    '$2b$10$hmVhunI7Fq2ZzQ0PguAH5OeXUyb/gNAORUpLPD2g44Ik9/Fd9sM7a',
    ARRAY['ADMIN'],
    TRUE,
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
    TRUE,
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



INSERT INTO agent_feed_items (id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state, created_at, updated_at)
VALUES
('e2e-proactive-ops-1', 'e2e-tenant', 'operations', '{"feature_type": "proactive_ops", "description": "Review Daily Prep Checklist"}'::jsonb, '{"action_type": "mark_complete", "message": "Review Checklist", "feature_type": "proactive_ops"}'::jsonb, 'PENDING_APPROVAL', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
('e2e-proactive-ops-2', 'e2e-tenant', 'operations', '{"feature_type": "proactive_ops", "description": "Follow up on delayed supplier delivery from yesterday"}'::jsonb, '{"action_type": "assign_to_staff", "message": "Assign to Staff", "feature_type": "proactive_ops"}'::jsonb, 'PENDING_APPROVAL', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
('e2e-proactive-ops-3', 'e2e-tenant', 'operations', '{"feature_type": "proactive_ops", "description": "Staffing alert: Only 1 person scheduled for closing shift."}'::jsonb, '{"action_type": "draft_schedule_request", "message": "Draft Schedule Request", "feature_type": "proactive_ops"}'::jsonb, 'PENDING_APPROVAL', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
ON CONFLICT DO NOTHING;

INSERT INTO agent_feed_items (id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state, created_at, updated_at)

VALUES
('e2e-feed-social', 'e2e-tenant', 'marketing', '{"feature_type": "social_post_draft", "tiktok": "Check out our new product!", "instagram": "New arrival! Link in bio.", "facebook": "We just added a new product to our store."}'::jsonb, '{}'::jsonb, 'PENDING_APPROVAL', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
ON CONFLICT (id) DO UPDATE
SET lifecycle_state = EXCLUDED.lifecycle_state,
    updated_at = CURRENT_TIMESTAMP;

INSERT INTO agent_approvals (id, tenant_id, department, description, status, action_risk, payload, created_at, updated_at)
VALUES
('e2e-approval-1', 'e2e-tenant', 'customer_success', 'Draft email for review', 'DRAFT', 'HIGH', '{"feature_type": "ambassador_reply", "original_message": "Do you have vegan options for birthday cakes?", "generated_response": "Yes, we have several vegan options for birthday cakes. We would love to help you plan your special day!", "past_orders": "Returning Customer (2 past orders).", "context_used": "Customer prefers vegan options."}'::jsonb, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
,
('e2e-approval-social', 'e2e-tenant', 'marketing', 'Generated 7-day social media plan for Vegan Celebration Cake', 'DRAFT', 'LOW', '{"feature_type": "social_post_draft"}'::jsonb, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
('e2e-approval-cart', 'e2e-tenant', 'sales', 'Abandoned cart recovery: 10% discount for Sarah', 'DRAFT', 'HIGH', '{"feature_type": "abandoned_cart", "context": {"abandoned_carts_count": 3, "potential_revenue": 120.00}}'::jsonb, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
('e2e-approval-review', 'e2e-tenant', 'customer_success', '3 customers haven''t reviewed their orders. Request reviews?', 'DRAFT', 'HIGH', '{"feature_type": "automated_review_request", "target": "recent_unreviewed_orders", "count": 3}'::jsonb, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
('e2e-approval-pricing', 'e2e-tenant', 'business_advisory', 'Smart Price Suggestion: Vegan Celebration Cake', 'DRAFT', 'HIGH', '{"context": {"smart_pricing": true, "product_id": "e2e-product-cake", "product_name": "Vegan Celebration Cake", "old_price": 39.99, "new_price": 45.00, "discount_amount": -5.01, "sales_projection": "+$150", "stagnant_days": 10, "margin_percent": 45}}'::jsonb, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
,('823e4567-e89b-12d3-a456-426614174000', 'e2e-tenant', 'sales', 'Draft Quote Ready: Fix leaking sink for John Doe', 'DRAFT', 'HIGH', '{"feature_type": "quote_draft", "quote_id": "823e4567-e89b-12d3-a456-426614174000", "customer_inquiry": "How much to fix a leaking sink? Here is a picture", "suggested_price": 150.0, "scope": "Fix leaking sink including labor and standard materials.", "suggested_time": "Tomorrow at 2 PM", "generated_response": "Based on our past projects, I can offer Fix leaking sink starting at 50.00. Should I send over the formal agreement?", "service": "Fix leaking sink", "price": 150.0}'::jsonb, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
('e2e-approval-omnichannel-return', 'e2e-tenant', 'operations', 'Return requested by Sarah for Order #1042. Operations Agent has generated a return label and prepared a $45.00 refund. Tap ''Approve'' to finalize.', 'DRAFT', 'HIGH', '{"feature_type": "omnichannel_return", "order_id": "1042", "product_id": "product-123", "return_type": "Refund", "refund_amount": 45.00}'::jsonb, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
('app-test-ab12-34f7-e43e-7264a9c4021d', 'e2e-tenant', 'Operations', 'Mark requested to reschedule his 4 PM lesson to 5 PM today. You have a conflict. Suggest tomorrow at 4 PM?', 'DRAFT', 'HIGH', '{"context":{"description": "Mark requested to reschedule his 4 PM lesson to 5 PM today. You have a conflict. Suggest tomorrow at 4 PM?"}}'::jsonb, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
('app-test-cd34-34f7-e43e-7264a9c4021d', 'e2e-tenant', 'Operations', 'Agent tentatively booked a roof repair estimate for Sarah on Tuesday 2 PM. Pending $50 deposit. No action needed.', 'DRAFT', 'HIGH', '{"context":{"description": "Agent tentatively booked a roof repair estimate for Sarah on Tuesday 2 PM. Pending $50 deposit. No action needed."}}'::jsonb, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
ON CONFLICT (id) DO UPDATE
SET status = EXCLUDED.status,
    updated_at = CURRENT_TIMESTAMP;

INSERT INTO agent_feed_items (id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state, created_at, updated_at)
VALUES
('e2e-proactive-ops-1', 'e2e-tenant', 'operations', '{"feature_type": "proactive_ops", "description": "Review Daily Prep Checklist"}'::jsonb, '{"action_type": "mark_complete", "message": "Review Checklist", "feature_type": "proactive_ops"}'::jsonb, 'PENDING_APPROVAL', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
('e2e-proactive-ops-2', 'e2e-tenant', 'operations', '{"feature_type": "proactive_ops", "description": "Follow up on delayed supplier delivery from yesterday"}'::jsonb, '{"action_type": "assign_to_staff", "message": "Assign to Staff", "feature_type": "proactive_ops"}'::jsonb, 'PENDING_APPROVAL', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
('e2e-proactive-ops-3', 'e2e-tenant', 'operations', '{"feature_type": "proactive_ops", "description": "Staffing alert: Only 1 person scheduled for closing shift."}'::jsonb, '{"action_type": "draft_schedule_request", "message": "Draft Schedule Request", "feature_type": "proactive_ops"}'::jsonb, 'PENDING_APPROVAL', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
ON CONFLICT DO NOTHING;

INSERT INTO agent_feed_items (id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state, created_at, updated_at)

VALUES
  ('app-test-ab12-34f7-e43e-7264a9c4021d', 'e2e-tenant', 'Operations', '{"description": "Mark requested to reschedule his 4 PM lesson to 5 PM today. You have a conflict. Suggest tomorrow at 4 PM?"}', '{"context":{"description": "Mark requested to reschedule his 4 PM lesson to 5 PM today. You have a conflict. Suggest tomorrow at 4 PM?"}}', 'PENDING_APPROVAL', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('app-test-cd34-34f7-e43e-7264a9c4021d', 'e2e-tenant', 'Operations', '{"description": "Agent tentatively booked a roof repair estimate for Sarah on Tuesday 2 PM. Pending $50 deposit. No action needed."}', '{"context":{"description": "Agent tentatively booked a roof repair estimate for Sarah on Tuesday 2 PM. Pending $50 deposit. No action needed."}}', 'PENDING_APPROVAL', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('e2e-feed-test-proactive', 'e2e-tenant', 'proactive_analysis', '{"summary": "You have 3 pending orders and 1 unconfirmed booking. You should follow up.", "insight_type": "operations"}', '{"title": "DraftFollowups", "description": "Draft followups for pending orders", "type": "DraftFollowups", "payload": "Draft followups for pending orders"}', 'PENDING_APPROVAL', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
ON CONFLICT (id) DO NOTHING;

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

INSERT INTO products (id, tenant_id, title, description, type, price, price_cents, currency, inventory_count, metadata, is_subscribable, subscription_frequency, subscription_discount_percent)
VALUES
  ('e2e-product-cake', 'e2e-tenant', 'Vegan Celebration Cake', 'Plant-based celebration cake for local pickup.', 'physical', 39.99, 3999, 'USD', 12, '{"seeded_by":"e2e"}'::jsonb, true, 'monthly', 10),
  ('e2e-product-class', 'e2e-tenant', 'Cake Decorating Class', 'Hands-on decorating session for small groups.', 'booking', 75.00, 7500, 'USD', 8, '{"seeded_by":"e2e"}'::jsonb, false, null, null)
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

INSERT INTO orders (id, tenant_id, customer_id, total_amount_cents, currency, status)
VALUES
  ('e2e-order-1', 'e2e-tenant', 'e2e-customer-ava', 39.99, 'USD', 'ready'),
  ('e2e-order-2', 'e2e-tenant', 'e2e-customer-ben', 75.00, 'USD', 'pending'),
  ('e2e-order-abandoned-1', 'e2e-tenant', 'e2e-customer-ben', 100.00, 'USD', 'abandoned')
ON CONFLICT (id) DO UPDATE
SET customer_id = EXCLUDED.customer_id,
    total_amount_cents = EXCLUDED.total_amount_cents,
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

INSERT INTO inbox_messages (id, tenant_id, source, content, draft_reply, status, sender_id)
VALUES
  ('e2e-inbox-msg-1', 'e2e-tenant', 'Instagram DM', 'Do you have vegan options for birthday cakes?', 'Hi there! Yes, we do offer vegan birthday cakes. They start at $45. Would you like to see our menu?', 'pending', 'maya_bakes'),
  ('e2e-inbox-msg-2', 'e2e-tenant', 'WhatsApp', 'Can I schedule a consultation for my wedding?', 'Hi! Absolutely. I have availability this Thursday at 2pm or Friday at 10am. Which works best for you?', 'pending', '+15550102')
ON CONFLICT DO NOTHING;

INSERT INTO omni_inbox_messages (id, tenant_id, source, original_content, translated_content, target_language, draft_reply, status, sender_id, customer_id)
VALUES
  ('e2e-inbox-msg-1', 'e2e-tenant', 'Instagram DM', 'Do you have vegan options for birthday cakes?', 'Do you have vegan options for birthday cakes?', 'English', 'Hi there! Yes, we do offer vegan birthday cakes. They start at $45. Would you like to see our menu?', 'pending', 'maya_bakes', 'e2e-customer-ava'),
  ('e2e-inbox-msg-2', 'e2e-tenant', 'WhatsApp', 'Can I schedule a consultation for my wedding?', 'Can I schedule a consultation for my wedding?', 'English', 'Hi! Absolutely. I have availability this Thursday at 2pm or Friday at 10am. Which works best for you?', 'pending', '+15550102', 'e2e-customer-ben')
ON CONFLICT DO NOTHING;

ALTER TABLE IF EXISTS tenants ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS users ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS business_milestones ENABLE ROW LEVEL SECURITY;
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
ALTER TABLE IF EXISTS business_milestones FORCE ROW LEVEL SECURITY;
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
  ('triage-test-1', 'e2e-tenant', 'Instagram DM', 'Urgent', 'Maya requested a custom cake for Friday', 'pending'),
  ('triage-test-2', 'e2e-tenant', 'WhatsApp', 'Medium', 'Question about delivery times', 'pending')
ON CONFLICT (id) DO NOTHING;

INSERT INTO triage_proposed_actions (id, triage_item_id, tenant_id, action_type, payload)
VALUES
  ('action-test-1', 'triage-test-1', 'e2e-tenant', 'Draft Reply', 'Hi Maya! I can definitely help with the custom cake. It will be $50.'),
  ('action-test-2', 'triage-test-2', 'e2e-tenant', 'Draft Reply', 'We deliver between 9 AM and 5 PM on weekdays.')
ON CONFLICT (id) DO NOTHING;

-- Seed 10th order milestone for e2e-tenant
INSERT INTO business_milestones (id, tenant_id, milestone_type, reached_at)
VALUES ('ms_e2e_10th_order', 'e2e-tenant', '10th_order', NOW())
ON CONFLICT (id) DO NOTHING;
-- Seed real data for Chaos Report
INSERT INTO telemetry_buffer (tenant_id, metric_name, metric_type, value, labels_json, timestamp, sync_status) VALUES
('test_org', 'api_latency', 'histogram', 12.0, '{}', CURRENT_TIMESTAMP, 'PENDING');
INSERT INTO telemetry_buffer (tenant_id, metric_name, metric_type, value, labels_json, timestamp, sync_status) VALUES
('test_org', 'api_latency', 'histogram', 22.5, '{}', CURRENT_TIMESTAMP, 'PENDING');
INSERT INTO telemetry_buffer (tenant_id, metric_name, metric_type, value, labels_json, timestamp, sync_status) VALUES
('test_org', 'api_latency', 'histogram', 35.0, '{}', CURRENT_TIMESTAMP, 'PENDING');
INSERT INTO telemetry_buffer (tenant_id, metric_name, metric_type, value, labels_json, timestamp, sync_status) VALUES
('test_org', 'api_latency', 'histogram', 65.0, '{}', CURRENT_TIMESTAMP, 'PENDING');
INSERT INTO telemetry_buffer (tenant_id, metric_name, metric_type, value, labels_json, timestamp, sync_status) VALUES
('test_org', 'api_latency', 'histogram', 150.0, '{}', CURRENT_TIMESTAMP, 'PENDING');
INSERT INTO telemetry_buffer (tenant_id, metric_name, metric_type, value, labels_json, timestamp, sync_status) VALUES
('test_org', 'api_latency', 'histogram', 400.0, '{}', CURRENT_TIMESTAMP, 'PENDING');
INSERT INTO telemetry_buffer (tenant_id, metric_name, metric_type, value, labels_json, timestamp, sync_status) VALUES
('test_org', 'api_latency', 'histogram', 850.0, '{}', CURRENT_TIMESTAMP, 'PENDING');

INSERT INTO telemetry_buffer (tenant_id, metric_name, metric_type, value, labels_json, timestamp, sync_status) VALUES
('test_org', 'error_rate', 'gauge', 0.012, '{}', CURRENT_TIMESTAMP, 'PENDING');
INSERT INTO telemetry_buffer (tenant_id, metric_name, metric_type, value, labels_json, timestamp, sync_status) VALUES
('test_org', 'error_rate', 'gauge', 0.021, '{}', CURRENT_TIMESTAMP, 'PENDING');
INSERT INTO telemetry_buffer (tenant_id, metric_name, metric_type, value, labels_json, timestamp, sync_status) VALUES
('test_org', 'error_rate', 'gauge', 0.038, '{}', CURRENT_TIMESTAMP, 'PENDING');
INSERT INTO telemetry_buffer (tenant_id, metric_name, metric_type, value, labels_json, timestamp, sync_status) VALUES
('test_org', 'error_rate', 'gauge', 0.025, '{}', CURRENT_TIMESTAMP, 'PENDING');
INSERT INTO telemetry_buffer (tenant_id, metric_name, metric_type, value, labels_json, timestamp, sync_status) VALUES
('test_org', 'error_rate', 'gauge', 0.008, '{}', CURRENT_TIMESTAMP, 'PENDING');
INSERT INTO agent_actions (id, tenant_id, session_id, agent_id, action_type, result, created_at, input_tokens, output_tokens)
VALUES ('e2e-cost-1', 'e2e-tenant', 'session1', 'e2e-agent', 'generate', '{"status": "ok"}', CURRENT_TIMESTAMP, 1000000000, 1000000000)
ON CONFLICT DO NOTHING;
ALTER TABLE IF EXISTS active_discounts ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS affiliate_ledgers ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS affiliate_links ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS affiliate_payouts ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS agent_action_requests ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS agent_actions ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS agent_approvals ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS agent_departments ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS agent_feed_items ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS agent_inbox ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS agent_kv_store ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS agent_memories ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS agent_missions ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS agent_status ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS agent_violations ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS agents ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS ai_memories ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS assistant_artifacts ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS assistant_file_changes ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS assistant_messages ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS assistant_tasks ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS assistant_workspaces ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS autodream_memories ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS availability_blocks ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS availability_ledger ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS availability_schedules ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS bom_items ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS booking_resource_reservations ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS booking_resources ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS bookings ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS builder_blocks ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS builder_brand_toolboxes ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS builder_pages ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS builder_sites ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS business_milestones ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS businesses ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS calendar_integrations ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS campaign_assets ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS campaigns ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS channel_executions ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS competitor_metrics ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS consolidated_memory ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS conversational_checkout_sessions ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS customer360 ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS customer_identities ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS customer_timeline ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS customers ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS department_dead_letters ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS department_tasks ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS depletion_logs ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS fulfillment_batches ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS hybrid_fs_sync_queue ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS inbox_messages ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS interactions ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS inventory_levels ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS inventory_predictions ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS invoice_line_items ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS invoices ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS knowledge_embeddings ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS lead_gen_campaigns ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS leads ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS ledger_accounts ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS ledger_entries ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS ledger_transactions ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS loyalty_ledger ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS mcp_config_sync_log ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS mcp_servers ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS meeting_rooms ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS meeting_transcripts ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS memories ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS migration_jobs ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS multi_party_split_ledgers ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS multi_party_splits ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS ohc_collective ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS ohc_collective_loyalty_balance ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS ohc_collective_member ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS ohc_i18n_strings ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS ohc_job_queue ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS ohc_multi_currency_ledger ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS ohc_shared_offer ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS ohc_staff_member ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS ohc_timecard_event ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS ohc_translation_preferences ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS ohc_universal_ledger ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS omni_inbox_messages ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS onboarding_state ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS opportunities ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS order_items ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS order_line_items ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS orders ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS pages ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS payment_events ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS po_line_items ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS pos_offline_transactions ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS pos_terminal_sessions ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS pricing_heuristics ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS product_variants ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS products ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS promotion_codes ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS purchase_orders ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS quotes ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS raw_materials ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS recovery_attempts ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS recovery_campaigns ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS referral_codes ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS referrals ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS reputation_profiles ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS reviews ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS revoked_tokens ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS roles ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS service_resource_requirements ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS services ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS shared_tasks ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS smart_pricing_policies ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS state_machine_transitions ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS sub_agent_queue ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS subscribers ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS subscription_plans ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS subscriptions ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS tasks ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS tenant_ai_budgets ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS travel_buffers ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS triage_items ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS triage_proposed_actions ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS users ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS vendors ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS team_invites ENABLE ROW LEVEL SECURITY;
INSERT INTO quotes (id, tenant_id, customer_id, status, total_amount_cents, currency, required_deposit_cents, stripe_payment_link, created_at, updated_at) VALUES
('823e4567-e89b-12d3-a456-426614174000', 'e2e-tenant', '648d7c4a-8f5b-4c3e-908f-7c6d5e4f3a2b', 'DRAFT', 15000, 'USD', 5000, NULL, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
ON CONFLICT DO NOTHING;

INSERT INTO quote_line_items (id, quote_id, description, unit_price_cents, quantity, is_optional, created_at, updated_at) VALUES
('823e4567-e89b-12d3-a456-426614174001', '823e4567-e89b-12d3-a456-426614174000', 'Fix leaking sink including labor and standard materials', 15000, 1, false, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
ON CONFLICT DO NOTHING;
INSERT INTO products (id, tenant_id, title, description, product_type, price, price_cents, currency, inventory_count, metadata)
VALUES
  ('e2e-product-cake-pos', 'e2e-tenant-pos', 'POS Sync Product', 'POS Sync Product', 'physical', 10.00, 1000, 'USD', 1, '{"seeded_by":"e2e"}'::jsonb),
  ('e2e-product-cake-pos-additional', 'e2e-tenant-pos-additional', 'POS Additional', 'POS Additional', 'physical', 10.00, 1000, 'USD', 5, '{"seeded_by":"e2e"}'::jsonb);

INSERT INTO tenants (id, name, industry, status, skip_onboarding, default_currency)
VALUES
  ('e2e-tenant-pos', 'OHC E2E Bakery POS', 'Food and beverage', 'active', true, 'USD'),
  ('e2e-tenant-pos-additional', 'OHC E2E Bakery POS Add', 'Food and beverage', 'active', true, 'USD');
INSERT INTO products (id, tenant_id, title, description, product_type, price, price_cents, currency, inventory_count, metadata)
VALUES
  ('e2e-product-pos-sync', 'e2e-tenant', 'POS Sync Product', 'POS Sync Product', 'physical', 10.00, 1000, 'USD', 1, '{"seeded_by":"e2e"}'::jsonb);
-- Add a 40.02 product to pos e2e tenant
INSERT INTO products (id, tenant_id, title, description, product_type, price, price_cents, currency, inventory_count, metadata)
VALUES
  ('e2e-product-4002-pos', 'e2e-tenant', 'POS Fail Product', 'POS Fail Product', 'physical', 40.02, 4002, 'USD', 100, '{"seeded_by":"e2e"}'::jsonb);
INSERT INTO business_milestones (id, tenant_id, milestone_type, reached_at) VALUES
('ms_e2e_revenue_1k', 'e2e-tenant', 'revenue_1k', CURRENT_TIMESTAMP)
ON CONFLICT DO NOTHING;

INSERT INTO business_milestones (id, tenant_id, milestone_type, reached_at) VALUES
('ms_e2e_revenue_10k', 'e2e-milestone-tenant', 'revenue_10k', CURRENT_TIMESTAMP)
ON CONFLICT DO NOTHING;
UPDATE tenants SET tier = 'Starter' WHERE id = 'e2e-tenant';
INSERT INTO tenants (id, name, industry, tier, has_claimed_trial_extension, default_currency)
VALUES
  ('e2e-tenant-free', 'OHC E2E Free Bakery', 'Food and beverage', 'Free', false, 'USD'),
  ('e2e-tenant-business', 'OHC E2E Business Bakery', 'Food and beverage', 'Business', false, 'USD')
ON CONFLICT (id) DO UPDATE
SET name = EXCLUDED.name,
    industry = EXCLUDED.industry,
    tier = EXCLUDED.tier,
    has_claimed_trial_extension = EXCLUDED.has_claimed_trial_extension,
    updated_at = CURRENT_TIMESTAMP;


INSERT INTO users (id, username, email, password_hash, roles, active, tenant_id, created_at, updated_at)
VALUES
  (
    'e2e-starter-user',
    'starter@example.com',
    'starter@example.com',
    '$2b$10$hmVhunI7Fq2ZzQ0PguAH5OeXUyb/gNAORUpLPD2g44Ik9/Fd9sM7a',
    ARRAY['ADMIN'],
    TRUE,
    'e2e-tenant',
    CURRENT_TIMESTAMP,
    CURRENT_TIMESTAMP
  ),
  (
    'e2e-business-user',
    'business@example.com',
    'business@example.com',
    '$2b$10$hmVhunI7Fq2ZzQ0PguAH5OeXUyb/gNAORUpLPD2g44Ik9/Fd9sM7a',
    ARRAY['ADMIN'],
    TRUE,
    'e2e-tenant-business',
    CURRENT_TIMESTAMP,
    CURRENT_TIMESTAMP
  ),
  (
    'e2e-pro-user',
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
    updated_at = EXCLUDED.updated_at;
INSERT INTO business_milestones (id, tenant_id, milestone_type, reached_at)
VALUES ('m-e2e-1', 'e2e-tenant', 'first_sale', CURRENT_TIMESTAMP)
ON CONFLICT DO NOTHING;

INSERT INTO telemetry_buffer (tenant_id, metric_name, metric_type, value, labels_json, timestamp, sync_status) VALUES
('e2e-tenant', 'ohc_llm_cost_total_cents', 'gauge', 200000, '{"agent_id": "agent_test_high_usage"}', CURRENT_TIMESTAMP, 'PENDING');

ALTER TABLE IF EXISTS service_routes DISABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS job_locations DISABLE ROW LEVEL SECURITY;

INSERT INTO service_routes (id, tenant_id, staff_id, route_date, status)
VALUES
  ('e2e-route-1', 'e2e-tenant', 'e2e-admin-user', CURRENT_DATE, 'planned')
ON CONFLICT (id) DO NOTHING;

INSERT INTO job_locations (id, tenant_id, service_route_id, customer_id, job_title, address, lat, lng, scheduled_start, scheduled_end, status, order_index)
VALUES
  ('e2e-job-1', 'e2e-tenant', 'e2e-route-1', 'e2e-customer-ava', 'Fix leaking sink', '123 Main St', 37.7749, -122.4194, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP + interval '1 hour', 'pending', 0),
  ('e2e-job-2', 'e2e-tenant', 'e2e-route-1', 'e2e-customer-ben', 'Roof repair estimate', '456 Oak Ave', 37.7849, -122.4294, CURRENT_TIMESTAMP + interval '2 hours', CURRENT_TIMESTAMP + interval '3 hours', 'pending', 1)
ON CONFLICT (id) DO NOTHING;

ALTER TABLE IF EXISTS job_locations ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS service_routes ENABLE ROW LEVEL SECURITY;

INSERT INTO agent_feed_items (id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state, created_at, updated_at)
VALUES
('e2e-feed-ops-daily-routine', 'e2e-tenant', 'Operations Agent', '{"feature_type": "daily_prep_checklist", "description": "Daily Prep Checklist"}'::jsonb, '{"action_type": "Daily Prep Checklist", "message": "Review Daily Prep Checklist"}'::jsonb, 'PENDING_APPROVAL', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
ON CONFLICT (id) DO UPDATE
SET lifecycle_state = EXCLUDED.lifecycle_state,
    updated_at = CURRENT_TIMESTAMP;

INSERT INTO agent_feed_items (id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state, created_at, updated_at)
VALUES ('req_replenish_123', 'e2e-tenant', 'system', '{"feature_type": "subscription_replenishment", "customer_name": "Maya Baker"}'::jsonb, '{"action_type": "email", "context": "Based on this customer''s order history and the estimated consumption rate, they are due for a replenishment. Would you like me to generate a personalized checkout link and draft an email suggesting they refill?"}'::jsonb, 'PENDING_APPROVAL', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
ON CONFLICT (id) DO UPDATE SET lifecycle_state = EXCLUDED.lifecycle_state;

-- Tenant must be created first
INSERT INTO tenants (id, name, industry, status, skip_onboarding, default_currency)
VALUES
  ('11111111-1111-1111-1111-111111111111', 'OHC E2E Edge Cache Tenant', 'Retail', 'active', true, 'USD')
ON CONFLICT (id) DO NOTHING;

-- Now add products and sites
INSERT INTO products (id, tenant_id, title, description, product_type, price, price_cents, currency, inventory_count, metadata)
VALUES
  ('22222222-2222-2222-2222-222222222222', '11111111-1111-1111-1111-111111111111', 'Edge Cached Product E2E', 'Edge Cached Product E2E', 'physical', 42.00, 4200, 'USD', 100, '{"seeded_by":"e2e"}'::jsonb)
ON CONFLICT (id) DO NOTHING;

INSERT INTO builder_sites (id, tenant_id, domain, published_at)
VALUES
  ('33333333-3333-3333-3333-333333333333', '11111111-1111-1111-1111-111111111111', 'edge-e2e.ohc.store', CURRENT_TIMESTAMP)
ON CONFLICT (id) DO NOTHING;
