-- +goose Up
-- Add location coordinates to appointments if they do not exist
ALTER TABLE appointments ADD COLUMN IF NOT EXISTS location_lat DOUBLE PRECISION;
ALTER TABLE appointments ADD COLUMN IF NOT EXISTS location_lng DOUBLE PRECISION;

-- +goose Down
ALTER TABLE appointments DROP COLUMN IF EXISTS location_lat;
ALTER TABLE appointments DROP COLUMN IF EXISTS location_lng;
