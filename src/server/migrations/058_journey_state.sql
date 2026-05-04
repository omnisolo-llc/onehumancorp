-- Create journey_states table
CREATE TABLE IF NOT EXISTS journey_states (
    tenant_id VARCHAR(255) PRIMARY KEY,
    phase VARCHAR(50) NOT NULL,
    updated_at BIGINT NOT NULL
);

-- Enable RLS
ALTER TABLE journey_states ENABLE ROW LEVEL SECURITY;

-- Create policy for tenant isolation
CREATE POLICY journey_states_tenant_isolation_policy ON journey_states
    USING (tenant_id = current_setting('app.current_tenant', true));
