ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS ultraplan_phase TEXT;
ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS deliberation_log JSONB DEFAULT '[]';
ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS depth INTEGER;

ALTER TABLE shared_tasks_v4 ADD COLUMN IF NOT EXISTS ultraplan_phase TEXT;
ALTER TABLE shared_tasks_v4 ADD COLUMN IF NOT EXISTS deliberation_log JSONB DEFAULT '[]';
ALTER TABLE shared_tasks_v4 ADD COLUMN IF NOT EXISTS depth INTEGER;
