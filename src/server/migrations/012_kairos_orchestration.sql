-- 012_kairos_orchestration.sql

-- For Postgres, the application layer runs `CREATE EXTENSION IF NOT EXISTS vector;`
-- separately during setup. However, since the issue requires `vector(1536)` explicitly,
-- and standard SQL doesn't have DO $$ blocks, we must rely on the application to handle driver differences.
-- In this codebase, the migrations run on both SQLite and Postgres.
-- However, SQLite ignores `vector(1536)` type affinity if defined, and just treats it as a BLOB/TEXT affinity.
-- So we can safely use `vector(1536)` and SQLite will just ignore it.

CREATE TABLE IF NOT EXISTS shared_tasks (
    id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL DEFAULT 'PENDING',
    agent_id TEXT,
    priority TEXT NOT NULL DEFAULT 'P2',
    payload TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS task_dependencies (
    task_id TEXT REFERENCES shared_tasks(id) ON DELETE CASCADE,
    depends_on_task_id TEXT REFERENCES shared_tasks(id) ON DELETE CASCADE,
    PRIMARY KEY (task_id, depends_on_task_id)
);

CREATE TABLE IF NOT EXISTS agent_memories (
    id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL,
    content TEXT NOT NULL,
    embedding vector(1536),
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
