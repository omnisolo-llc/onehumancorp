ALTER TABLE telemetry_buffer ADD COLUMN IF NOT EXISTS organization_id TEXT NOT NULL DEFAULT 'system';
