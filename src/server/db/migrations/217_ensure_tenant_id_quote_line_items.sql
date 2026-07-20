-- +goose Up
-- Add tenant_id to quote_line_items for standard RLS (if missing) and enforce RLS
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name='quote_line_items' AND column_name='tenant_id') THEN
        ALTER TABLE quote_line_items ADD COLUMN tenant_id TEXT;
        -- Backfill tenant_id from quotes if possible, otherwise it will remain null for old rows
        UPDATE quote_line_items qli
        SET tenant_id = q.tenant_id
        FROM quotes q
        WHERE qli.quote_id = q.id;

        ALTER TABLE quote_line_items ALTER COLUMN tenant_id SET NOT NULL;
    END IF;
END
$$;

ALTER TABLE quote_line_items ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_quote_line_items_tenant ON quote_line_items;
CREATE POLICY tenant_isolation_quote_line_items_tenant ON quote_line_items
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
