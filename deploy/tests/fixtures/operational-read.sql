-- Only loaded by the just-created, project-identity-checked Compose test database.
-- This is fixture setup, never a production initialization or API implementation.
\set ON_ERROR_STOP on
BEGIN;
SELECT set_config('app.current_tenant', :'tenant_id', true);
INSERT INTO customers (id, tenant_id, name, email)
VALUES ('compose-fixture-customer', :'tenant_id', 'Compose fixture customer', 'compose-fixture@example.test');
INSERT INTO orders (id, tenant_id, customer_id, total_amount, status)
VALUES ('compose-fixture-order', :'tenant_id', 'compose-fixture-customer', 158.50, 'completed');
INSERT INTO inbox_messages
  (id, tenant_id, source, content, original_content, translated_from_language, draft_reply, status, sender_id, customer_id)
VALUES ('compose-fixture-inbox', :'tenant_id', 'Email', 'Deployment fixture inquiry', 'Deployment fixture inquiry', '', '', 'unread', 'compose-fixture-sender', 'compose-fixture-customer');
INSERT INTO vendors (id, tenant_id, name, contact_info)
VALUES ('compose-fixture-vendor', :'tenant_id', 'Compose fixture vendor', 'vendor@example.test');
COMMIT;
