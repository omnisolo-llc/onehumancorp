-- +goose Up
-- Add sync metadata columns to memory tables for Hybrid MCP RAG Protocol
ALTER TABLE agent_memory ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE agent_memory ADD COLUMN last_sync_at TIMESTAMP WITH TIME ZONE NULL;

ALTER TABLE memories ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE memories ADD COLUMN last_sync_at TIMESTAMP WITH TIME ZONE NULL;

ALTER TABLE autodream_memories ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE autodream_memories ADD COLUMN last_sync_at TIMESTAMP WITH TIME ZONE NULL;

-- +goose Down
ALTER TABLE autodream_memories DROP COLUMN IF EXISTS last_sync_at;
ALTER TABLE autodream_memories DROP COLUMN IF EXISTS sync_status;

ALTER TABLE memories DROP COLUMN IF EXISTS last_sync_at;
ALTER TABLE memories DROP COLUMN IF EXISTS sync_status;

ALTER TABLE agent_memory DROP COLUMN IF EXISTS last_sync_at;
ALTER TABLE agent_memory DROP COLUMN IF EXISTS sync_status;
