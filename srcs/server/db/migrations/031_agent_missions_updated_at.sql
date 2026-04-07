ALTER TABLE agent_missions ADD COLUMN updated_at TIMESTAMPTZ;
UPDATE agent_missions SET updated_at = CURRENT_TIMESTAMP WHERE updated_at IS NULL;
