-- 062_billing_actions.sql
-- Add is_action column to usage_events for tracking AI action limits.

ALTER TABLE usage_events ADD COLUMN is_action BOOLEAN NOT NULL DEFAULT FALSE;
