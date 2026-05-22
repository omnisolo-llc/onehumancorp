BEGIN;

ALTER TABLE IF EXISTS agent_approvals DISABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS meeting_transcripts DISABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS meeting_rooms DISABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS agent_inbox DISABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS shared_tasks DISABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS orders DISABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS customers DISABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS products DISABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS agents DISABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS users DISABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS tenants DISABLE ROW LEVEL SECURITY;

INSERT INTO tenants (id, name, industry, tier)
VALUES ('e2e-tenant', 'OHC E2E Bakery', 'Food and beverage', 'starter')
ON CONFLICT (id) DO UPDATE
SET name = EXCLUDED.name,
    industry = EXCLUDED.industry,
    tier = EXCLUDED.tier,
    updated_at = CURRENT_TIMESTAMP;

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
  )
ON CONFLICT (id) DO UPDATE
SET username = EXCLUDED.username,
    email = EXCLUDED.email,
    password_hash = EXCLUDED.password_hash,
    roles = EXCLUDED.roles,
    active = EXCLUDED.active,
    tenant_id = EXCLUDED.tenant_id,
    updated_at = CURRENT_TIMESTAMP;

INSERT INTO agent_approvals (id, tenant_id, department, description, status, action_risk, feature_type, created_at, updated_at)
VALUES
('e2e-approval-1', 'e2e-tenant', 'customer_success', 'Draft email for review', 'PENDING', 'HIGH', NULL, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
('mock-1', 'e2e-tenant', 'customer_success', 'Test request', 'PENDING', 'HIGH', NULL, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
('mock-2', 'e2e-tenant', 'operations', 'Another request', 'PENDING', 'LOW', NULL, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
('mock-mkt-1', 'e2e-tenant', 'marketing', 'Global Reach: Translate your storefront to Spanish and show local currency for customers in Latin America?', 'PENDING', 'MEDIUM', 'global_localization', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
('mock-mkt-2', 'e2e-tenant', 'marketing', 'Smart Search Setup: Make your store more visible to customers using AI search tools?', 'PENDING', 'LOW', 'ai_geo', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
('mock-risk-high', 'e2e-tenant', 'legal', 'High Risk Action', 'PENDING', 'HIGH', NULL, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
('mock-risk-low', 'e2e-tenant', 'legal', 'Low Risk Action', 'PENDING', 'LOW', NULL, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
('mock-legal-1', 'e2e-tenant', 'legal', 'Action Required: Your sales are approaching the EU tax limit. Should we update your tax and privacy policies to keep you compliant?', 'PENDING', 'HIGH', 'legal_compliance', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
('mock-legal-2', 'e2e-tenant', 'legal', 'Action Required: Your sales are approaching the EU tax limit. Should we update your tax and privacy policies to keep you compliant?', 'PENDING', 'HIGH', 'legal_compliance', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
('e2e-approval-mock-1', 'e2e-tenant', 'customer_success', 'Draft email for review: Maya ordered a vegan cake', 'PENDING', 'HIGH', NULL, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
('e2e-approval-mock-2', 'e2e-tenant', 'marketing', 'Draft Instagram Post: New vegan cakes available!', 'PENDING', 'LOW', NULL, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
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

ALTER TABLE IF EXISTS tenants ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS users ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS agents ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS products ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS customers ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS orders ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS shared_tasks ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS agent_inbox ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS meeting_rooms ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS meeting_transcripts ENABLE ROW LEVEL SECURITY;

ALTER TABLE IF EXISTS tenants FORCE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS users FORCE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS agents FORCE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS products FORCE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS customers FORCE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS orders FORCE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS shared_tasks FORCE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS agent_inbox FORCE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS meeting_rooms FORCE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS meeting_transcripts FORCE ROW LEVEL SECURITY;

COMMIT;
