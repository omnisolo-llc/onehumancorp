-- Add updated_at column to agent_missions
ALTER TABLE agent_missions ADD COLUMN updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP;
