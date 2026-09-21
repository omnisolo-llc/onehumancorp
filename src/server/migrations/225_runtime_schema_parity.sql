-- Repair PostgreSQL runtime-schema drift exposed by the real-stack browser suite.
-- Keep this migration forward-only: src/server/migrations is executed by SQLx.

ALTER TABLE bookings
    ADD COLUMN IF NOT EXISTS service_id TEXT REFERENCES services(id) ON DELETE CASCADE;
ALTER TABLE bookings
    ADD COLUMN IF NOT EXISTS resource_id TEXT;

-- KAIROS uses organization_id while queue/worker paths use tenant_id. Keep both
-- live owner identifiers synchronized so either production path is valid.
ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS tenant_id TEXT;
ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS priority TEXT NOT NULL DEFAULT 'P2';
ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS parent_id TEXT;
ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS epic_id TEXT;
ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS assigned_agent TEXT;

UPDATE shared_tasks SET tenant_id = organization_id WHERE tenant_id IS NULL;

CREATE OR REPLACE FUNCTION sync_shared_task_owner_ids()
RETURNS TRIGGER
LANGUAGE plpgsql
AS $$
BEGIN
    IF NEW.organization_id IS NULL THEN
        NEW.organization_id := NEW.tenant_id;
    END IF;
    IF NEW.tenant_id IS NULL THEN
        NEW.tenant_id := NEW.organization_id;
    END IF;
    IF NEW.organization_id IS NULL OR NEW.tenant_id IS NULL THEN
        RAISE EXCEPTION 'shared_tasks requires a tenant/organization owner';
    END IF;
    IF NEW.organization_id IS DISTINCT FROM NEW.tenant_id THEN
        RAISE EXCEPTION 'shared_tasks tenant_id and organization_id must match';
    END IF;
    RETURN NEW;
END;
$$;

DROP TRIGGER IF EXISTS shared_tasks_sync_owner_ids ON shared_tasks;
CREATE TRIGGER shared_tasks_sync_owner_ids
BEFORE INSERT OR UPDATE OF organization_id, tenant_id ON shared_tasks
FOR EACH ROW EXECUTE FUNCTION sync_shared_task_owner_ids();

ALTER TABLE shared_tasks ALTER COLUMN tenant_id SET NOT NULL;
CREATE INDEX IF NOT EXISTS idx_shared_tasks_tenant_id ON shared_tasks (tenant_id);

CREATE TABLE IF NOT EXISTS inbound_signals (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    source TEXT NOT NULL,
    raw_payload JSONB NOT NULL,
    status TEXT NOT NULL DEFAULT 'PENDING'
        CHECK (status IN ('PENDING', 'PROCESSED', 'FAILED')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS daily_work_items (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    signal_id TEXT REFERENCES inbound_signals(id) ON DELETE SET NULL,
    intent TEXT NOT NULL,
    customer_info JSONB,
    suggested_actions JSONB,
    status TEXT NOT NULL DEFAULT 'PENDING'
        CHECK (status IN ('PENDING', 'APPROVED', 'DISMISSED')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_daily_work_items_tenant_status_created
    ON daily_work_items (tenant_id, status, created_at DESC);

CREATE TABLE IF NOT EXISTS unified_threads (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    customer_id TEXT,
    channel TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'open',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS unified_messages (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    thread_id TEXT NOT NULL REFERENCES unified_threads(id) ON DELETE CASCADE,
    sender_type TEXT NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS unified_triage_actions (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    thread_id TEXT NOT NULL REFERENCES unified_threads(id) ON DELETE CASCADE,
    action_type TEXT NOT NULL,
    action_payload TEXT,
    status TEXT NOT NULL DEFAULT 'pending',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_unified_messages_thread_created
    ON unified_messages (thread_id, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_unified_triage_tenant_status_created
    ON unified_triage_actions (tenant_id, status, created_at DESC);

CREATE TABLE IF NOT EXISTS agent_event_subscriptions (
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    agent_role TEXT NOT NULL,
    topic TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (tenant_id, agent_role, topic)
);

CREATE INDEX IF NOT EXISTS idx_agent_event_subscriptions_topic
    ON agent_event_subscriptions (tenant_id, topic);

ALTER TABLE inbound_signals ENABLE ROW LEVEL SECURITY;
ALTER TABLE daily_work_items ENABLE ROW LEVEL SECURITY;
ALTER TABLE unified_threads ENABLE ROW LEVEL SECURITY;
ALTER TABLE unified_messages ENABLE ROW LEVEL SECURITY;
ALTER TABLE unified_triage_actions ENABLE ROW LEVEL SECURITY;
ALTER TABLE agent_event_subscriptions ENABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_inbound_signals ON inbound_signals;
CREATE POLICY tenant_isolation_inbound_signals ON inbound_signals
    USING (tenant_id = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

DROP POLICY IF EXISTS tenant_isolation_daily_work_items ON daily_work_items;
CREATE POLICY tenant_isolation_daily_work_items ON daily_work_items
    USING (tenant_id = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

DROP POLICY IF EXISTS tenant_isolation_unified_threads ON unified_threads;
CREATE POLICY tenant_isolation_unified_threads ON unified_threads
    USING (tenant_id = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

DROP POLICY IF EXISTS tenant_isolation_unified_messages ON unified_messages;
CREATE POLICY tenant_isolation_unified_messages ON unified_messages
    USING (tenant_id = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

DROP POLICY IF EXISTS tenant_isolation_unified_triage_actions ON unified_triage_actions;
CREATE POLICY tenant_isolation_unified_triage_actions ON unified_triage_actions
    USING (tenant_id = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

DROP POLICY IF EXISTS tenant_isolation_agent_event_subscriptions ON agent_event_subscriptions;
CREATE POLICY tenant_isolation_agent_event_subscriptions ON agent_event_subscriptions
    USING (tenant_id = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

GRANT ALL PRIVILEGES ON inbound_signals, daily_work_items, unified_threads,
    unified_messages, unified_triage_actions, agent_event_subscriptions
TO ohc_bypassrls;
