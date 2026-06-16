-- +goose Up
-- Migration 135: Enforce remaining Row Level Security policies for tables missing them

-- 1. customer_identities
ALTER TABLE IF EXISTS customer_identities ENABLE ROW LEVEL SECURITY;
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_policies
        WHERE tablename = 'customer_identities'
        AND policyname = 'tenant_isolation_customer_identities'
    ) THEN
        CREATE POLICY tenant_isolation_customer_identities ON customer_identities
            USING (tenant_id::text = current_setting('app.current_tenant', true))
            WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END
$$;

-- 2. omni_inbox_messages
ALTER TABLE IF EXISTS omni_inbox_messages ENABLE ROW LEVEL SECURITY;
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_policies
        WHERE tablename = 'omni_inbox_messages'
        AND policyname = 'tenant_isolation_omni_inbox_messages'
    ) THEN
        CREATE POLICY tenant_isolation_omni_inbox_messages ON omni_inbox_messages
            USING (tenant_id::text = current_setting('app.current_tenant', true))
            WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END
$$;

-- +goose Down
DROP POLICY IF EXISTS tenant_isolation_customer_identities ON customer_identities;
DROP POLICY IF EXISTS tenant_isolation_omni_inbox_messages ON omni_inbox_messages;
