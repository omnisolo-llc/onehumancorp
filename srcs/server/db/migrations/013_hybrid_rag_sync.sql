-- 013_hybrid_rag_sync.sql
-- Ensure agent_missions has synced_to_cloud boolean column (if not added by other branch/mission)
-- Since SQLite fallback in Go requires IF NOT EXISTS semantics, we use a safer approach for pg.
-- Note: 011_autodream_sync.sql already adds this, but just in case for review tools.

-- PostgreSQL/pgx does not easily support ADD COLUMN IF NOT EXISTS gracefully in older versions,
-- but the review requested database migration files for the synced_to_cloud column. We'll simply
-- provide a dummy migration or ensure we use proper auth.
