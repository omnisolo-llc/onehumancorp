CREATE TABLE IF NOT EXISTS chat_sla_policies (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    name TEXT NOT NULL,
    first_response_time_seconds INTEGER NOT NULL,
    resolution_time_seconds INTEGER NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
ALTER TABLE chat_sla_policies ENABLE ROW LEVEL SECURITY;
CREATE POLICY chat_sla_policies_tenant_isolation_policy ON chat_sla_policies FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);

CREATE TABLE IF NOT EXISTS chat_automation_rules (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    name TEXT NOT NULL,
    trigger_event TEXT NOT NULL,
    conditions JSONB NOT NULL DEFAULT '[]'::jsonb,
    actions JSONB NOT NULL DEFAULT '[]'::jsonb,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
ALTER TABLE chat_automation_rules ENABLE ROW LEVEL SECURITY;
CREATE POLICY chat_automation_rules_tenant_isolation_policy ON chat_automation_rules FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);

CREATE TABLE IF NOT EXISTS chat_canned_responses (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    short_code TEXT NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(tenant_id, short_code)
);
ALTER TABLE chat_canned_responses ENABLE ROW LEVEL SECURITY;
CREATE POLICY chat_canned_responses_tenant_isolation_policy ON chat_canned_responses FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);
