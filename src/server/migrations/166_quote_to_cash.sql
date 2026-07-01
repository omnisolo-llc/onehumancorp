-- +goose Up

-- We need to make sure the core `quotes` and `quote_line_items` exist properly and handle state transitions correctly, per the Acceptance Criteria.
-- Note: 'quotes' table already exists from 078_quote_engine.sql, let's update it.
ALTER TABLE quotes ADD COLUMN IF NOT EXISTS customer_id TEXT;
ALTER TABLE quotes ADD COLUMN IF NOT EXISTS total_amount_cents BIGINT DEFAULT 0;
ALTER TABLE quotes ADD COLUMN IF NOT EXISTS required_deposit_cents BIGINT DEFAULT 0;
ALTER TABLE quotes ADD COLUMN IF NOT EXISTS stripe_payment_link TEXT;
ALTER TABLE quotes ADD COLUMN IF NOT EXISTS created_at TIMESTAMPTZ NOT NULL DEFAULT NOW();
ALTER TABLE quotes ADD COLUMN IF NOT EXISTS updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW();

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
CREATE POLICY tenant_isolation_quote_line_items ON quote_line_items USING (
    quote_id IN (SELECT id FROM quotes WHERE tenant_id::text = current_setting('app.current_tenant', true))
) WITH CHECK (
    quote_id IN (SELECT id FROM quotes WHERE tenant_id::text = current_setting('app.current_tenant', true))
);
