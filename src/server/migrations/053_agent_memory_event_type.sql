-- +goose Up
-- Migration 053: Add event_type to agent_memories
ALTER TABLE agent_memories ADD COLUMN event_type TEXT;

-- +goose Down
ALTER TABLE agent_memories DROP COLUMN event_type;
