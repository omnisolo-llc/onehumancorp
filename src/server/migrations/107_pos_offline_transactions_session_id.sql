-- +goose Up
-- Migration 107: Add session_id to pos_offline_transactions

ALTER TABLE pos_offline_transactions ADD COLUMN IF NOT EXISTS session_id TEXT;

-- +goose Down
ALTER TABLE pos_offline_transactions DROP COLUMN IF EXISTS session_id;
