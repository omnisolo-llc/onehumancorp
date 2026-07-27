ALTER TABLE chat_inboxes ADD COLUMN IF NOT EXISTS working_hours_enabled BOOLEAN DEFAULT FALSE;
ALTER TABLE chat_inboxes ADD COLUMN IF NOT EXISTS out_of_office_message TEXT;
ALTER TABLE chat_inboxes ADD COLUMN IF NOT EXISTS greeting_enabled BOOLEAN DEFAULT FALSE;
ALTER TABLE chat_inboxes ADD COLUMN IF NOT EXISTS greeting_message TEXT;

ALTER TABLE chat_conversations ADD COLUMN IF NOT EXISTS bot_assignee_id UUID;
