-- Migration 1001: Create missing omnichannel and quoting tables and fix quotes schema

CREATE TABLE IF NOT EXISTS omni_inbox_messages (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    source TEXT NOT NULL,
    original_content TEXT NOT NULL,
    translated_content TEXT NOT NULL,
    source_language TEXT,
    target_language TEXT NOT NULL,
    draft_reply TEXT,
    status TEXT NOT NULL DEFAULT 'unread',
    sender_id TEXT,
    customer_id TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE omni_inbox_messages ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_omni_inbox_messages ON omni_inbox_messages;
CREATE POLICY tenant_isolation_omni_inbox_messages ON omni_inbox_messages USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));


CREATE TABLE IF NOT EXISTS quote_line_items (
    id TEXT PRIMARY KEY,
    quote_id TEXT NOT NULL REFERENCES quotes(id) ON DELETE CASCADE,
    description TEXT NOT NULL,
    unit_price_cents BIGINT NOT NULL,
    quantity INTEGER NOT NULL DEFAULT 1,
    is_optional BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE quote_line_items ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_quote_line_items ON quote_line_items;
CREATE POLICY tenant_isolation_quote_line_items ON quote_line_items USING (
    quote_id IN (SELECT id FROM quotes WHERE tenant_id::text = current_setting('app.current_tenant', true))
) WITH CHECK (
    quote_id IN (SELECT id FROM quotes WHERE tenant_id::text = current_setting('app.current_tenant', true))
);


CREATE TABLE IF NOT EXISTS pricing_heuristics (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    service_category TEXT NOT NULL,
    base_rate_cents BIGINT NOT NULL,
    materials_markup_percentage NUMERIC NOT NULL,
    instructions TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE pricing_heuristics ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_pricing_heuristics ON pricing_heuristics;
CREATE POLICY tenant_isolation_pricing_heuristics ON pricing_heuristics USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));


ALTER TABLE quotes ADD COLUMN IF NOT EXISTS customer_id TEXT REFERENCES customers(id) ON DELETE SET NULL;
ALTER TABLE quotes ADD COLUMN IF NOT EXISTS last_follow_up_at TIMESTAMPTZ;
ALTER TABLE quotes ADD COLUMN IF NOT EXISTS follow_up_count INTEGER DEFAULT 0;

DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name='quotes' AND column_name='total_amount') THEN
        ALTER TABLE quotes RENAME COLUMN total_amount TO total_amount_cents;
    END IF;
    IF EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name='quotes' AND column_name='required_deposit') THEN
        ALTER TABLE quotes RENAME COLUMN required_deposit TO required_deposit_cents;
    END IF;
    IF EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name='quotes' AND column_name='checkout_url') THEN
        ALTER TABLE quotes RENAME COLUMN checkout_url TO stripe_payment_link;
    END IF;
END
$$;
