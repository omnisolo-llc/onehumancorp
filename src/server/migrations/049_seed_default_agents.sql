-- 049_seed_default_agents.sql
-- Add is_default column to agents to distinguish between system-seeded department agents and user-created agents.

ALTER TABLE agents ADD COLUMN is_default BOOLEAN NOT NULL DEFAULT FALSE;
