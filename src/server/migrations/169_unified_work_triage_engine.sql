-- Migration: Unified Work Triage Engine (SQLite)

-- Update unified_messages
ALTER TABLE unified_messages ADD COLUMN channel TEXT;
ALTER TABLE unified_messages ADD COLUMN sender_id TEXT;
ALTER TABLE unified_messages ADD COLUMN raw_payload TEXT;
ALTER TABLE unified_messages ADD COLUMN normalized_text TEXT;
ALTER TABLE unified_messages ADD COLUMN status TEXT DEFAULT 'pending';

-- Create action_cards table
CREATE TABLE IF NOT EXISTS action_cards (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    message_id TEXT,
    card_type TEXT NOT NULL,
    content_json TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
