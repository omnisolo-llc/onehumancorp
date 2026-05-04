CREATE EXTENSION IF NOT EXISTS vector;

-- Upgrade TEXT payload columns to JSONB on postgres
ALTER TABLE kairos_shared_tasks ALTER COLUMN payload TYPE JSONB USING payload::jsonb;
ALTER TABLE kairos_shared_tasks ALTER COLUMN dependencies TYPE JSONB USING dependencies::jsonb;
ALTER TABLE kairos_sub_agent_jobs ALTER COLUMN payload TYPE JSONB USING payload::jsonb;

-- Vector column
ALTER TABLE autodream_vector_memories ADD COLUMN embedding_vec vector(1536);

-- RLS Policies
ALTER TABLE kairos_shared_tasks ENABLE ROW LEVEL SECURITY;
ALTER TABLE kairos_state_transitions ENABLE ROW LEVEL SECURITY;
ALTER TABLE kairos_sub_agent_jobs ENABLE ROW LEVEL SECURITY;
ALTER TABLE autodream_vector_memories ENABLE ROW LEVEL SECURITY;

CREATE POLICY "tenant_isolation_kairos_shared_tasks" ON kairos_shared_tasks FOR ALL USING (tenant_id = current_setting('app.current_tenant', true));
CREATE POLICY "tenant_isolation_kairos_state_transitions" ON kairos_state_transitions FOR ALL USING (tenant_id = current_setting('app.current_tenant', true));
CREATE POLICY "tenant_isolation_kairos_sub_agent_jobs" ON kairos_sub_agent_jobs FOR ALL USING (tenant_id = current_setting('app.current_tenant', true));
CREATE POLICY "tenant_isolation_autodream_vector_memories" ON autodream_vector_memories FOR ALL USING (tenant_id = current_setting('app.current_tenant', true));

CREATE POLICY bypass_rls_kairos_shared_tasks ON kairos_shared_tasks FOR ALL TO bypassrls USING (true) WITH CHECK (true);
CREATE POLICY bypass_rls_kairos_state_transitions ON kairos_state_transitions FOR ALL TO bypassrls USING (true) WITH CHECK (true);
CREATE POLICY bypass_rls_kairos_sub_agent_jobs ON kairos_sub_agent_jobs FOR ALL TO bypassrls USING (true) WITH CHECK (true);
CREATE POLICY bypass_rls_autodream_vector_memories ON autodream_vector_memories FOR ALL TO bypassrls USING (true) WITH CHECK (true);
