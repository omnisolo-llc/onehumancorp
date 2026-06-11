-- Migration 118: Booking System Enhancements for Agentic Workflows

-- Ensure products table has columns for packages and deposits
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name='products' AND column_name='is_package') THEN
        ALTER TABLE products ADD COLUMN is_package BOOLEAN DEFAULT FALSE;
    END IF;
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name='products' AND column_name='package_credits') THEN
        ALTER TABLE products ADD COLUMN package_credits INTEGER DEFAULT 0;
    END IF;
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name='products' AND column_name='deposit_amount_cents') THEN
        ALTER TABLE products ADD COLUMN deposit_amount_cents BIGINT DEFAULT 0;
    END IF;
END $$;

-- Ensure services table has columns for packages if it exists
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM information_schema.tables WHERE table_name='services') THEN
        IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name='services' AND column_name='is_package') THEN
            ALTER TABLE services ADD COLUMN is_package BOOLEAN DEFAULT FALSE;
        END IF;
        IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name='services' AND column_name='package_credits') THEN
            ALTER TABLE services ADD COLUMN package_credits INTEGER DEFAULT 0;
        END IF;
    END IF;
END $$;

-- Extend bookings table to support package credit usage and rescheduling context
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name='bookings' AND column_name='credits_consumed') THEN
        ALTER TABLE bookings ADD COLUMN credits_consumed INTEGER DEFAULT 1;
    END IF;
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name='bookings' AND column_name='rescheduled_from_id') THEN
        ALTER TABLE bookings ADD COLUMN rescheduled_from_id TEXT REFERENCES bookings(id) ON DELETE SET NULL;
    END IF;
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name='bookings' AND column_name='notes') THEN
        ALTER TABLE bookings ADD COLUMN notes TEXT;
    END IF;
END $$;

-- Create a table for customer package balances if it doesn't exist
CREATE TABLE IF NOT EXISTS customer_package_balances (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    customer_id TEXT NOT NULL,
    product_id TEXT NOT NULL,
    remaining_credits INTEGER DEFAULT 0,
    expires_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- RLS for package balances
ALTER TABLE customer_package_balances ENABLE ROW LEVEL SECURITY;
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_policies WHERE policyname = 'tenant_isolation_customer_package_balances') THEN
        CREATE POLICY tenant_isolation_customer_package_balances ON customer_package_balances
            USING (tenant_id = current_setting('app.current_tenant', true))
            WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
    END IF;
END $$;
