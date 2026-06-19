-- +goose Up
ALTER TABLE IF EXISTS pos_terminal_sessions ADD COLUMN IF NOT EXISTS sync_status TEXT DEFAULT 'SYNCED';
ALTER TABLE IF EXISTS pos_terminal_sessions ADD COLUMN IF NOT EXISTS pending_reconciliation JSONB DEFAULT '[]'::jsonb;

-- +goose Down
ALTER TABLE IF EXISTS pos_terminal_sessions DROP COLUMN IF EXISTS pending_reconciliation;
ALTER TABLE IF EXISTS pos_terminal_sessions DROP COLUMN IF EXISTS sync_status;
