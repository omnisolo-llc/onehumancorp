-- +goose Up
-- +goose StatementBegin
-- Add missing columns to telemetry_buffer created in 026
ALTER TABLE telemetry_buffer ADD COLUMN metric_name TEXT DEFAULT '';
ALTER TABLE telemetry_buffer ADD COLUMN value DOUBLE PRECISION DEFAULT 0;
ALTER TABLE telemetry_buffer ADD COLUMN labels_json JSONB DEFAULT '{}';
ALTER TABLE telemetry_buffer ADD COLUMN timestamp TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP;
ALTER TABLE telemetry_buffer ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';

-- Migrate old data if any and clean up
UPDATE telemetry_buffer SET metric_name = metric_type WHERE metric_name = '';

CREATE INDEX IF NOT EXISTS idx_telemetry_buffer_status ON telemetry_buffer(sync_status);
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
DROP INDEX IF EXISTS idx_telemetry_buffer_status;
ALTER TABLE telemetry_buffer DROP COLUMN sync_status;
ALTER TABLE telemetry_buffer DROP COLUMN timestamp;
ALTER TABLE telemetry_buffer DROP COLUMN labels_json;
ALTER TABLE telemetry_buffer DROP COLUMN value;
ALTER TABLE telemetry_buffer DROP COLUMN metric_name;
-- +goose StatementEnd
