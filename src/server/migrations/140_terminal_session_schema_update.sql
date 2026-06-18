-- +goose Up
ALTER TABLE pos_terminal_sessions ADD COLUMN sync_status TEXT DEFAULT 'SYNCED';
ALTER TABLE pos_terminal_sessions ADD COLUMN pending_reconciliation JSONB DEFAULT '[]'::jsonb;

-- +goose Down
ALTER TABLE pos_terminal_sessions DROP COLUMN IF EXISTS pending_reconciliation;
ALTER TABLE pos_terminal_sessions DROP COLUMN IF EXISTS sync_status;
