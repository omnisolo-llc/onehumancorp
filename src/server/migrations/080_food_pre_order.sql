-- Add notes to orders for Food Pre-Order & Pickup Workflow
ALTER TABLE orders ADD COLUMN IF NOT EXISTS notes TEXT;
ALTER TABLE orders ADD COLUMN IF NOT EXISTS translated_notes TEXT;
