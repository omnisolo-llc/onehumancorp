-- +goose Up
-- Migration 111: Add customer_id to inbox_messages

ALTER TABLE inbox_messages
ADD COLUMN IF NOT EXISTS customer_id TEXT;

-- +goose Down
ALTER TABLE inbox_messages
DROP COLUMN IF EXISTS customer_id;
DO $$
BEGIN
    IF to_regclass('inbox_messages') IS NOT NULL THEN
        ALTER TABLE inbox_messages ENABLE ROW LEVEL SECURITY;
        DROP POLICY IF EXISTS tenant_isolation_inbox_messages ON inbox_messages;
        CREATE POLICY tenant_isolation_inbox_messages ON inbox_messages USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END
$$;
