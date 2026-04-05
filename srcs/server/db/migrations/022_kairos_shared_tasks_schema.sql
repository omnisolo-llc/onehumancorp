-- Add missing columns to shared_tasks for KAIROS
ALTER TABLE shared_tasks ADD COLUMN parent_plan_id TEXT;

-- Update task_dependencies indices
CREATE INDEX IF NOT EXISTS idx_task_deps_task_022 ON task_dependencies(task_id);
CREATE INDEX IF NOT EXISTS idx_task_deps_depends_022 ON task_dependencies(depends_on_task_id);
