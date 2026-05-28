CREATE TABLE IF NOT EXISTS swarm_memory_embeddings (
    memory_id TEXT PRIMARY KEY,
    context TEXT NOT NULL,
    vector_embedding vector(1536),
    source_plugin TEXT
);
