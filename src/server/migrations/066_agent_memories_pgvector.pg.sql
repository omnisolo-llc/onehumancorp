-- PostgreSQL specific extension for agent_memories
CREATE EXTENSION IF NOT EXISTS vector;
ALTER TABLE agent_memories ALTER COLUMN embedding TYPE vector(1536) USING embedding::vector;
