-- Add quote_id to invoices to link them
ALTER TABLE invoices ADD COLUMN IF NOT EXISTS quote_id TEXT REFERENCES quotes(id) ON DELETE SET NULL;
