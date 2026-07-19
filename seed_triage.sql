BEGIN;
ALTER TABLE triage_items DISABLE ROW LEVEL SECURITY;
ALTER TABLE triage_proposed_actions DISABLE ROW LEVEL SECURITY;

INSERT INTO triage_items (id, tenant_id, customer_id, source, priority, context, status)
VALUES
  ('e2e-triage-1', 'e2e-tenant', 'Ada Baker', 'Instagram DM', 'high', 'Do you have vegan chocolate cake available this weekend?', 'pending'),
  ('e2e-triage-2', 'e2e-tenant', 'System', 'Inventory Alert', 'urgent', 'Flour is running low.', 'pending'),
  ('e2e-triage-3', 'e2e-tenant', 'John Doe', 'Website Booking', 'medium', 'Wants an estimate tomorrow at 2PM.', 'pending')
ON CONFLICT (id) DO UPDATE
SET tenant_id = EXCLUDED.tenant_id,
    customer_id = EXCLUDED.customer_id,
    source = EXCLUDED.source,
    priority = EXCLUDED.priority,
    context = EXCLUDED.context,
    status = EXCLUDED.status;

INSERT INTO triage_proposed_actions (id, triage_item_id, tenant_id, action_type, payload)
VALUES
  ('e2e-triage-act-1', 'e2e-triage-1', 'e2e-tenant', 'Draft Reply', 'Yes, we have vegan celebration cakes available for this weekend. Would you like me to send a payment link?'),
  ('e2e-triage-act-2', 'e2e-triage-2', 'e2e-tenant', 'Reorder', 'Order 50lbs of AP flour from supplier.'),
  ('e2e-triage-act-3', 'e2e-triage-3', 'e2e-tenant', 'Accept Booking', 'Booked for 2PM tomorrow.')
ON CONFLICT (id) DO UPDATE
SET triage_item_id = EXCLUDED.triage_item_id,
    tenant_id = EXCLUDED.tenant_id,
    action_type = EXCLUDED.action_type,
    payload = EXCLUDED.payload;

ALTER TABLE triage_items ENABLE ROW LEVEL SECURITY;
ALTER TABLE triage_proposed_actions ENABLE ROW LEVEL SECURITY;
ALTER TABLE triage_items FORCE ROW LEVEL SECURITY;
ALTER TABLE triage_proposed_actions FORCE ROW LEVEL SECURITY;
COMMIT;
