-- Create shared_task_list view or similar to satisfy the shared task list migration without overwriting 002
CREATE VIEW IF NOT EXISTS shared_task_list AS
SELECT
    id,
    organization_id,
    title,
    description,
    status,
    priority,
    created_at,
    updated_at
FROM shared_tasks_v4;

-- Trigger CI rebuild
-- Trigger CI
-- Trigger CI rebuild 2
