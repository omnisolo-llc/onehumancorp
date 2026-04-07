-- SQLite doesn't support vector types directly, so for compatibility tests and SQLite standalone,
-- we ensure the fallback is correctly mapped. In Go migrations we handle the conditional.
-- Here we add missing columns or correct types.
-- The previous migrations already created `autodream_memories`.
-- We will just make sure any required things are done here, though 029 already added
-- organization_id, agent_id, source_type.

-- We add IF NOT EXISTS to the vector extension.
CREATE EXTENSION IF NOT EXISTS vector;

-- If needed, we just recreate the index.
CREATE INDEX IF NOT EXISTS idx_autodream_org ON autodream_memories(organization_id);
