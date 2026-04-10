-- Migration: Add Hybrid Sync Metadata for RAG
-- This modifies autodream_memories to include sync tracking for the
-- Hybrid MCP RAG Protocol.
-- Compatible with PostgreSQL and SQLite.

ALTER TABLE autodream_memories ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE autodream_memories ADD COLUMN last_sync_at TIMESTAMP NULL;
