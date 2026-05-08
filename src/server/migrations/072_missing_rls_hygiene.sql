-- 072_missing_rls_hygiene.sql
-- Enforce ROW LEVEL SECURITY on recently identified tables to ensure multi-tenant isolation

ALTER TABLE swarm_tasks ENABLE ROW LEVEL SECURITY;
ALTER TABLE state_machine_transitions ENABLE ROW LEVEL SECURITY;
