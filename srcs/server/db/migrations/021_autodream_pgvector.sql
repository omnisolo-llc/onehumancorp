-- 021_autodream_pgvector.sql
-- Postgres migration to alter swarm_memory_embeddings to vector

CREATE EXTENSION IF NOT EXISTS vector;

-- In Postgres we need to alter the BYTEA column to VECTOR
-- In SQLite we just ignore it.
-- We will write a special string that database.go can regex out for SQLite.

ALTER TABLE swarm_memory_embeddings ALTER COLUMN vector_embedding TYPE vector(1536) USING vector_embedding::text::vector;
