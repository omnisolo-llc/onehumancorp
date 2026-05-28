CREATE TABLE IF NOT EXISTS profit_ledger_entries (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    entry_type TEXT NOT NULL CHECK (entry_type IN ('revenue', 'cogs', 'expense', 'fee', 'refund')),
    amount_cents BIGINT NOT NULL DEFAULT 0,
    source_type TEXT NOT NULL DEFAULT 'manual',
    source_id TEXT,
    plain_language_label TEXT NOT NULL DEFAULT '',
    occurred_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    metadata JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_profit_ledger_entries_tenant_day
    ON profit_ledger_entries (tenant_id, occurred_at);

CREATE INDEX IF NOT EXISTS idx_profit_ledger_entries_source
    ON profit_ledger_entries (tenant_id, source_type, source_id);
