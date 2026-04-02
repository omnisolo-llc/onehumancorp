CREATE TABLE IF NOT EXISTS swarm_ultra_plans (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    mission_id TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'DELIBERATING' CHECK (status IN ('DELIBERATING', 'EXECUTING', 'COMPLETED', 'FAILED')),
    state_machine JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

ALTER TABLE swarm_tasks ADD COLUMN parent_plan_id UUID;
ALTER TABLE swarm_tasks ADD COLUMN dependencies JSONB DEFAULT '[]';

CREATE TABLE IF NOT EXISTS swarm_dream_epochs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    status TEXT NOT NULL DEFAULT 'STARTED' CHECK (status IN ('STARTED', 'COMPLETED', 'FAILED')),
    cluster_results JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    completed_at TIMESTAMPTZ
);
