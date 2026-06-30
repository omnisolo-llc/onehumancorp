CREATE TABLE IF NOT EXISTS dispute_evidence_packages (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    dispute_id TEXT NOT NULL,
    charge_id TEXT NOT NULL,
    customer_id TEXT,
    reason TEXT NOT NULL,
    amount INTEGER NOT NULL,
    currency TEXT NOT NULL,
    evidence_payload TEXT NOT NULL DEFAULT '{}',
    status TEXT NOT NULL DEFAULT 'draft',
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
