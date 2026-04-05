-- Add created_at column to autodream_memories
ALTER TABLE autodream_memories ADD COLUMN created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP;
