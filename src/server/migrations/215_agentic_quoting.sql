-- Migration 215: Add unstructured context for agentic quoting
ALTER TABLE quotes ADD COLUMN IF NOT EXISTS unstructured_context TEXT;
