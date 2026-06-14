-- +goose Up
ALTER TABLE quotes ADD COLUMN IF NOT EXISTS total_amount BIGINT;
ALTER TABLE quotes ADD COLUMN IF NOT EXISTS required_deposit BIGINT;
ALTER TABLE quotes ADD COLUMN IF NOT EXISTS checkout_url TEXT;

-- +goose Down
ALTER TABLE quotes DROP COLUMN IF EXISTS total_amount;
ALTER TABLE quotes DROP COLUMN IF EXISTS required_deposit;
ALTER TABLE quotes DROP COLUMN IF EXISTS checkout_url;
