ALTER TABLE chat_inboxes
    ADD COLUMN IF NOT EXISTS enable_auto_assignment BOOLEAN DEFAULT FALSE,
    ADD COLUMN IF NOT EXISTS greeting_message TEXT,
    ADD COLUMN IF NOT EXISTS working_hours_enabled BOOLEAN DEFAULT FALSE;

ALTER TABLE chat_contacts
    ADD COLUMN IF NOT EXISTS custom_attributes JSONB DEFAULT '{}'::jsonb;

ALTER TABLE chat_conversations
    ADD COLUMN IF NOT EXISTS priority INTEGER DEFAULT 0;

ALTER TABLE chat_messages
    ADD COLUMN IF NOT EXISTS content_type TEXT DEFAULT 'text',
    ADD COLUMN IF NOT EXISTS additional_attributes JSONB DEFAULT '{}'::jsonb;
