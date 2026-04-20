CREATE TABLE IF NOT EXISTS competitor_metrics (
    id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL,
    competitor_name TEXT NOT NULL,
    metric_type TEXT NOT NULL,
    metric_value TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_competitor_metrics_org_name ON competitor_metrics(organization_id, competitor_name);
