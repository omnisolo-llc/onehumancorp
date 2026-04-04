-- 021_swarm_memory_embeddings_vector.sql

CREATE EXTENSION IF NOT EXISTS vector;

-- Use ALTER COLUMN TYPE instead of DROP COLUMN to avoid data loss.
-- SQLite ignores this in tests via RunMigrations if we skip ALTER COLUMN.

ALTER TABLE swarm_memory_embeddings ALTER COLUMN vector_embedding TYPE vector(1536) USING vector_embedding::text::vector;
