-- +goose Up
-- Enable Row Level Security on remaining multi-tenant tables
ALTER TABLE consolidated_memory ENABLE ROW LEVEL SECURITY;
ALTER TABLE shared_tasks_decomposition ENABLE ROW LEVEL SECURITY;
ALTER TABLE task_dependencies ENABLE ROW LEVEL SECURITY;
ALTER TABLE tasks ENABLE ROW LEVEL SECURITY;

-- +goose Down
-- Revert RLS enablement
ALTER TABLE tasks DISABLE ROW LEVEL SECURITY;
ALTER TABLE task_dependencies DISABLE ROW LEVEL SECURITY;
ALTER TABLE shared_tasks_decomposition DISABLE ROW LEVEL SECURITY;
ALTER TABLE consolidated_memory DISABLE ROW LEVEL SECURITY;
