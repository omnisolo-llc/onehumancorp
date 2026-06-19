-- +goose Up
-- Migration 143: Add Work Triage Engine columns to agent_feed_items

ALTER TABLE agent_feed_items ADD COLUMN IF NOT EXISTS correlation_id TEXT;
ALTER TABLE agent_feed_items ADD COLUMN IF NOT EXISTS priority_score INTEGER DEFAULT 0;
CREATE INDEX IF NOT EXISTS idx_agent_feed_correlation ON agent_feed_items(tenant_id, correlation_id);

-- +goose Down
DROP INDEX IF EXISTS idx_agent_feed_correlation;
ALTER TABLE agent_feed_items DROP COLUMN IF EXISTS correlation_id;
ALTER TABLE agent_feed_items DROP COLUMN IF EXISTS priority_score;
