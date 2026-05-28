-- Migration 020: Unify Onboarding Schema

-- Standardize Products table to support unified storefront and match OnboardingAgent expectations
ALTER TABLE products ADD COLUMN IF NOT EXISTS name TEXT;
ALTER TABLE products ADD COLUMN IF NOT EXISTS fulfillment_strategy TEXT;
ALTER TABLE products ADD COLUMN IF NOT EXISTS organization_id TEXT;

-- Ensure users table matches code expectations for organization_id
ALTER TABLE users ADD COLUMN IF NOT EXISTS organization_id TEXT;

-- Standardize bookings table
ALTER TABLE bookings ADD COLUMN IF NOT EXISTS organization_id TEXT;

-- Sync data between old and new column names to ensure compatibility
DO $$
BEGIN
    -- Products sync
    IF EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name='products' AND column_name='title') THEN
        UPDATE products SET name = title WHERE name IS NULL;
    END IF;

    IF EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name='products' AND column_name='type') THEN
        UPDATE products SET fulfillment_strategy = type WHERE fulfillment_strategy IS NULL;
    END IF;

    IF EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name='products' AND column_name='tenant_id') THEN
        UPDATE products SET organization_id = tenant_id WHERE organization_id IS NULL;
    END IF;

    -- Users sync
    IF EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name='users' AND column_name='tenant_id') THEN
        UPDATE users SET organization_id = tenant_id WHERE organization_id IS NULL;
    END IF;

    -- Bookings sync
    IF EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name='bookings' AND column_name='tenant_id') THEN
        UPDATE bookings SET organization_id = tenant_id WHERE organization_id IS NULL;
    END IF;
END $$;

-- Ensure agents table has required columns for OnboardingAgent
ALTER TABLE agents ADD COLUMN IF NOT EXISTS is_default BOOLEAN DEFAULT FALSE;
ALTER TABLE agents ADD COLUMN IF NOT EXISTS tenant_id TEXT;
ALTER TABLE agents ADD COLUMN IF NOT EXISTS organization_id TEXT;

-- Sync agents data
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name='agents' AND column_name='tenant_id') THEN
        UPDATE agents SET organization_id = tenant_id WHERE organization_id IS NULL;
    END IF;
END $$;
