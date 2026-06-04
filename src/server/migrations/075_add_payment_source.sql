-- +goose Up
ALTER TABLE orders ADD COLUMN IF NOT EXISTS payment_source TEXT;

-- +goose Down
ALTER TABLE orders DROP COLUMN IF NOT EXISTS payment_source;
