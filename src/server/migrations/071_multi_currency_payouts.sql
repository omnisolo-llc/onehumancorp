-- Add payout_status and guaranteed_at columns to multi_currency_ledger
ALTER TABLE ohc_multi_currency_ledger
ADD COLUMN payout_status TEXT NOT NULL DEFAULT 'pending',
ADD COLUMN guaranteed_at TIMESTAMPTZ;
