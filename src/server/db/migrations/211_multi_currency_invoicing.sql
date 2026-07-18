-- +goose Up
-- Migration 211: Multi-Currency Checkout and Localized Invoicing

CREATE TABLE IF NOT EXISTS currencies (
    code TEXT PRIMARY KEY,
    exchange_rate DECIMAL NOT NULL DEFAULT 1.0,
    last_updated TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS product_prices (
    product_id TEXT NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    currency_code TEXT NOT NULL REFERENCES currencies(code) ON DELETE CASCADE,
    price_cents BIGINT NOT NULL,
    PRIMARY KEY (product_id, currency_code)
);
ALTER TABLE product_prices ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_product_prices ON product_prices
    USING (EXISTS (SELECT 1 FROM products p WHERE p.id = product_id AND p.tenant_id = current_setting('app.current_tenant', true)))
    WITH CHECK (EXISTS (SELECT 1 FROM products p WHERE p.id = product_id AND p.tenant_id = current_setting('app.current_tenant', true)));

CREATE TABLE IF NOT EXISTS invoices (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    order_id TEXT NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
    currency_code TEXT NOT NULL REFERENCES currencies(code),
    converted_base_amount_cents BIGINT NOT NULL,
    tax_details JSONB NOT NULL DEFAULT '{}'::jsonb,
    pdf_url TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
ALTER TABLE invoices ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_invoices ON invoices
    USING (tenant_id = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

ALTER TABLE tenants ADD COLUMN IF NOT EXISTS base_currency TEXT DEFAULT 'USD';
ALTER TABLE tenants ADD COLUMN IF NOT EXISTS enabled_currencies JSONB DEFAULT '["USD"]'::jsonb;

-- +goose Down
ALTER TABLE tenants DROP COLUMN IF NOT EXISTS enabled_currencies;
ALTER TABLE tenants DROP COLUMN IF NOT EXISTS base_currency;
DROP TABLE IF EXISTS invoices;
DROP TABLE IF EXISTS product_prices;
DROP TABLE IF EXISTS currencies;
