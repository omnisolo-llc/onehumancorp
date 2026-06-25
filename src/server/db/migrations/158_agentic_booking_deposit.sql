-- +goose Up
ALTER TABLE services ADD COLUMN IF NOT EXISTS requires_deposit BOOLEAN NOT NULL DEFAULT FALSE;
ALTER TABLE services ADD COLUMN IF NOT EXISTS deposit_amount_cents BIGINT NOT NULL DEFAULT 0;

-- +goose Down
ALTER TABLE services DROP COLUMN IF EXISTS deposit_amount_cents;
ALTER TABLE services DROP COLUMN IF EXISTS requires_deposit;
