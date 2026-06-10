ALTER TABLE inbox_messages
ADD COLUMN IF NOT EXISTS customer_id TEXT;
ALTER TABLE inbox_messages ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_inbox_messages ON inbox_messages;
CREATE POLICY tenant_isolation_inbox_messages ON inbox_messages USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
