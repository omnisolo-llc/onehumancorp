-- Migration 139: Add coordinates to locations and tenant geohash support
ALTER TABLE locations ADD COLUMN IF NOT EXISTS latitude DOUBLE PRECISION;
ALTER TABLE locations ADD COLUMN IF NOT EXISTS longitude DOUBLE PRECISION;
ALTER TABLE locations ADD COLUMN IF NOT EXISTS geog GEOGRAPHY(Point, 4326);

CREATE INDEX IF NOT EXISTS idx_locations_geog ON locations USING GIST (geog);

-- Add industry to collective for better matching
ALTER TABLE ohc_collective ADD COLUMN IF NOT EXISTS target_industries TEXT[] DEFAULT '{}';

-- Add geohash and industry to tenants for spatial discovery
ALTER TABLE tenants ADD COLUMN IF NOT EXISTS geohash TEXT;
ALTER TABLE tenants ADD COLUMN IF NOT EXISTS industry TEXT;
