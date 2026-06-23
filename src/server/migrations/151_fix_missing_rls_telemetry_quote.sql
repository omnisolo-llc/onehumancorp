-- +goose Up
-- Enable missing RLS on specific tables

ALTER TABLE IF EXISTS telemetry_buffer ENABLE ROW LEVEL SECURITY;

CREATE POLICY tenant_isolation_telemetry_buffer ON telemetry_buffer USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- +goose Down
DROP POLICY IF EXISTS tenant_isolation_telemetry_buffer ON telemetry_buffer;
ALTER TABLE IF EXISTS telemetry_buffer DISABLE ROW LEVEL SECURITY;
