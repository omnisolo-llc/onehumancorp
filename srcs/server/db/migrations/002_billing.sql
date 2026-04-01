-- 002_billing.sql
-- Append-only token usage events for the billing subsystem.

CREATE TABLE IF NOT EXISTS usage_events (
    id                BIGSERIAL PRIMARY KEY,
    agent_id          TEXT NOT NULL,
    agent_role        TEXT NOT NULL,
    organization_id   TEXT NOT NULL,
    model             TEXT NOT NULL,
    prompt_tokens     BIGINT NOT NULL DEFAULT 0,
    completion_tokens BIGINT NOT NULL DEFAULT 0,
    cost_usd          DOUBLE PRECISION NOT NULL DEFAULT 0,
    occurred_at       TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_usage_events_org ON usage_events (organization_id);
CREATE INDEX IF NOT EXISTS idx_usage_events_agent ON usage_events (agent_id);
