-- +goose Up
-- Migration 038: Refine pos_terminal_sessions for offline-sync reconciliation

DO $$
BEGIN
    IF to_regclass('pos_terminal_sessions') IS NOT NULL THEN
        ALTER TABLE pos_terminal_sessions
        ADD COLUMN IF NOT EXISTS pending_reconciliation JSONB DEFAULT '[]'::jsonb,
        ADD COLUMN IF NOT EXISTS last_conflict_resolved_at TIMESTAMPTZ,
        ADD COLUMN IF NOT EXISTS sync_status TEXT DEFAULT 'SYNCED';
    END IF;
END
$$;

-- +goose Down
DO $$
BEGIN
    IF to_regclass('pos_terminal_sessions') IS NOT NULL THEN
        ALTER TABLE pos_terminal_sessions
        DROP COLUMN IF NOT EXISTS pending_reconciliation,
        DROP COLUMN IF NOT EXISTS last_conflict_resolved_at,
        DROP COLUMN IF NOT EXISTS sync_status;
    END IF;
END
$$;
