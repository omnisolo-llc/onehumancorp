-- Migration: Add missing columns to invoices table

ALTER TABLE invoices ADD COLUMN IF NOT EXISTS client_id TEXT;
ALTER TABLE invoices ADD COLUMN IF NOT EXISTS client_name TEXT;
ALTER TABLE invoices ADD COLUMN IF NOT EXISTS payment_status TEXT DEFAULT 'unpaid';
ALTER TABLE invoices ADD COLUMN IF NOT EXISTS view_count INTEGER DEFAULT 0;
ALTER TABLE invoices ADD COLUMN IF NOT EXISTS amount_paid_cents INTEGER DEFAULT 0;
ALTER TABLE invoices ADD COLUMN IF NOT EXISTS total_amount_cents INTEGER DEFAULT 0;
ALTER TABLE invoices ADD COLUMN IF NOT EXISTS stripe_payment_link TEXT;
