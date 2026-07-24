-- Create native chat schema to replace external chat service

CREATE TABLE custom_inboxes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    name VARCHAR(255) NOT NULL,
    channel_type VARCHAR(50) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE custom_contacts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255),
    phone_number VARCHAR(50),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE custom_conversations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    inbox_id UUID NOT NULL REFERENCES custom_inboxes(id),
    contact_id UUID NOT NULL REFERENCES custom_contacts(id),
    status VARCHAR(50) DEFAULT 'open',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE custom_messages (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    conversation_id UUID NOT NULL REFERENCES custom_conversations(id),
    content TEXT NOT NULL,
    message_type VARCHAR(50) NOT NULL, -- e.g., 'incoming', 'outgoing'
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Enable RLS
ALTER TABLE custom_inboxes ENABLE ROW LEVEL SECURITY;
ALTER TABLE custom_contacts ENABLE ROW LEVEL SECURITY;
ALTER TABLE custom_conversations ENABLE ROW LEVEL SECURITY;
ALTER TABLE custom_messages ENABLE ROW LEVEL SECURITY;

-- Create policies
CREATE POLICY tenant_isolation_custom_inboxes ON custom_inboxes FOR ALL USING (tenant_id = current_setting('app.current_tenant')::UUID);
CREATE POLICY tenant_isolation_custom_contacts ON custom_contacts FOR ALL USING (tenant_id = current_setting('app.current_tenant')::UUID);
CREATE POLICY tenant_isolation_custom_conversations ON custom_conversations FOR ALL USING (tenant_id = current_setting('app.current_tenant')::UUID);
CREATE POLICY tenant_isolation_custom_messages ON custom_messages FOR ALL USING (tenant_id = current_setting('app.current_tenant')::UUID);
