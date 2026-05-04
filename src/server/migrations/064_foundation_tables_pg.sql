-- 064_foundation_tables_pg.sql
-- Postgres-specific extensions and RLS for foundation tables

CREATE EXTENSION IF NOT EXISTS vector;

-- Cast timestamps to TIMESTAMPTZ where appropriate
ALTER TABLE tenants ALTER COLUMN created_at TYPE TIMESTAMPTZ USING created_at::TIMESTAMPTZ;
ALTER TABLE tenants ALTER COLUMN created_at SET DEFAULT CURRENT_TIMESTAMP;
ALTER TABLE tenants ALTER COLUMN updated_at TYPE TIMESTAMPTZ USING updated_at::TIMESTAMPTZ;
ALTER TABLE tenants ALTER COLUMN updated_at SET DEFAULT CURRENT_TIMESTAMP;

ALTER TABLE customers ALTER COLUMN created_at TYPE TIMESTAMPTZ USING created_at::TIMESTAMPTZ;
ALTER TABLE customers ALTER COLUMN created_at SET DEFAULT CURRENT_TIMESTAMP;
ALTER TABLE customers ALTER COLUMN updated_at TYPE TIMESTAMPTZ USING updated_at::TIMESTAMPTZ;
ALTER TABLE customers ALTER COLUMN updated_at SET DEFAULT CURRENT_TIMESTAMP;

ALTER TABLE orders ALTER COLUMN created_at TYPE TIMESTAMPTZ USING created_at::TIMESTAMPTZ;
ALTER TABLE orders ALTER COLUMN created_at SET DEFAULT CURRENT_TIMESTAMP;
ALTER TABLE orders ALTER COLUMN updated_at TYPE TIMESTAMPTZ USING updated_at::TIMESTAMPTZ;
ALTER TABLE orders ALTER COLUMN updated_at SET DEFAULT CURRENT_TIMESTAMP;

ALTER TABLE bookings ALTER COLUMN created_at TYPE TIMESTAMPTZ USING created_at::TIMESTAMPTZ;
ALTER TABLE bookings ALTER COLUMN created_at SET DEFAULT CURRENT_TIMESTAMP;
ALTER TABLE bookings ALTER COLUMN updated_at TYPE TIMESTAMPTZ USING updated_at::TIMESTAMPTZ;
ALTER TABLE bookings ALTER COLUMN updated_at SET DEFAULT CURRENT_TIMESTAMP;
ALTER TABLE bookings ALTER COLUMN start_time TYPE TIMESTAMPTZ USING start_time::TIMESTAMPTZ;

-- Add embedding vector to interactions
ALTER TABLE interactions ADD COLUMN IF NOT EXISTS embedding vector(1536);

-- Enable RLS
ALTER TABLE tenants ENABLE ROW LEVEL SECURITY;
ALTER TABLE customers ENABLE ROW LEVEL SECURITY;
ALTER TABLE orders ENABLE ROW LEVEL SECURITY;
ALTER TABLE bookings ENABLE ROW LEVEL SECURITY;

-- Apply Tenant Isolation Policies (Drop if exists first to be safe)
DROP POLICY IF EXISTS tenant_isolation_tenants_foundation ON tenants;
CREATE POLICY tenant_isolation_tenants_foundation ON tenants USING (id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system' OR current_setting('app.current_tenant', true) = '');

DROP POLICY IF EXISTS tenant_isolation_customers_foundation ON customers;
CREATE POLICY tenant_isolation_customers_foundation ON customers USING (tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system' OR current_setting('app.current_tenant', true) = '');

DROP POLICY IF EXISTS tenant_isolation_orders_foundation ON orders;
CREATE POLICY tenant_isolation_orders_foundation ON orders USING (tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system' OR current_setting('app.current_tenant', true) = '');

DROP POLICY IF EXISTS tenant_isolation_bookings_foundation ON bookings;
CREATE POLICY tenant_isolation_bookings_foundation ON bookings USING (tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system' OR current_setting('app.current_tenant', true) = '');
