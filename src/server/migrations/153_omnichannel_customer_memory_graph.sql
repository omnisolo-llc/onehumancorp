-- +goose Up
CREATE EXTENSION IF NOT EXISTS vector;

-- Customers table was added earlier with a TEXT id
ALTER TABLE customers ADD COLUMN IF NOT EXISTS embedding vector(1536);
ALTER TABLE customers ADD COLUMN IF NOT EXISTS profile_summary JSONB;

-- Note: In OHC, customer_id is TEXT
CREATE TABLE IF NOT EXISTS interaction_events (
    id UUID PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    customer_id TEXT NOT NULL REFERENCES customers(id) ON DELETE CASCADE,
    channel TEXT NOT NULL,
    raw_content TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE interaction_events ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_interaction_events ON interaction_events
    USING (tenant_id = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

CREATE TABLE IF NOT EXISTS context_snippets (
    id UUID PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    customer_id TEXT NOT NULL REFERENCES customers(id) ON DELETE CASCADE,
    category TEXT NOT NULL,
    extracted_value TEXT NOT NULL,
    embedding vector(1536),
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE context_snippets ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_context_snippets ON context_snippets
    USING (tenant_id = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- Update interaction_events to have an AI job queue
CREATE TABLE IF NOT EXISTS interaction_event_jobs (
    job_id UUID PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    interaction_event_id UUID NOT NULL REFERENCES interaction_events(id) ON DELETE CASCADE,
    status TEXT NOT NULL DEFAULT 'pending',
    retry_count INTEGER DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE interaction_event_jobs ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_interaction_event_jobs ON interaction_event_jobs
    USING (tenant_id = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
