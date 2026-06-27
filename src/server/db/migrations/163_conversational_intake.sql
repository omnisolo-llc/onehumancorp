-- +goose Up

CREATE TABLE IF NOT EXISTS conversational_intakes (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    customer_id TEXT,
    inbox_message_id TEXT,
    status TEXT NOT NULL DEFAULT 'pending' CHECK (status IN ('pending', 'quote_sent', 'payment_pending', 'confirmed')),
    context TEXT,
    service_name TEXT,
    suggested_price DOUBLE PRECISION,
    suggested_time TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_conversational_intakes_tenant_id ON conversational_intakes(tenant_id);
CREATE INDEX IF NOT EXISTS idx_conversational_intakes_message_id ON conversational_intakes(inbox_message_id);

ALTER TABLE conversational_intakes ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_conversational_intakes ON conversational_intakes;
CREATE POLICY tenant_isolation_conversational_intakes
ON conversational_intakes
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- +goose Down
DROP POLICY IF EXISTS tenant_isolation_conversational_intakes ON conversational_intakes;
DROP TABLE IF EXISTS conversational_intakes CASCADE;
