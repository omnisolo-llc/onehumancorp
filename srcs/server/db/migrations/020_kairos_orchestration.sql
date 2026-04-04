-- Ensure swarm_tasks and autodream_memories schemas are fully supported
-- These were already mostly created in 008_swarm_tasks.sql and 007_teammate_mesh_and_autodream.sql
-- Let's just make sure shared_tasks has mission_id which might have been dropped in 013_shared_tasks.sql

ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS mission_id TEXT;
