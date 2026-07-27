ALTER TABLE chat_messages ADD COLUMN IF NOT EXISTS status VARCHAR(50) DEFAULT 'pending_ai';
ALTER TABLE chat_messages ADD COLUMN IF NOT EXISTS draft_reply TEXT;
