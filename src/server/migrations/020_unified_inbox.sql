-- 19866: Unified Omnichannel AI Ambassador Inbox tables
CREATE TABLE IF NOT EXISTS unified_thread (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    merchant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    customer_id TEXT, -- nullable for now
    subject TEXT,
    requires_human_attention BOOLEAN DEFAULT false,
    last_activity_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS unified_message (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    thread_id UUID NOT NULL REFERENCES unified_thread(id) ON DELETE CASCADE,
    channel TEXT NOT NULL, -- e.g. IG_DM, WHATSAPP, SMS, EMAIL
    external_message_id TEXT,
    direction TEXT NOT NULL, -- INBOUND, OUTBOUND
    sender_type TEXT NOT NULL, -- CUSTOMER, HUMAN_MERCHANT, AI_AMBASSADOR
    body TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- RLS policies for multi-tenant isolation
ALTER TABLE unified_thread ENABLE ROW LEVEL SECURITY;
CREATE POLICY unified_thread_tenant_isolation ON unified_thread
    FOR ALL
    USING (merchant_id = current_setting('app.current_tenant', true));

ALTER TABLE unified_message ENABLE ROW LEVEL SECURITY;
CREATE POLICY unified_message_tenant_isolation ON unified_message
    FOR ALL
    USING (
        thread_id IN (
            SELECT id FROM unified_thread WHERE merchant_id = current_setting('app.current_tenant', true)
        )
    );
