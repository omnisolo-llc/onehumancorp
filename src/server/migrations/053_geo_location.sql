-- Migration 053: Geo Location Support

-- Enable PostGIS
CREATE EXTENSION IF NOT EXISTS postgis;

-- Add location support to businesses
ALTER TABLE businesses ADD COLUMN IF NOT EXISTS is_mobile BOOLEAN DEFAULT FALSE;
ALTER TABLE businesses ADD COLUMN IF NOT EXISTS accepting_orders BOOLEAN DEFAULT FALSE;
ALTER TABLE businesses ADD COLUMN IF NOT EXISTS current_location geometry(Point, 4326);
ALTER TABLE businesses ADD COLUMN IF NOT EXISTS last_location_update TIMESTAMPTZ;
ALTER TABLE businesses ADD COLUMN IF NOT EXISTS service_radius_km DECIMAL DEFAULT 10.0;

-- Index for spatial queries
CREATE INDEX IF NOT EXISTS idx_businesses_location ON businesses USING GIST (current_location);
