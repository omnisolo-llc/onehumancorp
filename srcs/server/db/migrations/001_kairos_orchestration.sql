CREATE TABLE IF NOT EXISTS shared_tasks (
    id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL,
    title TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'PENDING',
    dependencies JSONB
);

CREATE EXTENSION IF NOT EXISTS vector;
CREATE TABLE autodream_memories (
    id TEXT PRIMARY KEY,
    embedding vector(1536),
    content TEXT NOT NULL
);
