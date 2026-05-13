-- 042_telemetry_mesh.sql
-- Upgrade telemetry_buffer from Go migration 032

ALTER TABLE telemetry_buffer ADD COLUMN IF NOT EXISTS metric_name TEXT;
ALTER TABLE telemetry_buffer ADD COLUMN IF NOT EXISTS value REAL;
ALTER TABLE telemetry_buffer ADD COLUMN IF NOT EXISTS labels_json TEXT;
ALTER TABLE telemetry_buffer ADD COLUMN IF NOT EXISTS timestamp TIMESTAMPTZ;
ALTER TABLE telemetry_buffer ADD COLUMN IF NOT EXISTS sync_status TEXT DEFAULT 'pending';
