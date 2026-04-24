ALTER TABLE agent_session_data ADD COLUMN capabilities JSONB DEFAULT '[]'::jsonb;
