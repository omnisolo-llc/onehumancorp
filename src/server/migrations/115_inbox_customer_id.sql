ALTER TABLE inbox_messages
ADD COLUMN IF NOT EXISTS customer_id TEXT;

DO $$
BEGIN
    IF to_regclass('omni_inbox_messages') IS NOT NULL THEN
        ALTER TABLE omni_inbox_messages ADD COLUMN IF NOT EXISTS customer_id TEXT;
    END IF;
END
$$;
