-- +goose Up
-- Migration 114: Add idempotency_key and device_id to pos_offline_transactions

ALTER TABLE pos_offline_transactions ADD COLUMN IF NOT EXISTS idempotency_key TEXT;
ALTER TABLE pos_offline_transactions ADD COLUMN IF NOT EXISTS device_id TEXT;
CREATE INDEX IF NOT EXISTS idx_pos_offline_idempotency ON pos_offline_transactions(tenant_id, idempotency_key);
ALTER TABLE pos_offline_transactions ADD CONSTRAINT unique_pos_offline_idempotency UNIQUE (tenant_id, idempotency_key);

-- +goose Down
ALTER TABLE pos_offline_transactions DROP CONSTRAINT IF EXISTS unique_pos_offline_idempotency;
DROP INDEX IF EXISTS idx_pos_offline_idempotency;
ALTER TABLE pos_offline_transactions DROP COLUMN IF EXISTS idempotency_key;
ALTER TABLE pos_offline_transactions DROP COLUMN IF EXISTS device_id;
