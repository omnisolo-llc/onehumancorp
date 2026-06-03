CREATE TABLE support_tickets (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id TEXT NOT NULL REFERENCES tenants(id),
    customer_id TEXT REFERENCES customers(id),
    channel VARCHAR(50) NOT NULL, -- 'instagram', 'whatsapp', 'sms', 'web'
    external_message_id VARCHAR(255),
    status VARCHAR(50) NOT NULL, -- 'open', 'draft', 'resolved'
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

ALTER TABLE support_tickets ENABLE ROW LEVEL SECURITY;

CREATE POLICY "Tenant isolation for support_tickets" ON support_tickets
    USING (tenant_id = current_setting('app.current_tenant_id')::TEXT);

CREATE TABLE ticket_messages (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    ticket_id UUID NOT NULL REFERENCES support_tickets(id),
    sender_type VARCHAR(50) NOT NULL, -- 'customer', 'ai', 'owner'
    content TEXT NOT NULL,
    ai_confidence DECIMAL(5,2),
    created_at TIMESTAMPTZ DEFAULT NOW()
);

ALTER TABLE ticket_messages ENABLE ROW LEVEL SECURITY;

CREATE POLICY "Tenant isolation for ticket_messages" ON ticket_messages
    USING (ticket_id IN (SELECT id FROM support_tickets WHERE tenant_id = current_setting('app.current_tenant_id')::TEXT));
