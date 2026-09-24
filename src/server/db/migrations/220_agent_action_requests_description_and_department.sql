-- +goose Up
ALTER TABLE agent_action_requests ADD COLUMN IF NOT EXISTS description TEXT;
ALTER TABLE agent_action_requests ADD COLUMN IF NOT EXISTS department_type TEXT;

-- +goose Down
ALTER TABLE agent_action_requests DROP COLUMN IF EXISTS description;
ALTER TABLE agent_action_requests DROP COLUMN IF EXISTS department_type;
