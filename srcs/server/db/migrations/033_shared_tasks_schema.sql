-- We need to ensure that the SQLite compatibility works.
-- According to the memory: SQLite does not support IF NOT EXISTS in ALTER TABLE...
-- The feedback says: "In SQLite, unparenthesized function calls in DEFAULT constraints result in a fatal parse/syntax error (and SQLite does not natively support gen_random_uuid() or JSONB anyway). This single migration file will completely break the application's Standalone Desktop Mode."

CREATE TABLE IF NOT EXISTS shared_tasks (
    id VARCHAR PRIMARY KEY,
    organization_id VARCHAR NOT NULL,
    title VARCHAR NOT NULL,
    description TEXT,
    status VARCHAR NOT NULL DEFAULT 'PENDING',
    agent_id VARCHAR,
    priority VARCHAR NOT NULL DEFAULT 'P2',
    payload TEXT,
    parent_plan_id TEXT,
    dependencies TEXT NOT NULL DEFAULT '[]',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
