ALTER TABLE inbox_messages
ADD COLUMN IF NOT EXISTS customer_id TEXT;

DO $$
BEGIN
    IF to_regclass('chat_messages') IS NOT NULL THEN
        ALTER TABLE chat_messages ADD COLUMN IF NOT EXISTS customer_id TEXT;
    END IF;
END
$$;
