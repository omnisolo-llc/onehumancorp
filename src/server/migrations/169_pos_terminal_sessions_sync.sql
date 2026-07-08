-- +goose Up
-- Migration 169: Add sync_status and pending_reconciliation to pos_terminal_sessions

ALTER TABLE pos_terminal_sessions ADD COLUMN IF NOT EXISTS sync_status TEXT DEFAULT 'OK';
ALTER TABLE pos_terminal_sessions ADD COLUMN IF NOT EXISTS pending_reconciliation JSONB DEFAULT '[]'::jsonb;

-- +goose Down
ALTER TABLE pos_terminal_sessions DROP COLUMN IF EXISTS sync_status;
ALTER TABLE pos_terminal_sessions DROP COLUMN IF EXISTS pending_reconciliation;
