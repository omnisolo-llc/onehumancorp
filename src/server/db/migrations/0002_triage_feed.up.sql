CREATE TABLE triage_items (
    item_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    priority INT NOT NULL,
    source_icon TEXT,
    customer_name TEXT,
    summary TEXT,
    source_event_type TEXT,
    source_event_id TEXT,
    source_payload_json JSONB,
    suggested_actions JSONB DEFAULT '[]'::JSONB,
    agent_draft_id TEXT,
    agent_draft_content TEXT,
    agent_context_summary TEXT,
    is_resolved BOOLEAN DEFAULT false,
    created_at_unix BIGINT NOT NULL
);

ALTER TABLE triage_items ENABLE ROW LEVEL SECURITY;
CREATE POLICY triage_items_tenant_isolation ON triage_items FOR ALL USING (tenant_id = current_setting('app.current_tenant_id')::UUID);
