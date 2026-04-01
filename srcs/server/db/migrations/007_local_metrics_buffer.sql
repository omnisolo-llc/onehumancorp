-- 007_local_metrics_buffer.sql
-- Swarm Intelligence Protocol local metrics buffer table

CREATE TABLE IF NOT EXISTS local_metrics_buffer (
    id          SERIAL PRIMARY KEY,
    metric_type TEXT NOT NULL,
    payload     TEXT NOT NULL,
    created_at  TIMESTAMPTZ DEFAULT NOW()
);
