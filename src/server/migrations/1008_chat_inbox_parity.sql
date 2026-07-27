ALTER TABLE chat_inboxes
    ADD COLUMN IF NOT EXISTS working_hours_enabled BOOLEAN DEFAULT false,
    ADD COLUMN IF NOT EXISTS out_of_office_message TEXT,
    ADD COLUMN IF NOT EXISTS greeting_enabled BOOLEAN DEFAULT false,
    ADD COLUMN IF NOT EXISTS greeting_message TEXT,
    ADD COLUMN IF NOT EXISTS csat_survey_enabled BOOLEAN DEFAULT false,
    ADD COLUMN IF NOT EXISTS auto_assignment_config JSONB DEFAULT '{}'::jsonb;

ALTER TABLE chat_conversations
    ADD COLUMN IF NOT EXISTS bot_assignee_id UUID,
    ADD COLUMN IF NOT EXISTS waiting_since TIMESTAMPTZ;
