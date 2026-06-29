-- +goose Up
-- Migration 107: Add fields to pos_terminal_sessions

ALTER TABLE pos_terminal_sessions ADD COLUMN IF NOT EXISTS pending_offline_syncs INT DEFAULT 0;
ALTER TABLE pos_terminal_sessions ADD COLUMN IF NOT EXISTS last_reconciled_at TIMESTAMPTZ;

-- +goose Down
ALTER TABLE pos_terminal_sessions DROP COLUMN IF EXISTS pending_offline_syncs;
ALTER TABLE pos_terminal_sessions DROP COLUMN IF EXISTS last_reconciled_at;
