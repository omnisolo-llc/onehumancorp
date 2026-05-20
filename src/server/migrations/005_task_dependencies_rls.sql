-- Enable Row Level Security on the task_dependencies table
ALTER TABLE task_dependencies ENABLE ROW LEVEL SECURITY;

-- Create policy to restrict access to records belonging to the current tenant
CREATE POLICY task_dependencies_tenant_policy ON task_dependencies
    USING (tenant_id = current_setting('app.current_tenant')::uuid);
