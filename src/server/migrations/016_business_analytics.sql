-- Migration 016: Invisible Business Analytics

CREATE TABLE IF NOT EXISTS business_events (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    customer_id TEXT,
    event_type TEXT NOT NULL,
    payload TEXT NOT NULL DEFAULT '{}',
    occurred_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE business_events ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_business_events ON business_events USING (tenant_id::text = current_setting('app.current_tenant', true));

CREATE TABLE IF NOT EXISTS daily_briefings (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    plain_language_summary TEXT NOT NULL,
    briefing_date DATE NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE daily_briefings ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_daily_briefings ON daily_briefings USING (tenant_id::text = current_setting('app.current_tenant', true));

CREATE TABLE IF NOT EXISTS proactive_actions (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    action_type TEXT NOT NULL,
    ai_department_owner TEXT NOT NULL,
    status TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE proactive_actions ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_proactive_actions ON proactive_actions USING (tenant_id::text = current_setting('app.current_tenant', true));
