-- Create tenant journey state tracking tables
CREATE TABLE IF NOT EXISTS tenant_journey (
    tenant_id TEXT PRIMARY KEY,
    phase TEXT NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS tenant_journey_history (
    id SERIAL PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    from_phase TEXT NOT NULL,
    to_phase TEXT NOT NULL,
    occurred_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_tenant_journey_history_tenant_id ON tenant_journey_history(tenant_id);
