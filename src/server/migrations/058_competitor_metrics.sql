-- 058_competitor_metrics.sql

CREATE TABLE IF NOT EXISTS competitor_metrics (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    competitor_name TEXT NOT NULL,
    metrics_data TEXT NOT NULL,
    probed_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE competitor_metrics ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_competitor_metrics ON competitor_metrics USING (tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');
