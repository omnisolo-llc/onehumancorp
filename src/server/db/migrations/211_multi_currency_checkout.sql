ALTER TABLE checkout_sessions ADD COLUMN IF NOT EXISTS settlement_currency TEXT;
ALTER TABLE checkout_sessions ADD COLUMN IF NOT EXISTS settlement_amount_cents BIGINT;
ALTER TABLE checkout_sessions ADD COLUMN IF NOT EXISTS snapshotted_fx_rate DOUBLE PRECISION;
ALTER TABLE checkout_sessions ADD COLUMN IF NOT EXISTS applied_tax_rate DOUBLE PRECISION;
