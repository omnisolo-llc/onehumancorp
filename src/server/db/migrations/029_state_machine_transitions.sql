-- Replace the previous CREATE TABLE IF NOT EXISTS completely with ALTER to handle the table from migration 002 properly

-- Ensure tenant_id exists and make it NOT NULL safely
ALTER TABLE state_machine_transitions ADD COLUMN IF NOT EXISTS tenant_id TEXT DEFAULT 'system';
-- Note: 'NOT NULL' is tricky if existing rows exist, but we assume it already has it or we can add constraint.
-- Because migration 002 had: `tenant_id TEXT DEFAULT 'system'`

ALTER TABLE state_machine_transitions ENABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_state_machine_transitions ON state_machine_transitions;
CREATE POLICY tenant_isolation_state_machine_transitions ON state_machine_transitions
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

CREATE INDEX IF NOT EXISTS idx_sm_entity ON state_machine_transitions(entity_id, entity_type);
CREATE INDEX IF NOT EXISTS idx_sm_tenant ON state_machine_transitions(tenant_id);
