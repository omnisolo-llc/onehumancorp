-- Repair migration to ensure autodream_memories table exists with required base schema
-- This handles cases where migrations ran out of order or schema was inconsistent

-- Only create if table doesn't exist (preserve any columns added by later migrations like 029)
CREATE TABLE IF NOT EXISTS autodream_memories (
    id TEXT PRIMARY KEY,
    content TEXT NOT NULL,
    embedding TEXT,
    source_mission_id TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
