-- +goose Up
-- Migration 126: Unified Omnichannel Customer Context & AI Memory Architecture

CREATE EXTENSION IF NOT EXISTS vector;

-- Ensure customers table exists, although we know it should from previous migrations, adding as safety if run out of order
CREATE TABLE IF NOT EXISTS customers (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    name TEXT NOT NULL,
    email TEXT,
    phone TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- 1. Create interaction_events table (stores events like message, purchase, booking)
CREATE TABLE IF NOT EXISTS interaction_events (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    customer_id TEXT NOT NULL REFERENCES customers(id) ON DELETE CASCADE,
    event_type TEXT NOT NULL,
    payload JSONB NOT NULL DEFAULT '{}',
    occurred_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);


-- Note: customer_identities already exists from 035_customer_identities.sql
-- We will use that table:
-- id TEXT, tenant_id TEXT, customer_id TEXT, channel TEXT, channel_identity TEXT

-- RLS setup for interaction_events
DO $$
BEGIN
    IF to_regclass('interaction_events') IS NOT NULL THEN
        ALTER TABLE interaction_events ENABLE ROW LEVEL SECURITY;
        CREATE POLICY tenant_isolation_interaction_events ON interaction_events
            USING (tenant_id::text = current_setting('app.current_tenant', true))
            WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END
$$;

-- Create vector embeddings table
CREATE TABLE IF NOT EXISTS interaction_event_embeddings (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    interaction_event_id TEXT NOT NULL REFERENCES interaction_events(id) ON DELETE CASCADE,
    embedding vector(1536),
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- RLS setup for interaction_event_embeddings
DO $$
BEGIN
    IF to_regclass('interaction_event_embeddings') IS NOT NULL THEN
        ALTER TABLE interaction_event_embeddings ENABLE ROW LEVEL SECURITY;
        CREATE POLICY tenant_isolation_interaction_event_embeddings ON interaction_event_embeddings
            USING (tenant_id::text = current_setting('app.current_tenant', true))
            WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END
$$;

-- +goose Down
DO $$
BEGIN
    DROP POLICY IF EXISTS tenant_isolation_interaction_event_embeddings ON interaction_event_embeddings;
    DROP POLICY IF EXISTS tenant_isolation_interaction_events ON interaction_events;
END
$$;

DROP TABLE IF EXISTS interaction_event_embeddings CASCADE;
DROP TABLE IF EXISTS interaction_events CASCADE;
