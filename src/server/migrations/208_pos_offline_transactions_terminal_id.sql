-- +goose Up
-- Migration 208: Add terminal_id to pos_offline_transactions table

ALTER TABLE pos_offline_transactions ADD COLUMN IF NOT EXISTS terminal_id TEXT;
ALTER TABLE pos_offline_transactions ADD COLUMN IF NOT EXISTS device_signature TEXT;

-- +goose Down
ALTER TABLE pos_offline_transactions DROP COLUMN IF EXISTS terminal_id;
-- Note: device_signature is omitted from down migration on purpose since it seems to exist in other places
