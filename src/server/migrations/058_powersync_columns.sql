-- Migration: 058_powersync_columns.sql
-- Add sync-related columns to core tables required for offline standalone sync.

-- Table: agent_missions
ALTER TABLE agent_missions ADD COLUMN IF NOT EXISTS _sync_status TEXT DEFAULT 'pending';
ALTER TABLE agent_missions ADD COLUMN IF NOT EXISTS updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP;
ALTER TABLE agent_missions ADD COLUMN IF NOT EXISTS version INTEGER DEFAULT 1;

-- Table: shared_tasks
ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS _sync_status TEXT DEFAULT 'pending';
-- updated_at is already in shared_tasks from 024_kairos_orchestration.sql
ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS version INTEGER DEFAULT 1;

-- Table: swarm_tasks
ALTER TABLE swarm_tasks ADD COLUMN IF NOT EXISTS _sync_status TEXT DEFAULT 'pending';
-- updated_at should exist in swarm_tasks as well
ALTER TABLE swarm_tasks ADD COLUMN IF NOT EXISTS version INTEGER DEFAULT 1;

-- Table: agent_memories
ALTER TABLE agent_memories ADD COLUMN IF NOT EXISTS _sync_status TEXT DEFAULT 'pending';
ALTER TABLE agent_memories ADD COLUMN IF NOT EXISTS updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP;
ALTER TABLE agent_memories ADD COLUMN IF NOT EXISTS version INTEGER DEFAULT 1;
