-- Complete runtime-schema parity discovered by the real-stack browser suite.
-- This follows 225 so databases that already applied it keep a stable checksum.

-- The original schema calls the subscription level "tier"; billing and growth
-- paths also use "plan_tier". Keep the aliases coherent until code converges.
ALTER TABLE tenants ADD COLUMN IF NOT EXISTS plan_tier TEXT;
UPDATE tenants
SET plan_tier = COALESCE(tier, 'free')
WHERE plan_tier IS NULL OR plan_tier = '';
ALTER TABLE tenants ALTER COLUMN plan_tier SET DEFAULT 'free';

CREATE OR REPLACE FUNCTION sync_tenant_tier_columns()
RETURNS TRIGGER
LANGUAGE plpgsql
AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        IF NEW.tier IS NULL AND NEW.plan_tier IS NULL THEN
            NEW.tier := 'free';
            NEW.plan_tier := 'free';
        ELSIF NEW.tier IS NULL THEN
            NEW.tier := NEW.plan_tier;
        ELSIF NEW.plan_tier IS NULL THEN
            NEW.plan_tier := NEW.tier;
        ELSIF NEW.tier IS DISTINCT FROM NEW.plan_tier THEN
            IF NEW.tier = 'free' THEN
                NEW.tier := NEW.plan_tier;
            ELSIF NEW.plan_tier = 'free' THEN
                NEW.plan_tier := NEW.tier;
            ELSE
                RAISE EXCEPTION 'tenants tier and plan_tier must match';
            END IF;
        END IF;
    ELSE
        IF NEW.tier IS DISTINCT FROM OLD.tier
           AND NEW.plan_tier IS NOT DISTINCT FROM OLD.plan_tier THEN
            NEW.plan_tier := NEW.tier;
        ELSIF NEW.plan_tier IS DISTINCT FROM OLD.plan_tier
              AND NEW.tier IS NOT DISTINCT FROM OLD.tier THEN
            NEW.tier := NEW.plan_tier;
        ELSIF NEW.tier IS DISTINCT FROM NEW.plan_tier THEN
            RAISE EXCEPTION 'tenants tier and plan_tier must match';
        END IF;
    END IF;
    RETURN NEW;
END;
$$;

DROP TRIGGER IF EXISTS tenants_sync_tier_columns ON tenants;
CREATE TRIGGER tenants_sync_tier_columns
BEFORE INSERT OR UPDATE OF tier, plan_tier ON tenants
FOR EACH ROW EXECUTE FUNCTION sync_tenant_tier_columns();

-- Migration 008 declared these after migration 001 had already created orders.
ALTER TABLE orders ADD COLUMN IF NOT EXISTS notes TEXT;
ALTER TABLE orders ADD COLUMN IF NOT EXISTS translated_notes TEXT;

-- The staff API writes title, while migration 206 only carried description.
ALTER TABLE staff_tasks ADD COLUMN IF NOT EXISTS title TEXT;
UPDATE staff_tasks SET title = description WHERE title IS NULL;

-- Staff mesh existed in the retired migration set and SQLite bootstrap but was
-- never promoted into the active SQLx PostgreSQL set.
CREATE TABLE IF NOT EXISTS ohc_staff_member (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    name TEXT NOT NULL,
    phone_number TEXT NOT NULL,
    role TEXT NOT NULL,
    pin_hash TEXT,
    status TEXT NOT NULL DEFAULT 'ACTIVE',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_ohc_staff_member_tenant
    ON ohc_staff_member (tenant_id);

CREATE TABLE IF NOT EXISTS ohc_timecard_event (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    staff_id TEXT NOT NULL,
    event_type TEXT NOT NULL,
    event_time TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    sync_status TEXT NOT NULL DEFAULT 'SYNCED',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_ohc_timecard_event_tenant_staff
    ON ohc_timecard_event (tenant_id, staff_id);

ALTER TABLE ohc_staff_member ENABLE ROW LEVEL SECURITY;
ALTER TABLE ohc_timecard_event ENABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_ohc_staff_member ON ohc_staff_member;
CREATE POLICY tenant_isolation_ohc_staff_member ON ohc_staff_member
    USING (tenant_id = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

DROP POLICY IF EXISTS tenant_isolation_ohc_timecard_event ON ohc_timecard_event;
CREATE POLICY tenant_isolation_ohc_timecard_event ON ohc_timecard_event
    USING (tenant_id = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

GRANT ALL PRIVILEGES ON ohc_staff_member, ohc_timecard_event TO ohc_bypassrls;
