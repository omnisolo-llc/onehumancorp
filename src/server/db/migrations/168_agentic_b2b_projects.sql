-- Create projects table
CREATE TABLE IF NOT EXISTS projects (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    name VARCHAR(255) NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'DRAFT',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Enforce multi-tenant isolation
ALTER TABLE projects ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_policy_projects ON projects FOR ALL USING (tenant_id = current_setting('app.current_tenant_id')::UUID);

-- Create milestones table
CREATE TABLE IF NOT EXISTS milestones (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    project_id UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'PENDING',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Enforce multi-tenant isolation
ALTER TABLE milestones ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_policy_milestones ON milestones FOR ALL USING (tenant_id = current_setting('app.current_tenant_id')::UUID);

-- Create milestone_invoices table
CREATE TABLE IF NOT EXISTS milestone_invoices (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    milestone_id UUID NOT NULL REFERENCES milestones(id) ON DELETE CASCADE,
    stripe_invoice_id VARCHAR(255),
    amount_cents BIGINT NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'DRAFT',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Enforce multi-tenant isolation
ALTER TABLE milestone_invoices ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_policy_milestone_invoices ON milestone_invoices FOR ALL USING (tenant_id = current_setting('app.current_tenant_id')::UUID);
