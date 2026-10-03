-- Migration 230: Add description and department_type to agent_action_requests
ALTER TABLE agent_action_requests ADD COLUMN IF NOT EXISTS description TEXT;
ALTER TABLE agent_action_requests ADD COLUMN IF NOT EXISTS department_type TEXT;
