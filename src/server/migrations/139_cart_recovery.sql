-- +goose Up
-- Migration 139: Cart Recovery

ALTER TABLE carts ADD COLUMN IF NOT EXISTS last_activity_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP;
ALTER TABLE carts ADD COLUMN IF NOT EXISTS recovery_status TEXT DEFAULT 'none'; -- 'none', 'scheduled', 'recovered', 'failed'

-- +goose Down
ALTER TABLE carts DROP COLUMN IF EXISTS last_activity_at;
ALTER TABLE carts DROP COLUMN IF EXISTS recovery_status;
