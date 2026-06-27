-- Migration 163: Universal Capacity & Appointment Ledger (UCAL) Core
CREATE TABLE IF NOT EXISTS ucal_resources (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    resource_type TEXT NOT NULL, -- STAFF, EQUIPMENT, SPACE
    base_capacity INTEGER NOT NULL DEFAULT 1,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_ucal_resources_tenant ON ucal_resources(tenant_id);

ALTER TABLE ucal_resources ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_ucal_resources ON ucal_resources
    USING (tenant_id = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));


CREATE TABLE IF NOT EXISTS ucal_ledger (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    resource_id UUID NOT NULL REFERENCES ucal_resources(id) ON DELETE CASCADE,
    start_time TIMESTAMPTZ NOT NULL,
    end_time TIMESTAMPTZ NOT NULL,
    consumed_units INTEGER NOT NULL DEFAULT 1,
    total_units_at_time INTEGER, -- Overrides base_capacity if set
    status TEXT NOT NULL DEFAULT 'LOCKED', -- LOCKED, TENTATIVE, BLOCKED, BUFFER
    reference_id TEXT, -- booking_id, signal_id, etc.
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_ucal_ledger_tenant_resource_time ON ucal_ledger(tenant_id, resource_id, start_time, end_time);
CREATE INDEX IF NOT EXISTS idx_ucal_ledger_reference ON ucal_ledger(reference_id);

ALTER TABLE ucal_ledger ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_ucal_ledger ON ucal_ledger
    USING (tenant_id = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));


CREATE TABLE IF NOT EXISTS ucal_dynamic_buffers (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    ledger_id UUID NOT NULL REFERENCES ucal_ledger(id) ON DELETE CASCADE,
    buffer_type TEXT NOT NULL, -- TRAVEL, PREP, CLEANUP
    duration_minutes INTEGER NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_ucal_buffers_ledger ON ucal_dynamic_buffers(ledger_id);

ALTER TABLE ucal_dynamic_buffers ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_ucal_dynamic_buffers ON ucal_dynamic_buffers
    USING (tenant_id = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
