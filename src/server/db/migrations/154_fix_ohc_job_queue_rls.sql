-- +goose Up
-- Make sure RLS is properly enforced to make multi-tenant polling secure
ALTER TABLE ohc_job_queue ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ohc_job_queue ON ohc_job_queue;
CREATE POLICY tenant_isolation_ohc_job_queue ON ohc_job_queue USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));


ALTER TABLE conversations ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_conversations ON conversations;
CREATE POLICY tenant_isolation_conversations ON conversations USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE messages ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_messages ON messages;
CREATE POLICY tenant_isolation_messages ON messages USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE ai_drafts ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ai_drafts ON ai_drafts;
CREATE POLICY tenant_isolation_ai_drafts ON ai_drafts USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- +goose Down
-- Reverting RLS changes
