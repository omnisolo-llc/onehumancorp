-- Migration 022: Sync missing tables and enforce RLS for multi-tenant safety

-- Inbox Messages
CREATE TABLE IF NOT EXISTS inbox_messages (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    source TEXT,
    content TEXT,
    draft_reply TEXT,
    status TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    _sync_status TEXT DEFAULT 'pending',
    version INTEGER DEFAULT 1
);

-- Interactions
CREATE TABLE IF NOT EXISTS interactions (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    customer_id TEXT NOT NULL,
    channel TEXT NOT NULL,
    content TEXT NOT NULL,
    embedding vector(1536),
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    _sync_status TEXT DEFAULT 'pending',
    version INTEGER DEFAULT 1
);

-- Agent Actions
CREATE TABLE IF NOT EXISTS agent_actions (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    agent_id TEXT NOT NULL,
    interaction_id TEXT REFERENCES interactions(id) ON DELETE SET NULL,
    action_type TEXT NOT NULL,
    payload JSONB DEFAULT '{}',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    _sync_status TEXT DEFAULT 'pending',
    version INTEGER DEFAULT 1
);

-- Enable RLS and add policies
ALTER TABLE inbox_messages ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_inbox_messages ON inbox_messages;
CREATE POLICY tenant_isolation_inbox_messages ON inbox_messages USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE interactions ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_interactions ON interactions;
CREATE POLICY tenant_isolation_interactions ON interactions USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE agent_actions ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_agent_actions ON agent_actions;
CREATE POLICY tenant_isolation_agent_actions ON agent_actions USING (tenant_id::text = current_setting('app.current_tenant', true));

-- business_milestones was missing RLS in Postgres
ALTER TABLE business_milestones ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_business_milestones ON business_milestones;
CREATE POLICY tenant_isolation_business_milestones ON business_milestones USING (tenant_id::text = current_setting('app.current_tenant', true));
