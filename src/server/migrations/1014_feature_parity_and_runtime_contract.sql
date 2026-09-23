-- 1014_feature_parity_and_runtime_contract.sql
-- Missing runtime tables, columns, and compatibility views required by real-stack E2E and background workers.

-- 1. Agent Event Subscriptions
CREATE TABLE IF NOT EXISTS agent_event_subscriptions (
    tenant_id TEXT NOT NULL,
    agent_role TEXT NOT NULL,
    topic TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (tenant_id, agent_role, topic)
);
CREATE INDEX IF NOT EXISTS idx_agent_event_subscriptions_tenant ON agent_event_subscriptions(tenant_id);
ALTER TABLE agent_event_subscriptions ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_agent_event_subscriptions ON agent_event_subscriptions;
CREATE POLICY tenant_isolation_agent_event_subscriptions ON agent_event_subscriptions
    USING (tenant_id = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- 2. shared_tasks parity columns: tenant_id, payload
ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS tenant_id TEXT;
ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS payload TEXT;
UPDATE shared_tasks SET tenant_id = organization_id WHERE tenant_id IS NULL;

-- 3. Unified Inbox & Triage (threads, messages, triage actions)
CREATE TABLE IF NOT EXISTS unified_threads (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    customer_id TEXT,
    channel TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'open',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_unified_threads_tenant ON unified_threads(tenant_id);
ALTER TABLE unified_threads ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_unified_threads ON unified_threads;
CREATE POLICY tenant_isolation_unified_threads ON unified_threads
    USING (tenant_id = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

CREATE TABLE IF NOT EXISTS unified_messages (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    thread_id TEXT NOT NULL REFERENCES unified_threads(id) ON DELETE CASCADE,
    sender_type TEXT NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_unified_messages_tenant ON unified_messages(tenant_id);
ALTER TABLE unified_messages ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_unified_messages ON unified_messages;
CREATE POLICY tenant_isolation_unified_messages ON unified_messages
    USING (tenant_id = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

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
CREATE INDEX IF NOT EXISTS idx_unified_triage_actions_tenant ON unified_triage_actions(tenant_id);
ALTER TABLE unified_triage_actions ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_unified_triage_actions ON unified_triage_actions;
CREATE POLICY tenant_isolation_unified_triage_actions ON unified_triage_actions
    USING (tenant_id = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- 4. Inbound signals and daily work items
CREATE TABLE IF NOT EXISTS inbound_signals (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    source TEXT NOT NULL,
    raw_payload JSONB NOT NULL,
    status TEXT NOT NULL DEFAULT 'PENDING' CHECK (status IN ('PENDING', 'PROCESSED', 'FAILED')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_inbound_signals_tenant ON inbound_signals(tenant_id);
ALTER TABLE inbound_signals ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_inbound_signals ON inbound_signals;
CREATE POLICY tenant_isolation_inbound_signals ON inbound_signals
    USING (tenant_id = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

CREATE TABLE IF NOT EXISTS daily_work_items (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    signal_id TEXT REFERENCES inbound_signals(id) ON DELETE SET NULL,
    intent TEXT NOT NULL,
    customer_info JSONB,
    suggested_actions JSONB,
    status TEXT NOT NULL DEFAULT 'PENDING' CHECK (status IN ('PENDING', 'APPROVED', 'DISMISSED')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_daily_work_items_tenant ON daily_work_items(tenant_id, status, created_at DESC);
ALTER TABLE daily_work_items ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_daily_work_items ON daily_work_items;
CREATE POLICY tenant_isolation_daily_work_items ON daily_work_items
    USING (tenant_id = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- 5. Affiliate ledgers commission_amount column
ALTER TABLE affiliate_ledgers ADD COLUMN IF NOT EXISTS commission_amount BIGINT DEFAULT 0;

-- 6. Views for backward-compatible worker / dashboard queries
CREATE OR REPLACE VIEW pos_orders AS SELECT * FROM orders;
CREATE OR REPLACE VIEW growth_team_invites AS SELECT * FROM team_invites;
