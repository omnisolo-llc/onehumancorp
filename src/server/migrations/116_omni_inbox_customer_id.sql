-- Migration 115: Add customer_id to omni_inbox_messages

ALTER TABLE omni_inbox_messages
ADD COLUMN IF NOT EXISTS customer_id TEXT REFERENCES customers(id) ON DELETE SET NULL;
