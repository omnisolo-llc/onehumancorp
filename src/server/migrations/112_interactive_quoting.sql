-- +goose Up
-- Migration 112: Interactive Quoting Alignment

-- Ensure customer_id exists in quotes
ALTER TABLE quotes ADD COLUMN IF NOT EXISTS customer_id UUID;

-- Convert quotes.id to UUID if it's currently TEXT
-- This assumes existing IDs are valid UUID strings or the table is empty.
ALTER TABLE quotes ALTER COLUMN id TYPE UUID USING id::uuid;

-- Create quote_line_items if it doesn't exist
CREATE TABLE IF NOT EXISTS quote_line_items (
    id UUID PRIMARY KEY,
    quote_id UUID NOT NULL REFERENCES quotes(id) ON DELETE CASCADE,
    description TEXT NOT NULL,
    unit_price_cents BIGINT NOT NULL,
    quantity INTEGER NOT NULL DEFAULT 1,
    is_optional BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Enable RLS on quote_line_items
ALTER TABLE quote_line_items ENABLE ROW LEVEL SECURITY;

-- Add RLS policy for quote_line_items
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_policies WHERE policyname = 'tenant_isolation_quote_line_items' AND tablename = 'quote_line_items') THEN
        CREATE POLICY tenant_isolation_quote_line_items ON quote_line_items USING (
            quote_id IN (SELECT id FROM quotes WHERE tenant_id::text = current_setting('app.current_tenant', true))
        );
    END IF;
END $$;
