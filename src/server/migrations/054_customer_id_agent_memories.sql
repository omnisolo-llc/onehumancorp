-- +goose Up
-- Migration 054: Add customer_id to agent_memories
ALTER TABLE agent_memories ADD COLUMN IF NOT EXISTS customer_id TEXT REFERENCES customers(id) ON DELETE CASCADE;

-- +goose Down
ALTER TABLE agent_memories DROP COLUMN IF NOT EXISTS customer_id;
