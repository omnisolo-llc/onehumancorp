-- +goose Up
-- Migration 077: Add availability_schedules for native booking system

CREATE TABLE IF NOT EXISTS availability_schedules (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    provider_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    day_of_week INT NOT NULL CHECK (day_of_week >= 0 AND day_of_week <= 6),
    start_time TIME NOT NULL,
    end_time TIME NOT NULL,
    is_available BOOLEAN DEFAULT true,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT chk_time_range CHECK (end_time > start_time)
);

CREATE INDEX IF NOT EXISTS idx_availability_schedules_tenant_provider ON availability_schedules(tenant_id, provider_id);
CREATE INDEX IF NOT EXISTS idx_availability_schedules_day ON availability_schedules(day_of_week);

DO $$
DECLARE
    t_name text := 'availability_schedules';
    pol_name text := 'tenant_isolation_availability_schedules';
BEGIN
    IF to_regclass(t_name) IS NOT NULL THEN
        EXECUTE format('ALTER TABLE %I ENABLE ROW LEVEL SECURITY', t_name);
        IF NOT EXISTS (
            SELECT 1
            FROM pg_policies
            WHERE schemaname = current_schema()
                AND tablename = t_name
                AND policyname = pol_name
        ) THEN
            EXECUTE format(
                'CREATE POLICY %I ON %I USING (tenant_id::text = current_setting(''app.current_tenant'', true)) WITH CHECK (tenant_id::text = current_setting(''app.current_tenant'', true))',
                pol_name,
                t_name
            );
        END IF;
    END IF;
END
$$;

-- +goose Down
-- Reverse Migration 077

DO $$
DECLARE
    t_name text := 'availability_schedules';
    pol_name text := 'tenant_isolation_availability_schedules';
BEGIN
    IF to_regclass(t_name) IS NOT NULL THEN
        EXECUTE format('DROP POLICY IF EXISTS %I ON %I', pol_name, t_name);
        EXECUTE format('ALTER TABLE %I DISABLE ROW LEVEL SECURITY', t_name);
    END IF;
END
$$;

DROP INDEX IF EXISTS idx_availability_schedules_tenant_provider;
DROP INDEX IF EXISTS idx_availability_schedules_day;
DROP TABLE IF EXISTS availability_schedules CASCADE;
