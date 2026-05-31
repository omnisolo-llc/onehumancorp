-- +goose Up
-- Migration 054: Add customer_id to agent_memories
ALTER TABLE agent_memories ADD COLUMN customer_id TEXT REFERENCES customers(id) ON DELETE CASCADE;

-- +goose Down
ALTER TABLE agent_memories DROP COLUMN customer_id;
