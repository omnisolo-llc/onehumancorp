-- Add business_tagline column to tenants table to remove mock data from UI
ALTER TABLE tenants ADD COLUMN IF NOT EXISTS business_tagline TEXT;
