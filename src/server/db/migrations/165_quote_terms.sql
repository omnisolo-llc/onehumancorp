-- +goose Up
CREATE TABLE IF NOT EXISTS quote_terms (
    id UUID PRIMARY KEY,
    quote_id UUID NOT NULL REFERENCES quotes(id) ON DELETE CASCADE,
    term_text TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE quote_terms ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_quote_terms ON quote_terms USING (
    quote_id IN (SELECT id FROM quotes WHERE tenant_id = current_setting('app.current_tenant', true))
) WITH CHECK (
    quote_id IN (SELECT id FROM quotes WHERE tenant_id = current_setting('app.current_tenant', true))
);

-- +goose Down
DROP POLICY IF EXISTS tenant_isolation_quote_terms ON quote_terms;
DROP TABLE IF EXISTS quote_terms CASCADE;
