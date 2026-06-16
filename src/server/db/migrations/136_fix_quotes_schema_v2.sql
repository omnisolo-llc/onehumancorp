-- +goose Up
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name='quotes' AND column_name='total_amount_cents') THEN
        ALTER TABLE quotes RENAME COLUMN total_amount TO total_amount_cents;
    END IF;
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name='quotes' AND column_name='required_deposit_cents') THEN
        ALTER TABLE quotes RENAME COLUMN required_deposit TO required_deposit_cents;
    END IF;
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name='quotes' AND column_name='stripe_payment_link') THEN
        ALTER TABLE quotes RENAME COLUMN checkout_url TO stripe_payment_link;
    END IF;
END
$$;
