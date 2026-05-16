-- +goose Up
ALTER TABLE local_telemetry_metrics ENABLE ROW LEVEL SECURITY;

-- +goose Down
ALTER TABLE local_telemetry_metrics DISABLE ROW LEVEL SECURITY;
