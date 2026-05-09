-- +goose Up
ALTER TABLE local_telemetry_metrics ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_local_telemetry_metrics ON local_telemetry_metrics;
CREATE POLICY tenant_isolation_local_telemetry_metrics ON local_telemetry_metrics
    USING (true); -- Local standalone telemetry has no tenant scope but requires RLS to be consistently enabled in the DB per security policy

-- +goose Down
DROP POLICY IF EXISTS tenant_isolation_local_telemetry_metrics ON local_telemetry_metrics;
ALTER TABLE local_telemetry_metrics DISABLE ROW LEVEL SECURITY;
