-- +goose Up
-- +goose StatementBegin
-- Migration 145: Add RLS policy to chat_messages

DO $$
BEGIN
    IF to_regclass('chat_messages') IS NOT NULL THEN
        IF NOT EXISTS (
            SELECT 1
            FROM pg_policies
            WHERE schemaname = current_schema()
              AND tablename = 'chat_messages'
              AND policyname = 'tenant_isolation_chat_messages'
        ) THEN
            EXECUTE 'CREATE POLICY tenant_isolation_chat_messages ON chat_messages USING (tenant_id::text = current_setting(''app.current_tenant'', true)) WITH CHECK (tenant_id::text = current_setting(''app.current_tenant'', true))';
        END IF;
    END IF;
END
$$;
-- +goose StatementEnd
