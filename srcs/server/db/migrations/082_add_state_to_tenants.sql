-- +goose Up
-- Add owner_email if not exists
ALTER TABLE tenants ADD COLUMN IF NOT EXISTS owner_email VARCHAR;
-- Add state column to tenants for frontend wizard persistence
ALTER TABLE tenants ADD COLUMN IF NOT EXISTS state TEXT;

-- +goose Down
ALTER TABLE tenants DROP COLUMN IF EXISTS state;
ALTER TABLE tenants DROP COLUMN IF EXISTS owner_email;
