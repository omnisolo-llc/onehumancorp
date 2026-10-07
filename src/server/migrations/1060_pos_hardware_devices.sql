-- +goose Up
CREATE TABLE IF NOT EXISTS pos_hardware_devices (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    stripe_reader_id TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'OFFLINE',
    last_seen TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    battery_level INT DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(tenant_id, stripe_reader_id)
);

DO $$
BEGIN
    IF to_regclass('pos_hardware_devices') IS NOT NULL THEN
        ALTER TABLE pos_hardware_devices ENABLE ROW LEVEL SECURITY;
        IF NOT EXISTS (
            SELECT 1
            FROM pg_policies
            WHERE schemaname = current_schema()
                AND tablename = 'pos_hardware_devices'
                AND policyname = 'tenant_isolation_pos_hardware_devices'
        ) THEN
            CREATE POLICY tenant_isolation_pos_hardware_devices ON pos_hardware_devices USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
        END IF;
    END IF;
END
$$;
