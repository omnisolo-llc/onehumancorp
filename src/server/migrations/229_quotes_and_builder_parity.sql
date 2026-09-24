-- Migration 229: Quotes and Builder sites column parity

ALTER TABLE quotes ADD COLUMN IF NOT EXISTS valid_until TIMESTAMPTZ;
ALTER TABLE quotes ADD COLUMN IF NOT EXISTS total_amount_cents BIGINT DEFAULT 0;
ALTER TABLE quotes ADD COLUMN IF NOT EXISTS required_deposit_cents BIGINT DEFAULT 0;
ALTER TABLE quotes ADD COLUMN IF NOT EXISTS stripe_payment_link TEXT;
ALTER TABLE quotes ADD COLUMN IF NOT EXISTS proposed_slot_id TEXT;
ALTER TABLE quotes ADD COLUMN IF NOT EXISTS service_id TEXT;

ALTER TABLE builder_sites ADD COLUMN IF NOT EXISTS published_at TIMESTAMPTZ;
