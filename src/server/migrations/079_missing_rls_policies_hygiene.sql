-- 079_missing_rls_policies_hygiene.sql
-- Hardening hygiene audit: Add missing RLS policies to tables that have tenant_id but were missing ENABLE ROW LEVEL SECURITY

ALTER TABLE IF EXISTS consolidated_memory ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS crdt_deltas ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS local_mcp_rag_tasks ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS shared_tasks ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS task_dependencies ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS department_tasks ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS agent_violations ENABLE ROW LEVEL SECURITY;
