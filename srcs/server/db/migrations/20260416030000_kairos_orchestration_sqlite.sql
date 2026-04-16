-- +goose Up
-- +goose StatementBegin
CREATE TABLE IF NOT EXISTS mission_queue (
    mission_id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    title TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'QUEUED',
    assigned_agent TEXT,
    priority TEXT NOT NULL,
    payload JSON NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS autodream_vectors (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    task_id TEXT REFERENCES mission_queue(mission_id),
    content TEXT NOT NULL,
    embedding BLOB,
    metadata JSON,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS sub_agent_queue (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    parent_task_id TEXT NOT NULL,
    payload JSON,
    status TEXT NOT NULL DEFAULT 'QUEUED',
    worker_id TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
DROP TABLE IF EXISTS sub_agent_queue;
DROP TABLE IF EXISTS autodream_vectors;
DROP TABLE IF EXISTS mission_queue;
-- +goose StatementEnd
