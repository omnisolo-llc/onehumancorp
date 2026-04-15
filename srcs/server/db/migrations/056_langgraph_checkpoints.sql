-- Migration: 056_langgraph_checkpoints.sql
-- Description: Create swarm_checkpoints table for Stateful Episodic Memory & LangGraph Checkpointing

CREATE TABLE IF NOT EXISTS swarm_checkpoints (
    thread_id TEXT NOT NULL,
    checkpoint_id TEXT NOT NULL,
    parent_id TEXT,
    checkpoint JSONB NOT NULL,
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (thread_id, checkpoint_id)
);

CREATE INDEX IF NOT EXISTS idx_checkpoints_thread_parent ON swarm_checkpoints (thread_id, parent_id);
