ALTER TABLE state_machine_transitions ADD COLUMN IF NOT EXISTS handoff_payload JSONB;
