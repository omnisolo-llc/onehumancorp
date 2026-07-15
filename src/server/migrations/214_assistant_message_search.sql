DO $$
BEGIN
    IF to_regclass('assistant_messages') IS NOT NULL THEN
        CREATE INDEX IF NOT EXISTS idx_assistant_messages_tenant_created_at
            ON assistant_messages (tenant_id, created_at DESC, id);

        CREATE INDEX IF NOT EXISTS idx_assistant_messages_content_fts
            ON assistant_messages
            USING GIN (to_tsvector('simple', content));
    END IF;
END
$$;
