-- Migration 123: Agentic Quoting & Invoicing Workflow Support

-- Add customer_id if not present
ALTER TABLE quotes ADD COLUMN IF NOT EXISTS customer_id TEXT;

-- Epics act as Projects
ALTER TABLE epics ADD COLUMN IF NOT EXISTS quote_id TEXT REFERENCES quotes(id) ON DELETE SET NULL;

-- Make sure we have a reference to project/epic on invoices
ALTER TABLE invoices ADD COLUMN IF NOT EXISTS epic_id UUID REFERENCES epics(id) ON DELETE SET NULL;
