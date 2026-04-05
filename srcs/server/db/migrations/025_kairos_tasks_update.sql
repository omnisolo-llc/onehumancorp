-- Add parent_plan_id to shared_tasks for KAIROS DAG orchestration
ALTER TABLE shared_tasks ADD COLUMN parent_plan_id VARCHAR;
