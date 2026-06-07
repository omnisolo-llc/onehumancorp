ALTER TABLE inbox_messages
ADD COLUMN IF NOT EXISTS original_content TEXT;

ALTER TABLE inbox_messages
ADD COLUMN IF NOT EXISTS translated_from_language TEXT;
