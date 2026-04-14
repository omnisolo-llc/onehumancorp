-- Add crdt_vector JSONB column to shared_tasks for Hybrid CRDT MCP
ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS crdt_vector JSONB DEFAULT '{}';
