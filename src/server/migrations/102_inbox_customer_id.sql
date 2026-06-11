-- +goose Up
-- Migration 102: Add customer_id to inbox_messages

ALTER TABLE inbox_messages
ADD COLUMN IF NOT EXISTS customer_id TEXT;

-- +goose Down
ALTER TABLE inbox_messages
DROP COLUMN IF EXISTS customer_id;
