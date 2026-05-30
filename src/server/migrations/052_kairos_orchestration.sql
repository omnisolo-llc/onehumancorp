-- Migration 052: KAIROS Orchestration Schema Enhancements

-- 1. Create mcp_tool_state table
CREATE TABLE IF NOT EXISTS mcp_tool_state (
    tool_id TEXT NOT NULL,
    key TEXT NOT NULL,
    value TEXT NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (tool_id, key)
);

-- Apply RLS if applicable based on how it's queried (Assuming system-level or we add tenant_id later if needed, but per design doc, it's just tool_id, key, value, updated_at).
-- Wait, let's keep it simple as per design doc.

-- 2. Add columns to shared_tasks
ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS claimed_by TEXT;
ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS claim_status TEXT DEFAULT 'UNCLAIMED';
