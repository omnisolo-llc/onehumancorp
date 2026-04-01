-- 004_hub.sql
-- Agent registry, message inbox, and meeting rooms for the orchestration Hub.

CREATE TABLE IF NOT EXISTS agents (
    id              TEXT PRIMARY KEY,
    name            TEXT NOT NULL,
    role            TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT '',
    status          TEXT NOT NULL DEFAULT 'IDLE',
    provider_type   TEXT NOT NULL DEFAULT '',
    region          TEXT NOT NULL DEFAULT '',
    registered_at   DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS agent_inbox (
    seq         INTEGER PRIMARY KEY AUTOINCREMENT,   -- ordering guarantee
    agent_id    TEXT NOT NULL,
    message_id  TEXT NOT NULL,
    from_agent  TEXT NOT NULL,
    to_agent    TEXT NOT NULL DEFAULT '',
    type        TEXT NOT NULL,
    content     TEXT NOT NULL DEFAULT '',
    meeting_id  TEXT NOT NULL DEFAULT '',
    occurred_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_inbox_agent ON agent_inbox (agent_id);

CREATE TABLE IF NOT EXISTS meeting_rooms (
    id           TEXT PRIMARY KEY,
    agenda       TEXT NOT NULL DEFAULT '',
    participants TEXT NOT NULL DEFAULT '[]'
);

CREATE TABLE IF NOT EXISTS meeting_transcripts (
    seq         INTEGER PRIMARY KEY AUTOINCREMENT,
    meeting_id  TEXT NOT NULL REFERENCES meeting_rooms(id) ON DELETE CASCADE,
    message_id  TEXT NOT NULL,
    from_agent  TEXT NOT NULL,
    to_agent    TEXT NOT NULL DEFAULT '',
    type        TEXT NOT NULL,
    content     TEXT NOT NULL DEFAULT '',
    occurred_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_transcript_meeting ON meeting_transcripts (meeting_id);
