-- Add SEO metadata fields to products table
ALTER TABLE products
ADD COLUMN IF NOT EXISTS seo_title TEXT,
ADD COLUMN IF NOT EXISTS seo_description TEXT,
ADD COLUMN IF NOT EXISTS seo_schema_json JSONB;

-- Add SEO metadata fields to services table
ALTER TABLE services
ADD COLUMN IF NOT EXISTS seo_title TEXT,
ADD COLUMN IF NOT EXISTS seo_description TEXT,
ADD COLUMN IF NOT EXISTS seo_schema_json JSONB;
