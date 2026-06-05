-- +goose Up
-- Migration 076: Create schema for Native Booking System

CREATE TABLE IF NOT EXISTS availability_schedules (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    provider_id TEXT,
    schedule_json JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS deposits (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    booking_id TEXT NOT NULL REFERENCES bookings(id) ON DELETE CASCADE,
    amount_cents BIGINT NOT NULL,
    stripe_payment_intent_id TEXT,
    status TEXT NOT NULL DEFAULT 'pending',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE availability_schedules ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_availability_schedules ON availability_schedules USING (tenant_id = current_setting('app.current_tenant', true)::text) WITH CHECK (tenant_id = current_setting('app.current_tenant', true)::text);

ALTER TABLE deposits ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_deposits ON deposits USING (tenant_id = current_setting('app.current_tenant', true)::text) WITH CHECK (tenant_id = current_setting('app.current_tenant', true)::text);

-- +goose Down
DROP POLICY IF EXISTS tenant_isolation_deposits ON deposits;
ALTER TABLE deposits DISABLE ROW LEVEL SECURITY;
DROP TABLE IF EXISTS deposits;

DROP POLICY IF EXISTS tenant_isolation_availability_schedules ON availability_schedules;
ALTER TABLE availability_schedules DISABLE ROW LEVEL SECURITY;
DROP TABLE IF EXISTS availability_schedules;
