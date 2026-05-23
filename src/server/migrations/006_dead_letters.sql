-- Migration: 006_dead_letters.sql

CREATE TABLE IF NOT EXISTS department_dead_letters (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    event_type TEXT NOT NULL,
    department TEXT NOT NULL,
    payload TEXT NOT NULL,
    error_message TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
ALTER TABLE department_dead_letters ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_department_dead_letters ON department_dead_letters USING (tenant_id::text = current_setting('app.current_tenant', true));
