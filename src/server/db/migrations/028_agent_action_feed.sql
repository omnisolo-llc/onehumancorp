-- Unified Agent Action Feed

CREATE TYPE action_card_state AS ENUM ('PENDING_APPROVAL', 'APPROVED', 'REJECTED', 'EDITED', 'EXPIRED', 'EXECUTED');

CREATE TABLE IF NOT EXISTS action_cards (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id TEXT NOT NULL,
    agent_id TEXT NOT NULL,
    trigger_event TEXT NOT NULL,
    context_summary TEXT NOT NULL,
    proposed_action JSONB NOT NULL,
    state action_card_state NOT NULL DEFAULT 'PENDING_APPROVAL',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- RLS
ALTER TABLE action_cards ENABLE ROW LEVEL SECURITY;

CREATE POLICY tenant_isolation_policy_action_cards
    ON action_cards
    USING (tenant_id = current_setting('app.current_tenant', true));
