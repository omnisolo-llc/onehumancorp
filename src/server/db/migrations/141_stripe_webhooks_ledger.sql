CREATE TABLE IF NOT EXISTS stripe_webhooks (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    stripe_event_id TEXT NOT NULL,
    type TEXT NOT NULL,
    data JSONB NOT NULL,
    status TEXT NOT NULL DEFAULT 'PENDING',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (tenant_id, stripe_event_id)
);

CREATE INDEX IF NOT EXISTS idx_stripe_webhooks_polling
ON stripe_webhooks(status)
WHERE status = 'PENDING';

ALTER TABLE stripe_webhooks ENABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_stripe_webhooks ON stripe_webhooks;
CREATE POLICY tenant_isolation_stripe_webhooks
ON stripe_webhooks
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
