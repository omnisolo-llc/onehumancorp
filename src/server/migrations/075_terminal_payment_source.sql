-- Migration 075: Add payment_source to orders for POS integration

ALTER TABLE orders ADD COLUMN IF NOT EXISTS payment_source TEXT DEFAULT 'ONLINE';

-- Note: STRIPE_TERMINAL_TAP is one of the valid states for payment_source
