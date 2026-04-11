-- Add sync_status and last_sync_at to autodream_memories for Hybrid MCP RAG Protocol
ALTER TABLE autodream_memories ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE autodream_memories ADD COLUMN last_sync_at TIMESTAMP NULL;

-- Add sync_status and last_sync_at to agent_memories for Hybrid MCP RAG Protocol
ALTER TABLE agent_memories ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE agent_memories ADD COLUMN last_sync_at TIMESTAMP NULL;
