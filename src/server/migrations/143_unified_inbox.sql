-- Migration 143: Unified Inbox Schema (SQLite)

CREATE TABLE IF NOT EXISTS unified_messages (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    channel_type TEXT NOT NULL,
    sender_id TEXT NOT NULL,
    content TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'unread',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS pending_work_actions (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    message_id TEXT NOT NULL REFERENCES unified_messages(id) ON DELETE CASCADE,
    action_type TEXT NOT NULL,
    payload TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Note: RLS (Row Level Security) is not supported natively in SQLite the same way it is in PostgreSQL.
-- The queries against these tables in SQLite should enforce tenant isolation manually in the WHERE clauses.
