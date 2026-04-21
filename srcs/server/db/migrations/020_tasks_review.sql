-- Wait, let me just add the index and check constraint in SQLite compliant way.

-- Let's just create indexes based on status since locked_until might not be present or consistent
CREATE INDEX IF NOT EXISTS idx_shared_tasks_status ON shared_tasks(status);

-- Since swarm_tasks is no longer guaranteed to have 'locked_until' in all migration paths for SQLite, we will rely on shared_tasks logic.
-- CREATE INDEX IF NOT EXISTS idx_swarm_tasks_status_locked_until ON swarm_tasks(status, locked_until);

-- Drop the old constraint if it exists.
-- In SQLite this syntax will be ignored by our migrator, but we can just skip it,
-- or we can use the explicit check constraint since SQLite does not enforce CHECK if not defined in CREATE TABLE directly unless via recreate.
-- Our RunMigrations already filters out DROP CONSTRAINT for sqlite.
-- ALTER TABLE swarm_tasks DROP CONSTRAINT IF EXISTS swarm_tasks_status_check;
-- ALTER TABLE swarm_tasks ADD CONSTRAINT swarm_tasks_status_check CHECK (status IN ('PENDING', 'IN_PROGRESS', 'REVIEW', 'COMPLETED', 'FAILED'));

-- If shared_tasks needs a check, we can do it here too:
-- ALTER TABLE shared_tasks DROP CONSTRAINT IF EXISTS shared_tasks_status_check;
-- ALTER TABLE shared_tasks ADD CONSTRAINT shared_tasks_status_check CHECK (status IN ('PENDING', 'IN_PROGRESS', 'REVIEW', 'COMPLETED', 'FAILED'));
