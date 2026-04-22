-- +goose Up
-- add an index for metric type queries since we are adding probing metrics
CREATE INDEX IF NOT EXISTS idx_competitor_metrics_type ON competitor_metrics(metric_type);

-- +goose Down
DROP INDEX IF EXISTS idx_competitor_metrics_type;
