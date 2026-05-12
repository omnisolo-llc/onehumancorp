CREATE TABLE IF NOT EXISTS consolidated_memory (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    agent_id TEXT,
    content TEXT NOT NULL,
    embedding vector(1536),
    source_type TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    last_referenced_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    reference_count INTEGER DEFAULT 0,
    reliability_score INTEGER DEFAULT 50,
    owner_override BOOLEAN DEFAULT FALSE,
    metadata TEXT,
    task_id TEXT
);
