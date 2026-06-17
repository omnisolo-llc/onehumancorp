-- Smart Invoicing enhancements
ALTER TABLE invoices ADD COLUMN IF NOT EXISTS last_reminded_at TIMESTAMPTZ;
