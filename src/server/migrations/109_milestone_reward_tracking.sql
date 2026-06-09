-- +goose Up
-- Migration 109: Add reward_claimed_at to business_milestones
ALTER TABLE business_milestones ADD COLUMN IF NOT EXISTS reward_claimed_at TIMESTAMPTZ;

-- +goose Down
-- Reverse Migration 109
ALTER TABLE business_milestones DROP COLUMN IF EXISTS reward_claimed_at;
