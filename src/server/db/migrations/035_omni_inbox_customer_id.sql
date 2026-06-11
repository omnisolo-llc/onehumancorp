-- +goose Up
-- Migration 035: Add customer_id to omni_inbox_messages

ALTER TABLE omni_inbox_messages
ADD COLUMN IF NOT EXISTS customer_id TEXT;

ALTER TABLE inbox_messages
ADD COLUMN IF NOT EXISTS customer_id TEXT;

-- +goose Down
ALTER TABLE omni_inbox_messages
DROP COLUMN IF EXISTS customer_id;

ALTER TABLE inbox_messages
DROP COLUMN IF EXISTS customer_id;
