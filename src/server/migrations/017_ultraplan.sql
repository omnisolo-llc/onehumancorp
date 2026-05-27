CREATE TABLE IF NOT EXISTS swarm_ultra_plans (
    id UUID PRIMARY KEY,
    mission_id TEXT NOT NULL,
    status TEXT NOT NULL,
    state_machine JSONB NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Add swarm_tasks alterations just to be compliant, although shared_tasks is already created correctly.
CREATE TABLE IF NOT EXISTS swarm_tasks (
    id TEXT PRIMARY KEY
);

ALTER TABLE swarm_tasks ADD COLUMN IF NOT EXISTS parent_plan_id UUID;
ALTER TABLE swarm_tasks ADD COLUMN IF NOT EXISTS dependencies JSONB;
