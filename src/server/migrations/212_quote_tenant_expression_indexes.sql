-- +goose Up
-- Preserve UUID/TEXT schema compatibility for quote API predicates while
-- allowing PostgreSQL to satisfy tenant-scoped lookups from an index.

DO $$
BEGIN
    IF to_regclass('quotes') IS NOT NULL THEN
        CREATE INDEX IF NOT EXISTS idx_quotes_id_text_tenant
            ON quotes ((id::text), tenant_id);
    END IF;

    IF to_regclass('customers') IS NOT NULL THEN
        CREATE INDEX IF NOT EXISTS idx_customers_id_text_tenant
            ON customers ((id::text), tenant_id);
    END IF;

    IF to_regclass('quote_line_items') IS NOT NULL THEN
        CREATE INDEX IF NOT EXISTS idx_quote_line_items_quote_id_text_tenant
            ON quote_line_items ((quote_id::text), tenant_id);
    END IF;
END
$$;
