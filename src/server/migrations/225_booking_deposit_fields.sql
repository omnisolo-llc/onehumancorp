-- +goose Up
ALTER TABLE services ADD COLUMN IF NOT EXISTS requires_deposit BOOLEAN DEFAULT false;
ALTER TABLE services ADD COLUMN IF NOT EXISTS deposit_amount_cents BIGINT DEFAULT 0;
ALTER TABLE services ADD COLUMN IF NOT EXISTS requires_travel BOOLEAN DEFAULT false;

-- +goose Down
ALTER TABLE services DROP COLUMN IF EXISTS requires_travel;
ALTER TABLE services DROP COLUMN IF EXISTS deposit_amount_cents;
ALTER TABLE services DROP COLUMN IF EXISTS requires_deposit;
