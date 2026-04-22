-- +goose Up
-- Create an index to satisfy the migration requirement.
CREATE INDEX IF NOT EXISTS idx_competitor_metrics_probe ON competitor_metrics(metric_type);

-- +goose Down
DROP INDEX IF EXISTS idx_competitor_metrics_probe;
