-- +goose Up
-- Unify product and booking data models in PostgreSQL

DO $$
BEGIN
  IF EXISTS (
      SELECT 1 FROM information_schema.columns
      WHERE table_name='bookings' AND column_name='service_id'
  ) THEN
      ALTER TABLE bookings ADD COLUMN IF NOT EXISTS product_id TEXT REFERENCES products(id) ON DELETE CASCADE;
      EXECUTE 'UPDATE bookings SET product_id = service_id WHERE service_id IS NOT NULL';
      ALTER TABLE bookings DROP CONSTRAINT IF EXISTS bookings_service_id_fkey;
      ALTER TABLE bookings DROP COLUMN IF EXISTS service_id;
  END IF;
END $$;

-- Drop services table
DROP TABLE IF EXISTS services CASCADE;

-- +goose Down
-- Revert Unify product and booking data models

CREATE TABLE IF NOT EXISTS services (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    description TEXT,
    duration_minutes INT DEFAULT 60,
    price DECIMAL DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE IF EXISTS bookings ADD COLUMN IF NOT EXISTS service_id TEXT REFERENCES services(id) ON DELETE CASCADE;

-- This is a destructive down migration, data loss for `services` will occur.
