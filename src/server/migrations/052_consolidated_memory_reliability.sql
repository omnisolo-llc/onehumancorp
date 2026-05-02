-- Add reliability and owner_override to consolidated_memory
ALTER TABLE consolidated_memory ADD COLUMN IF NOT EXISTS reliability FLOAT DEFAULT 1.0;
ALTER TABLE consolidated_memory ADD COLUMN IF NOT EXISTS owner_override BOOLEAN DEFAULT FALSE;
