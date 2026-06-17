-- +goose Up
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name='products' AND column_name='is_subscription_enabled') THEN
        ALTER TABLE products ADD COLUMN is_subscription_enabled BOOLEAN DEFAULT FALSE;
    END IF;
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name='products' AND column_name='subscription_interval') THEN
        ALTER TABLE products ADD COLUMN subscription_interval TEXT;
    END IF;
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name='products' AND column_name='subscription_discount') THEN
        ALTER TABLE products ADD COLUMN subscription_discount INTEGER DEFAULT 0;
    END IF;
END
$$;
