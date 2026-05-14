CREATE TABLE IF NOT EXISTS swarm_tasks (
    mission_id UUID PRIMARY KEY,
    parent_plan_id TEXT,
    dependencies JSONB,
    title VARCHAR,
    status VARCHAR,
    assigned_agent_id VARCHAR,
    payload JSONB,
    locked_until TIMESTAMP,
    created_at TIMESTAMP
);

CREATE TABLE IF NOT EXISTS state_machine_transitions (
    id SERIAL PRIMARY KEY,
    entity_id UUID,
    entity_type VARCHAR,
    from_state VARCHAR,
    to_state VARCHAR,
    agent_id VARCHAR,
    reason TEXT,
    created_at TIMESTAMP
);

CREATE TABLE IF NOT EXISTS email_campaigns (
    id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL,
    template_name TEXT,
    subject TEXT,
    body TEXT,
    audience_type TEXT,
    emails_sent INTEGER DEFAULT 0,
    open_rate FLOAT DEFAULT 0.0,
    created_at_unix BIGINT NOT NULL
);

CREATE TABLE IF NOT EXISTS social_posts (
    id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL,
    platform TEXT NOT NULL,
    content TEXT NOT NULL,
    status TEXT DEFAULT 'PENDING',
    scheduled_for_unix BIGINT,
    created_at_unix BIGINT NOT NULL
);

CREATE TABLE IF NOT EXISTS milestone_notifications (
    id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL,
    milestone_type TEXT NOT NULL,
    message TEXT NOT NULL,
    is_read BOOLEAN DEFAULT FALSE,
    created_at_unix BIGINT NOT NULL
);

CREATE TABLE IF NOT EXISTS organizations (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    plan_tier TEXT DEFAULT 'Free',
    current_period_end BIGINT
);

CREATE TABLE IF NOT EXISTS referrals (
    id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL,
    user_id TEXT NOT NULL,
    referral_code TEXT UNIQUE NOT NULL,
    clicks INTEGER DEFAULT 0,
    conversions INTEGER DEFAULT 0,
    created_at_unix BIGINT NOT NULL
);
