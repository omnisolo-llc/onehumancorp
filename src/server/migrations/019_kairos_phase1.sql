ALTER TABLE state_machine_transitions ADD COLUMN IF NOT EXISTS parent_plan_id TEXT;
