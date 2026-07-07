-- +goose Up
ALTER TABLE pos_terminal_sessions ADD COLUMN IF NOT EXISTS sync_status TEXT DEFAULT 'SYNCED';
ALTER TABLE pos_terminal_sessions ADD COLUMN IF NOT EXISTS pending_reconciliation JSONB DEFAULT '[]'::jsonb;
ALTER TABLE pos_terminal_sessions ADD COLUMN IF NOT EXISTS last_conflict_resolved_at TIMESTAMPTZ;

-- +goose Down
ALTER TABLE pos_terminal_sessions DROP COLUMN IF EXISTS sync_status;
ALTER TABLE pos_terminal_sessions DROP COLUMN IF EXISTS pending_reconciliation;
ALTER TABLE pos_terminal_sessions DROP COLUMN IF EXISTS last_conflict_resolved_at;
