CREATE TABLE IF NOT EXISTS chat_macros (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    name TEXT NOT NULL,
    visibility TEXT NOT NULL DEFAULT 'personal', -- 'personal' or 'global'
    actions JSONB NOT NULL DEFAULT '[]'::jsonb,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
ALTER TABLE chat_macros ENABLE ROW LEVEL SECURITY;
CREATE POLICY chat_macros_tenant_isolation_policy ON chat_macros FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);

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
