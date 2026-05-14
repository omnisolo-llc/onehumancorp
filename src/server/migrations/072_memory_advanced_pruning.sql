-- 072_memory_advanced_pruning.sql

ALTER TABLE consolidated_memory ADD COLUMN IF NOT EXISTS business_event_type TEXT;
ALTER TABLE consolidated_memory ADD COLUMN IF NOT EXISTS owner_activity_level INTEGER DEFAULT 50;
