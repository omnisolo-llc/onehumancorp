CREATE TABLE IF NOT EXISTS customer_cohorts (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    criteria_json TEXT DEFAULT '{}',
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_customer_cohorts_tenant ON customer_cohorts(tenant_id);

CREATE TABLE IF NOT EXISTS customer_cohort_members (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    cohort_id TEXT NOT NULL,
    customer_id TEXT NOT NULL,
    added_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(tenant_id, cohort_id, customer_id)
);
CREATE INDEX IF NOT EXISTS idx_customer_cohort_members_tenant_cohort ON customer_cohort_members(tenant_id, cohort_id);

ALTER TABLE customer_cohorts ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_customer_cohorts ON customer_cohorts;
CREATE POLICY tenant_isolation_customer_cohorts
ON customer_cohorts
USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

ALTER TABLE customer_cohort_members ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_customer_cohort_members ON customer_cohort_members;
CREATE POLICY tenant_isolation_customer_cohort_members
ON customer_cohort_members
USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
