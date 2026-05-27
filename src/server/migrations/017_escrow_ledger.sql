-- Migration 017: Invisible Milestone & Escrow Ledger

CREATE TABLE IF NOT EXISTS project_escrows (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    total_amount DOUBLE PRECISION NOT NULL,
    fbo_account_id TEXT NOT NULL,
    status TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS escrow_milestones (
    id TEXT PRIMARY KEY,
    escrow_id TEXT NOT NULL REFERENCES project_escrows(id) ON DELETE CASCADE,
    tenant_id TEXT NOT NULL,
    release_amount DOUBLE PRECISION NOT NULL,
    status TEXT NOT NULL,
    proof_required TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS ledger_transactions (
    id TEXT PRIMARY KEY,
    escrow_id TEXT NOT NULL REFERENCES project_escrows(id) ON DELETE CASCADE,
    tenant_id TEXT NOT NULL,
    amount DOUBLE PRECISION NOT NULL,
    from_account TEXT NOT NULL,
    to_account TEXT NOT NULL,
    timestamp TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Enable RLS for multi-tenant isolation
ALTER TABLE project_escrows ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_project_escrows ON project_escrows USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE escrow_milestones ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_escrow_milestones ON escrow_milestones USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE ledger_transactions ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_ledger_transactions ON ledger_transactions USING (tenant_id::text = current_setting('app.current_tenant', true));
