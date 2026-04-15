-- Migration: create agent_task_outputs table for database-backed task output.
-- Task output (streaming logs from agent execution) is stored in this table
-- instead of on-disk files, making the system fully stateless from a disk
-- perspective and enabling cloud-native deployments.

CREATE TABLE IF NOT EXISTS agent_task_outputs (
    id          VARCHAR PRIMARY KEY,
    task_id     VARCHAR NOT NULL,
    chunk       TEXT    NOT NULL DEFAULT '',
    appended_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_agent_task_outputs_task_id ON agent_task_outputs(task_id);
