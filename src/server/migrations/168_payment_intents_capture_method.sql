-- +goose Up
ALTER TABLE payment_intents ADD COLUMN IF NOT EXISTS capture_method TEXT DEFAULT 'automatic';

-- +goose Down
ALTER TABLE payment_intents DROP COLUMN IF NOT EXISTS capture_method;
