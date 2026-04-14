-- 003_add_cached_tokens.sql
-- Add cached_tokens column to usage_events for tracking prompt caching costs.

ALTER TABLE usage_events ADD COLUMN cached_tokens BIGINT NOT NULL DEFAULT 0;
