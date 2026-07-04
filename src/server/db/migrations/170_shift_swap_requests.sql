-- +goose Up
CREATE TABLE IF NOT EXISTS shift_swap_requests (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    shift_id TEXT NOT NULL REFERENCES shifts(id) ON DELETE CASCADE,
    requesting_staff_id TEXT NOT NULL REFERENCES ohc_staff_member(id) ON DELETE CASCADE,
    covering_staff_id TEXT REFERENCES ohc_staff_member(id) ON DELETE SET NULL,
    status TEXT NOT NULL DEFAULT 'pending' CHECK (status IN ('pending', 'approved', 'rejected', 'cancelled')),
    reason TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_shift_swap_requests_tenant_id ON shift_swap_requests(tenant_id);
CREATE INDEX IF NOT EXISTS idx_shift_swap_requests_shift_id ON shift_swap_requests(shift_id);
CREATE INDEX IF NOT EXISTS idx_shift_swap_requests_staff_id ON shift_swap_requests(requesting_staff_id);

ALTER TABLE shift_swap_requests ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_shift_swap_requests ON shift_swap_requests;
CREATE POLICY tenant_isolation_shift_swap_requests ON shift_swap_requests
USING (tenant_id::text = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- +goose Down
DROP POLICY IF EXISTS tenant_isolation_shift_swap_requests ON shift_swap_requests;
DROP TABLE IF EXISTS shift_swap_requests CASCADE;
