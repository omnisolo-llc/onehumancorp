-- +goose Up
ALTER TABLE pos_terminal_sessions ADD COLUMN IF NOT EXISTS sync_status TEXT DEFAULT 'ACTIVE';
ALTER TABLE pos_terminal_sessions ADD COLUMN IF NOT EXISTS pending_reconciliation JSONB DEFAULT '[]';

-- +goose Down
ALTER TABLE pos_terminal_sessions DROP COLUMN IF EXISTS sync_status;
ALTER TABLE pos_terminal_sessions DROP COLUMN IF EXISTS pending_reconciliation;
