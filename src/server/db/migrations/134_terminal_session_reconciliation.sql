ALTER TABLE pos_terminal_sessions ADD COLUMN IF NOT EXISTS sync_status TEXT DEFAULT 'SYNCED';
ALTER TABLE pos_terminal_sessions ADD COLUMN IF NOT EXISTS pending_reconciliation JSONB DEFAULT '[]'::jsonb;
ALTER TABLE pos_terminal_sessions ADD COLUMN IF NOT EXISTS last_conflict_resolved_at TIMESTAMP;
