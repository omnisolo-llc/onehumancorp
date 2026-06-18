-- +goose Up
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name='products' AND column_name='is_subscribable') THEN
        ALTER TABLE products ADD COLUMN is_subscribable BOOLEAN DEFAULT FALSE;
    END IF;
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name='products' AND column_name='subscription_frequency') THEN
        ALTER TABLE products ADD COLUMN subscription_frequency TEXT;
    END IF;
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name='products' AND column_name='subscription_discount_percent') THEN
        ALTER TABLE products ADD COLUMN subscription_discount_percent INTEGER DEFAULT 0;
    END IF;
END
$$;
