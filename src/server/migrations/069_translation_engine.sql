ALTER TABLE tenants ADD COLUMN IF NOT EXISTS preferred_language TEXT DEFAULT 'en';
ALTER TABLE customers ADD COLUMN IF NOT EXISTS preferred_language TEXT DEFAULT 'en';
ALTER TABLE inbox_messages ADD COLUMN IF NOT EXISTS original_content TEXT;
