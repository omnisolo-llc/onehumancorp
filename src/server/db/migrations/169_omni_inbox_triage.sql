-- Create omni_messages table for Universal Omnichannel Inbox
CREATE TABLE IF NOT EXISTS omni_messages (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    source VARCHAR(255) NOT NULL, -- e.g., 'instagram_dm', 'whatsapp', 'email', 'sms'
    source_message_id VARCHAR(255) NOT NULL,
    sender_id VARCHAR(255),
    content TEXT NOT NULL,
    intent VARCHAR(255),
    urgency VARCHAR(50),
    status VARCHAR(50) DEFAULT 'pending',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_omni_messages_tenant_id ON omni_messages(tenant_id);
CREATE INDEX idx_omni_messages_status ON omni_messages(status);

ALTER TABLE omni_messages ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_omni_messages ON omni_messages
    USING (tenant_id = current_setting('app.current_tenant', true)::uuid);

-- Create triage_queue table
CREATE TABLE IF NOT EXISTS triage_queue (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    message_id UUID NOT NULL REFERENCES omni_messages(id) ON DELETE CASCADE,
    assigned_department VARCHAR(255),
    confidence_score FLOAT,
    status VARCHAR(50) DEFAULT 'queued',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_triage_queue_tenant_id ON triage_queue(tenant_id);
CREATE INDEX idx_triage_queue_status ON triage_queue(status);

ALTER TABLE triage_queue ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_triage_queue ON triage_queue
    USING (tenant_id = current_setting('app.current_tenant', true)::uuid);
