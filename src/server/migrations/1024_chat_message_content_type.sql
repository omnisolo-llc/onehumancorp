-- Preserve the original chat migration identities and SQLx checksums. Existing
-- installations may already have this upstream column; retain their values.
ALTER TABLE chat_messages
    ADD COLUMN IF NOT EXISTS content_type VARCHAR(50) NOT NULL DEFAULT 'text';
