INSERT INTO users (id, email, full_name, is_superadmin) VALUES ('test_id', 'test@example.com', 'Test User', false) ON CONFLICT DO NOTHING;
INSERT INTO tenants (id, name, owner_email) VALUES ('test_tenant', 'Test Store', 'test@example.com') ON CONFLICT DO NOTHING;
INSERT INTO carts (id, tenant_id, customer_id, channel, status, total_amount_cents, currency) VALUES ('test_cart', 'test_tenant', 'test_cust', 'online', 'abandoned', 8999, 'usd') ON CONFLICT DO NOTHING;
INSERT INTO abandoned_carts (id, tenant_id, cart_id, customer_email, items, status) VALUES ('test_ac', 'test_tenant', 'test_cart', 'abandoned@example.com', '[]'::jsonb, 'PENDING') ON CONFLICT DO NOTHING;
