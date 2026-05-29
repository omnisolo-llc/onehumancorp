-- +goose Up
-- Tenant-scoped conversational commerce backbone:
-- inbound omnichannel thread -> AI quote -> checkout session -> order/ledger sync.

CREATE TABLE IF NOT EXISTS conversation_threads (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    customer_id TEXT REFERENCES customers(id) ON DELETE SET NULL,
    source_channel TEXT NOT NULL,
    external_thread_id TEXT NOT NULL,
    external_customer_id TEXT,
    status TEXT NOT NULL DEFAULT 'open',
    intent TEXT,
    metadata JSONB DEFAULT '{}',
    last_message_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (tenant_id, source_channel, external_thread_id)
);

CREATE TABLE IF NOT EXISTS conversation_messages (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    thread_id TEXT REFERENCES conversation_threads(id) ON DELETE CASCADE,
    external_message_id TEXT,
    direction TEXT NOT NULL,
    sender_role TEXT NOT NULL,
    body TEXT NOT NULL,
    ai_intent TEXT,
    quote_id TEXT,
    checkout_session_id TEXT,
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS commerce_quotes (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    thread_id TEXT REFERENCES conversation_threads(id) ON DELETE SET NULL,
    customer_id TEXT REFERENCES customers(id) ON DELETE SET NULL,
    status TEXT NOT NULL DEFAULT 'draft',
    currency TEXT NOT NULL DEFAULT 'USD',
    subtotal_cents BIGINT NOT NULL DEFAULT 0,
    tax_cents BIGINT NOT NULL DEFAULT 0,
    discount_cents BIGINT NOT NULL DEFAULT 0,
    total_cents BIGINT NOT NULL DEFAULT 0,
    line_items JSONB NOT NULL DEFAULT '[]',
    inventory_reservations JSONB NOT NULL DEFAULT '[]',
    expires_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS checkout_sessions (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    quote_id TEXT REFERENCES commerce_quotes(id) ON DELETE SET NULL,
    order_id TEXT REFERENCES orders(id) ON DELETE SET NULL,
    customer_id TEXT REFERENCES customers(id) ON DELETE SET NULL,
    provider TEXT NOT NULL,
    provider_session_id TEXT,
    status TEXT NOT NULL DEFAULT 'created',
    amount_cents BIGINT NOT NULL DEFAULT 0,
    currency TEXT NOT NULL DEFAULT 'USD',
    checkout_url TEXT NOT NULL,
    success_url TEXT,
    cancel_url TEXT,
    expires_at TIMESTAMPTZ,
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS ledger_entries (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    order_id TEXT REFERENCES orders(id) ON DELETE SET NULL,
    checkout_session_id TEXT REFERENCES checkout_sessions(id) ON DELETE SET NULL,
    entry_type TEXT NOT NULL,
    amount_cents BIGINT NOT NULL,
    currency TEXT NOT NULL DEFAULT 'USD',
    provider TEXT,
    provider_event_id TEXT,
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE orders ADD COLUMN IF NOT EXISTS quote_id TEXT REFERENCES commerce_quotes(id) ON DELETE SET NULL;
ALTER TABLE orders ADD COLUMN IF NOT EXISTS checkout_session_id TEXT REFERENCES checkout_sessions(id) ON DELETE SET NULL;
ALTER TABLE orders ADD COLUMN IF NOT EXISTS payment_provider TEXT;
ALTER TABLE orders ADD COLUMN IF NOT EXISTS payment_status TEXT NOT NULL DEFAULT 'unpaid';

CREATE INDEX IF NOT EXISTS conversation_threads_tenant_channel_idx ON conversation_threads (tenant_id, source_channel, status);
CREATE INDEX IF NOT EXISTS conversation_messages_thread_idx ON conversation_messages (tenant_id, thread_id, created_at);
CREATE INDEX IF NOT EXISTS commerce_quotes_thread_idx ON commerce_quotes (tenant_id, thread_id, status);
CREATE INDEX IF NOT EXISTS checkout_sessions_quote_idx ON checkout_sessions (tenant_id, quote_id, status);
CREATE INDEX IF NOT EXISTS ledger_entries_order_idx ON ledger_entries (tenant_id, order_id, created_at);

DO $$
DECLARE
    t_name text;
    pol_name text;
BEGIN
    FOR t_name IN
        SELECT unnest(ARRAY[
            'conversation_threads',
            'conversation_messages',
            'commerce_quotes',
            'checkout_sessions',
            'ledger_entries'
        ])
    LOOP
        EXECUTE format('ALTER TABLE %I ENABLE ROW LEVEL SECURITY', t_name);

        pol_name := format('tenant_isolation_%s', t_name);
        IF NOT EXISTS (
            SELECT 1
            FROM pg_policies
            WHERE schemaname = current_schema()
                AND tablename = t_name
                AND policyname = pol_name
        ) THEN
            EXECUTE format(
                'CREATE POLICY %I ON %I USING (tenant_id::text = current_setting(''app.current_tenant'', true)) WITH CHECK (tenant_id::text = current_setting(''app.current_tenant'', true))',
                pol_name,
                t_name
            );
        END IF;
    END LOOP;
END
$$;

-- +goose Down
DROP INDEX IF EXISTS ledger_entries_order_idx;
DROP INDEX IF EXISTS checkout_sessions_quote_idx;
DROP INDEX IF EXISTS commerce_quotes_thread_idx;
DROP INDEX IF EXISTS conversation_messages_thread_idx;
DROP INDEX IF EXISTS conversation_threads_tenant_channel_idx;

DROP TABLE IF EXISTS ledger_entries CASCADE;
DROP TABLE IF EXISTS checkout_sessions CASCADE;
DROP TABLE IF EXISTS commerce_quotes CASCADE;
DROP TABLE IF EXISTS conversation_messages CASCADE;
DROP TABLE IF EXISTS conversation_threads CASCADE;

ALTER TABLE orders DROP COLUMN IF EXISTS payment_status;
ALTER TABLE orders DROP COLUMN IF EXISTS payment_provider;
ALTER TABLE orders DROP COLUMN IF EXISTS checkout_session_id;
ALTER TABLE orders DROP COLUMN IF EXISTS quote_id;
