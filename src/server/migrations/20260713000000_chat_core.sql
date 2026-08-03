CREATE TABLE inboxes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    name VARCHAR(255) NOT NULL,
    channel_type VARCHAR(50) NOT NULL,
    settings JSONB NOT NULL DEFAULT '{}'::jsonb
);

ALTER TABLE inboxes ENABLE ROW LEVEL SECURITY;

CREATE TABLE contacts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255),
    phone VARCHAR(50),
    external_id VARCHAR(255)
);

ALTER TABLE contacts ENABLE ROW LEVEL SECURITY;

CREATE TABLE conversations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    inbox_id UUID NOT NULL REFERENCES inboxes(id),
    contact_id UUID NOT NULL REFERENCES contacts(id),
    status VARCHAR(50) NOT NULL DEFAULT 'open',
    assignee_id UUID
);

ALTER TABLE conversations ENABLE ROW LEVEL SECURITY;

CREATE TABLE messages (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    conversation_id UUID NOT NULL REFERENCES conversations(id),
    sender_type VARCHAR(50) NOT NULL,
    content TEXT NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'sent'
);

ALTER TABLE messages ENABLE ROW LEVEL SECURITY;
