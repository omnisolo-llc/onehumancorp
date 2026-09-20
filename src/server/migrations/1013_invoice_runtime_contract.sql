-- The active PostgreSQL lineage creates invoices in 078/114. The similarly
-- named file in db/migrations is not executed by SQLx. Add the mounted API's
-- fields here without recreating tables, losing records or inventing payments.
ALTER TABLE invoices ADD COLUMN IF NOT EXISTS client_id TEXT;
ALTER TABLE invoices ADD COLUMN IF NOT EXISTS client_name TEXT;
ALTER TABLE invoices ADD COLUMN IF NOT EXISTS stripe_payment_link TEXT;
ALTER TABLE invoices ADD COLUMN IF NOT EXISTS total_amount_cents INTEGER;
ALTER TABLE invoices ADD COLUMN IF NOT EXISTS amount_paid_cents INTEGER;
ALTER TABLE invoices ADD COLUMN IF NOT EXISTS payment_status TEXT DEFAULT 'unverified';
ALTER TABLE invoices ADD COLUMN IF NOT EXISTS view_count INTEGER DEFAULT 0;
ALTER TABLE invoice_line_items ADD COLUMN IF NOT EXISTS updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP;

-- Keep unknown historical amounts/payments NULL. Only derive minor units when
-- the stored currency and exact amount support the existing two-decimal API.
UPDATE invoices SET total_amount_cents = ROUND(total_amount * 100)::INTEGER
WHERE total_amount_cents IS NULL AND total_amount >= 0 AND total_amount <= 999999.99
  AND ABS(total_amount * 100 - ROUND(total_amount * 100)) <= 0.00001
  AND LOWER(currency) IN ('usd','eur','gbp','cad','aud','nzd','sgd','hkd','inr','brl','mxn');
UPDATE invoices SET client_id = customer_id WHERE client_id IS NULL AND customer_id IS NOT NULL;
UPDATE invoices AS i SET client_name = c.name FROM customers AS c
WHERE i.client_name IS NULL AND i.client_id = c.id AND i.tenant_id = c.tenant_id;

CREATE INDEX IF NOT EXISTS idx_invoices_tenant_due ON invoices(tenant_id, due_date, id);
ALTER TABLE invoices ENABLE ROW LEVEL SECURITY;
ALTER TABLE invoices FORCE ROW LEVEL SECURITY;
ALTER TABLE invoice_line_items ENABLE ROW LEVEL SECURITY;
ALTER TABLE invoice_line_items FORCE ROW LEVEL SECURITY;
