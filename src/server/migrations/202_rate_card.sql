CREATE TABLE IF NOT EXISTS rate_cards (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    name TEXT NOT NULL,
    rules JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS rate_cards_tenant_id_idx ON rate_cards(tenant_id);

ALTER TABLE rate_cards ENABLE ROW LEVEL SECURITY;

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT FROM pg_policies WHERE tablename = 'rate_cards' AND policyname = 'tenant_isolation_rate_cards'
    ) THEN
        CREATE POLICY tenant_isolation_rate_cards ON rate_cards
            FOR ALL
            USING (tenant_id = current_setting('app.current_tenant', true));
    END IF;
END $$;
