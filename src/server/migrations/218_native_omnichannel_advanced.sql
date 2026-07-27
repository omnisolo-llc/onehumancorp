CREATE TABLE IF NOT EXISTS chat_labels (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    color TEXT NOT NULL DEFAULT '#000000',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
ALTER TABLE chat_labels ENABLE ROW LEVEL SECURITY;
CREATE POLICY chat_labels_tenant_isolation_policy ON chat_labels FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);

CREATE TABLE IF NOT EXISTS chat_canned_responses (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    short_code TEXT NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
ALTER TABLE chat_canned_responses ENABLE ROW LEVEL SECURITY;
CREATE POLICY chat_canned_responses_tenant_isolation_policy ON chat_canned_responses FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);

CREATE TABLE IF NOT EXISTS chat_macros (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    name TEXT NOT NULL,
    actions JSONB NOT NULL DEFAULT '[]'::jsonb,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
ALTER TABLE chat_macros ENABLE ROW LEVEL SECURITY;
CREATE POLICY chat_macros_tenant_isolation_policy ON chat_macros FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);

CREATE TABLE IF NOT EXISTS chat_routing_rules (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    name TEXT NOT NULL,
    conditions JSONB NOT NULL DEFAULT '[]'::jsonb,
    actions JSONB NOT NULL DEFAULT '[]'::jsonb,
    priority INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
ALTER TABLE chat_routing_rules ENABLE ROW LEVEL SECURITY;
CREATE POLICY chat_routing_rules_tenant_isolation_policy ON chat_routing_rules FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);

CREATE TABLE IF NOT EXISTS chat_sla_policies (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    name TEXT NOT NULL,
    first_response_time_threshold INTEGER NOT NULL,
    resolution_time_threshold INTEGER NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
ALTER TABLE chat_sla_policies ENABLE ROW LEVEL SECURITY;
CREATE POLICY chat_sla_policies_tenant_isolation_policy ON chat_sla_policies FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);
