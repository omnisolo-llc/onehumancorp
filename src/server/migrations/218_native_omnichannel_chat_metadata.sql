ALTER TABLE chat_messages
ADD COLUMN IF NOT EXISTS content_attributes JSONB DEFAULT '{}'::jsonb,
ADD COLUMN IF NOT EXISTS external_source_ids JSONB DEFAULT '[]'::jsonb;
