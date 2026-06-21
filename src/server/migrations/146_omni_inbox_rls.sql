-- +goose Up
-- +goose StatementBegin
-- Migration 145: Add RLS policy to omni_inbox_messages

DO $$
BEGIN
    IF to_regclass('omni_inbox_messages') IS NOT NULL THEN
        IF NOT EXISTS (
            SELECT 1
            FROM pg_policies
            WHERE schemaname = current_schema()
              AND tablename = 'omni_inbox_messages'
              AND policyname = 'tenant_isolation_omni_inbox_messages'
        ) THEN
            EXECUTE 'CREATE POLICY tenant_isolation_omni_inbox_messages ON omni_inbox_messages USING (tenant_id::text = current_setting(''app.current_tenant'', true)) WITH CHECK (tenant_id::text = current_setting(''app.current_tenant'', true))';
        END IF;
    END IF;
END
$$;
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
DO $$
BEGIN
    IF to_regclass('omni_inbox_messages') IS NOT NULL THEN
        DROP POLICY IF EXISTS tenant_isolation_omni_inbox_messages ON omni_inbox_messages;
    END IF;
END
$$;
-- +goose StatementEnd
