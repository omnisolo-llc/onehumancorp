-- 056_memory_consolidation_features.sql

ALTER TABLE consolidated_memory ADD COLUMN IF NOT EXISTS last_referenced_at TIMESTAMPTZ;
UPDATE consolidated_memory SET last_referenced_at = created_at WHERE last_referenced_at IS NULL;

ALTER TABLE consolidated_memory ADD COLUMN IF NOT EXISTS reference_count INTEGER DEFAULT 0;
ALTER TABLE consolidated_memory ADD COLUMN IF NOT EXISTS reliability_score INTEGER DEFAULT 50;
ALTER TABLE consolidated_memory ADD COLUMN IF NOT EXISTS owner_override BOOLEAN DEFAULT FALSE;
ALTER TABLE consolidated_memory ADD COLUMN IF NOT EXISTS metadata TEXT;
