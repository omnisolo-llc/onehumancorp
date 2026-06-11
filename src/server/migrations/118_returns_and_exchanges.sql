CREATE TABLE IF NOT EXISTS return_policies (
    tenant_id TEXT PRIMARY KEY,
    return_window_days INT DEFAULT 30,
    auto_approve BOOLEAN DEFAULT true,
    auto_refund_on_scan BOOLEAN DEFAULT true
);

CREATE TABLE IF NOT EXISTS return_requests (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    order_id TEXT NOT NULL,
    reason TEXT,
    status TEXT NOT NULL DEFAULT 'pending',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Enable RLS
ALTER TABLE return_policies ENABLE ROW LEVEL SECURITY;
ALTER TABLE return_requests ENABLE ROW LEVEL SECURITY;

-- Policies for return_policies
CREATE POLICY return_policies_isolation_policy ON return_policies
    USING (tenant_id = current_setting('app.current_tenant', true));

-- Policies for return_requests
CREATE POLICY return_requests_isolation_policy ON return_requests
    USING (tenant_id = current_setting('app.current_tenant', true));
