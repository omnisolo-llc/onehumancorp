-- Unified Resource Scheduling Matrix: Resource and Ledger Models
CREATE TABLE IF NOT EXISTS scheduling_resources (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    resource_type TEXT NOT NULL CHECK (resource_type IN ('time', 'stock')),
    name TEXT NOT NULL,
    metadata JSONB DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_scheduling_resources_tenant ON scheduling_resources(tenant_id);

ALTER TABLE scheduling_resources ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_scheduling_resources ON scheduling_resources;
CREATE POLICY tenant_isolation_scheduling_resources
ON scheduling_resources
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));


CREATE TABLE IF NOT EXISTS scheduling_ledger (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    resource_id TEXT NOT NULL REFERENCES scheduling_resources(id) ON DELETE CASCADE,
    action_type TEXT NOT NULL CHECK (action_type IN ('reserve', 'release', 'consume')),
    quantity INTEGER NOT NULL DEFAULT 1,
    start_time TIMESTAMPTZ,
    end_time TIMESTAMPTZ,
    reference_id TEXT, -- e.g. booking_id or order_id
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_scheduling_ledger_tenant ON scheduling_ledger(tenant_id);
CREATE INDEX IF NOT EXISTS idx_scheduling_ledger_resource ON scheduling_ledger(resource_id);

ALTER TABLE scheduling_ledger ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_scheduling_ledger ON scheduling_ledger;
CREATE POLICY tenant_isolation_scheduling_ledger
ON scheduling_ledger
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- Implement append-only constraint via trigger
CREATE OR REPLACE FUNCTION prevent_scheduling_ledger_update_or_delete()
RETURNS TRIGGER AS $$
BEGIN
    RAISE EXCEPTION 'scheduling_ledger is append-only';
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trg_append_only_scheduling_ledger_update ON scheduling_ledger;
CREATE TRIGGER trg_append_only_scheduling_ledger_update
BEFORE UPDATE ON scheduling_ledger
FOR EACH ROW EXECUTE FUNCTION prevent_scheduling_ledger_update_or_delete();

DROP TRIGGER IF EXISTS trg_append_only_scheduling_ledger_delete ON scheduling_ledger;
CREATE TRIGGER trg_append_only_scheduling_ledger_delete
BEFORE DELETE ON scheduling_ledger
FOR EACH ROW EXECUTE FUNCTION prevent_scheduling_ledger_update_or_delete();
