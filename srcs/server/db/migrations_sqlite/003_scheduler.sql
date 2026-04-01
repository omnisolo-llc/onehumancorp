-- 003_scheduler.sql
-- Scheduled tasks with distributed locking support.

CREATE TABLE IF NOT EXISTS scheduled_tasks (
    id              TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL,
    agent_id        TEXT NOT NULL,
    name            TEXT NOT NULL,
    schedule_type   TEXT NOT NULL,         -- 'once', 'interval', 'cron'
    schedule_at     DATETIME,              -- for 'once'
    interval_s      BIGINT DEFAULT 0,      -- for 'interval'
    expression      TEXT DEFAULT '',       -- for 'cron'
    status          TEXT NOT NULL DEFAULT 'pending',
    payload         TEXT DEFAULT '{}',
    created_at      DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_run_at     DATETIME,
    next_run_at     DATETIME
);

CREATE INDEX IF NOT EXISTS idx_sched_tasks_org ON scheduled_tasks (organization_id);
CREATE INDEX IF NOT EXISTS idx_sched_tasks_due ON scheduled_tasks (next_run_at)
    WHERE status = 'pending';
