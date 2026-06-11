-- Add split configurations to invoices table
ALTER TABLE invoices
ADD COLUMN IF NOT EXISTS split_partner_id TEXT,
ADD COLUMN IF NOT EXISTS split_percentage DOUBLE PRECISION;
