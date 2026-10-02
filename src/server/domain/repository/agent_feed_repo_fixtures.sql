
CREATE TEMP TABLE agent_feed_items (id TEXT, tenant_id TEXT, created_at TIMESTAMPTZ, updated_at TIMESTAMPTZ, event_source TEXT, context_payload JSONB, proposed_action JSONB, lifecycle_state TEXT);
CREATE TEMP TABLE agent_approvals (id TEXT, tenant_id TEXT, created_at TIMESTAMPTZ, updated_at TIMESTAMPTZ, department TEXT, description TEXT, payload JSONB, status TEXT);
CREATE TEMP TABLE agent_action_requests (id TEXT, tenant_id TEXT, created_at TIMESTAMPTZ, updated_at TIMESTAMPTZ, agent_type TEXT, department_type TEXT, description TEXT, action_type TEXT, payload JSONB, status TEXT);
CREATE TEMP TABLE omni_inbox_messages (id TEXT, tenant_id TEXT, created_at TIMESTAMPTZ, updated_at TIMESTAMPTZ, source TEXT, original_content TEXT, draft_reply TEXT, status TEXT);
CREATE TEMP TABLE orders (id TEXT, tenant_id TEXT, created_at TIMESTAMPTZ, updated_at TIMESTAMPTZ, status TEXT);
CREATE TEMP TABLE invoices (id TEXT, tenant_id TEXT, created_at TIMESTAMPTZ, updated_at TIMESTAMPTZ, status TEXT);
INSERT INTO agent_feed_items VALUES
('mirrored','owner','2026-01-03T00:00:00Z','2026-01-03T00:00:00Z','canonical','{"description":"Reviewed"}','{"amount":12}','APPROVED'),
('cross-tenant-id','other','2026-01-04T00:00:00Z','2026-01-04T00:00:00Z','private','{}','{}','PENDING_APPROVAL');
INSERT INTO agent_action_requests VALUES
('mirrored','owner','2026-01-03T00:00:00Z','2026-01-03T00:00:00Z','legacy','operations','Stale pending copy','request','{"amount":9}','Pending'),
('standalone','owner','2026-01-02T00:00:00Z','2026-01-02T00:00:00Z','legacy','operations','Unmirrored action','request','{}','Pending'),
('cross-tenant-id','owner','2026-01-01T00:00:00Z','2026-01-01T00:00:00Z','legacy','operations','Owned action','request','{}','Pending'),
('private-request','other','2026-01-04T00:00:00Z','2026-01-04T00:00:00Z','private','operations','Private request','request','{}','Pending');
