-- +goose Up

CREATE TABLE IF NOT EXISTS conversations (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    customer_id UUID REFERENCES customer_profile(id),
    channel TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'open',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

ALTER TABLE conversations ENABLE ROW LEVEL SECURITY;
CREATE POLICY conversations_tenant_isolation_policy ON conversations FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);

CREATE TABLE IF NOT EXISTS messages (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    conversation_id UUID NOT NULL REFERENCES conversations(id) ON DELETE CASCADE,
    direction TEXT NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

ALTER TABLE messages ENABLE ROW LEVEL SECURITY;
CREATE POLICY messages_tenant_isolation_policy ON messages FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);

CREATE TABLE IF NOT EXISTS ai_drafts (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    message_id UUID NOT NULL REFERENCES messages(id) ON DELETE CASCADE,
    proposed_response TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

ALTER TABLE ai_drafts ENABLE ROW LEVEL SECURITY;
CREATE POLICY ai_drafts_tenant_isolation_policy ON ai_drafts FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);

-- +goose Down
DROP POLICY IF EXISTS ai_drafts_tenant_isolation_policy ON ai_drafts;
DROP TABLE IF EXISTS ai_drafts CASCADE;

DROP POLICY IF EXISTS messages_tenant_isolation_policy ON messages;
DROP TABLE IF EXISTS messages CASCADE;

DROP POLICY IF EXISTS conversations_tenant_isolation_policy ON conversations;
DROP TABLE IF EXISTS conversations CASCADE;
