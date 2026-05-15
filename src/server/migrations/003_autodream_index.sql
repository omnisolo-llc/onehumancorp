CREATE INDEX IF NOT EXISTS knowledge_embeddings_embedding_idx ON knowledge_embeddings USING ivfflat (embedding vector_cosine_ops) WITH (lists = 100);
