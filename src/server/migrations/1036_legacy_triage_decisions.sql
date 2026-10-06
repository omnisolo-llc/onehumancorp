-- The owner decision, edited draft and supported local effect commit together.
-- A retry reads this receipt; it never repeats a provider call or local effect.
CREATE TABLE legacy_triage_decisions (
    tenant_id TEXT NOT NULL,
    action_id TEXT NOT NULL,
    approved BOOLEAN NOT NULL,
    edited_payload TEXT,
    receipt TEXT NOT NULL,
    actor_id TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (tenant_id, action_id)
);
ALTER TABLE legacy_triage_decisions ENABLE ROW LEVEL SECURITY;
ALTER TABLE legacy_triage_decisions FORCE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_legacy_triage_decisions ON legacy_triage_decisions
USING (tenant_id = current_setting('app.current_tenant',true))
WITH CHECK (tenant_id = current_setting('app.current_tenant',true));
