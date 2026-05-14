-- 073_memory_advanced_pruning_sqlite.sql

ALTER TABLE consolidated_memory ADD COLUMN business_event_type TEXT;
ALTER TABLE consolidated_memory ADD COLUMN owner_activity_level INTEGER DEFAULT 50;
