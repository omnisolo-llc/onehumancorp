-- Migration: 114_inbox_ai_columns.sql
-- Adds columns to track AI-handled messages and confidence scores.

ALTER TABLE inbox_messages ADD COLUMN IF NOT EXISTS handled_by_ai BOOLEAN DEFAULT FALSE;
ALTER TABLE inbox_messages ADD COLUMN IF NOT EXISTS confidence_score FLOAT DEFAULT 0.0;
ALTER TABLE inbox_messages ADD COLUMN IF NOT EXISTS ai_metadata JSONB DEFAULT '{}'::jsonb;
ALTER TABLE inbox_messages ADD COLUMN IF NOT EXISTS sender_id TEXT;

-- Index for analytics and metrics
CREATE INDEX IF NOT EXISTS idx_inbox_messages_handled_by_ai ON inbox_messages(tenant_id, handled_by_ai);
