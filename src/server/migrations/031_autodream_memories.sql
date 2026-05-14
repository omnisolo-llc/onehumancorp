-- 031_autodream_memories.sql
-- Add topic to autodream_memories from Go migration 027

ALTER TABLE autodream_memories ADD COLUMN IF NOT EXISTS topic TEXT NOT NULL DEFAULT '';
