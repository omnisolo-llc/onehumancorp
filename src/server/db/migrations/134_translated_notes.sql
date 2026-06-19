-- Migration 134: Add translated_notes to orders
ALTER TABLE orders
ADD COLUMN IF NOT EXISTS translated_notes TEXT;
