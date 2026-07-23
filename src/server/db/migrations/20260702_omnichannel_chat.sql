-- +goose Up
-- Create Omnichannel Chat Engine Tables

CREATE TABLE IF NOT EXISTS chat_inbox (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    name TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

ALTER TABLE chat_inbox ENABLE ROW LEVEL SECURITY;
CREATE POLICY "tenant_isolation_chat_inbox_select" ON chat_inbox FOR SELECT USING (tenant_id = current_setting('app.current_tenant', true));
CREATE POLICY "tenant_isolation_chat_inbox_insert" ON chat_inbox FOR INSERT WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
CREATE POLICY "tenant_isolation_chat_inbox_update" ON chat_inbox FOR UPDATE USING (tenant_id = current_setting('app.current_tenant', true));
CREATE POLICY "tenant_isolation_chat_inbox_delete" ON chat_inbox FOR DELETE USING (tenant_id = current_setting('app.current_tenant', true));

CREATE TABLE IF NOT EXISTS chat_contact (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    name TEXT,
    email TEXT,
    phone TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

ALTER TABLE chat_contact ENABLE ROW LEVEL SECURITY;
CREATE POLICY "tenant_isolation_chat_contact_select" ON chat_contact FOR SELECT USING (tenant_id = current_setting('app.current_tenant', true));
CREATE POLICY "tenant_isolation_chat_contact_insert" ON chat_contact FOR INSERT WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
CREATE POLICY "tenant_isolation_chat_contact_update" ON chat_contact FOR UPDATE USING (tenant_id = current_setting('app.current_tenant', true));
CREATE POLICY "tenant_isolation_chat_contact_delete" ON chat_contact FOR DELETE USING (tenant_id = current_setting('app.current_tenant', true));

CREATE TABLE IF NOT EXISTS chat_conversation (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    inbox_id TEXT NOT NULL REFERENCES chat_inbox(id) ON DELETE CASCADE,
    contact_id TEXT NOT NULL REFERENCES chat_contact(id) ON DELETE CASCADE,
    status TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

ALTER TABLE chat_conversation ENABLE ROW LEVEL SECURITY;
CREATE POLICY "tenant_isolation_chat_conversation_select" ON chat_conversation FOR SELECT USING (tenant_id = current_setting('app.current_tenant', true));
CREATE POLICY "tenant_isolation_chat_conversation_insert" ON chat_conversation FOR INSERT WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
CREATE POLICY "tenant_isolation_chat_conversation_update" ON chat_conversation FOR UPDATE USING (tenant_id = current_setting('app.current_tenant', true));
CREATE POLICY "tenant_isolation_chat_conversation_delete" ON chat_conversation FOR DELETE USING (tenant_id = current_setting('app.current_tenant', true));

CREATE TABLE IF NOT EXISTS chat_message (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    conversation_id TEXT NOT NULL REFERENCES chat_conversation(id) ON DELETE CASCADE,
    content TEXT NOT NULL,
    sender_type TEXT NOT NULL,
    source TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

ALTER TABLE chat_message ENABLE ROW LEVEL SECURITY;
CREATE POLICY "tenant_isolation_chat_message_select" ON chat_message FOR SELECT USING (tenant_id = current_setting('app.current_tenant', true));
CREATE POLICY "tenant_isolation_chat_message_insert" ON chat_message FOR INSERT WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
CREATE POLICY "tenant_isolation_chat_message_update" ON chat_message FOR UPDATE USING (tenant_id = current_setting('app.current_tenant', true));
CREATE POLICY "tenant_isolation_chat_message_delete" ON chat_message FOR DELETE USING (tenant_id = current_setting('app.current_tenant', true));
