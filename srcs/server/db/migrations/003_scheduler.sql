-- 003_scheduler.sql
-- Scheduled tasks with distributed locking support.

CREATE TABLE IF NOT EXISTS scheduled_tasks (
    id              TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL,
    agent_id        TEXT NOT NULL,
    name            TEXT NOT NULL,
    schedule_type   TEXT NOT NULL,         -- 'once', 'interval', 'cron'
    schedule_at     TIMESTAMPTZ,           -- for 'once'
    interval_s      BIGINT DEFAULT 0,      -- for 'interval'
    expression      TEXT DEFAULT '',        -- for 'cron'
    status          TEXT NOT NULL DEFAULT 'pending',
    payload         JSONB DEFAULT '{}',
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_run_at     TIMESTAMPTZ,
    next_run_at     TIMESTAMPTZ
);

CREATE INDEX IF NOT EXISTS idx_sched_tasks_org ON scheduled_tasks (organization_id);
CREATE INDEX IF NOT EXISTS idx_sched_tasks_due ON scheduled_tasks (next_run_at)
    WHERE status = 'pending';
