-- Add hybrid sync metadata to agent_memories
ALTER TABLE agent_memories ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE agent_memories ADD COLUMN last_sync_at TIMESTAMP NULL;
