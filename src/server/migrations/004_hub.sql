-- 005_hub.sql
-- Agent registry, message inbox, and meeting rooms for the orchestration Hub.

CREATE TABLE IF NOT EXISTS agents (
    id              TEXT PRIMARY KEY,
    name            TEXT NOT NULL,
    role            TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT '',
    status          TEXT NOT NULL DEFAULT 'IDLE',
    provider_type   TEXT NOT NULL DEFAULT '',
    region          TEXT NOT NULL DEFAULT '',
    registered_at   TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE agents ENABLE ROW LEVEL SECURITY;

CREATE TABLE IF NOT EXISTS agent_inbox (
    seq         BIGSERIAL PRIMARY KEY,   -- ordering guarantee
    agent_id    TEXT NOT NULL,
    message_id  TEXT NOT NULL,
    from_agent  TEXT NOT NULL,
    to_agent    TEXT NOT NULL DEFAULT '',
    type        TEXT NOT NULL,
    content     TEXT NOT NULL DEFAULT '',
    meeting_id  TEXT NOT NULL DEFAULT '',
    occurred_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE agent_inbox ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_agent_inbox ON agent_inbox;

CREATE INDEX idx_inbox_agent ON agent_inbox (agent_id);

CREATE TABLE IF NOT EXISTS meeting_rooms (
    id           TEXT PRIMARY KEY,
    agenda       TEXT NOT NULL DEFAULT '',
    participants TEXT[] NOT NULL DEFAULT '{}'
);

ALTER TABLE meeting_rooms ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_meeting_rooms ON meeting_rooms;

CREATE TABLE IF NOT EXISTS meeting_transcripts (
    seq         BIGSERIAL PRIMARY KEY,
    meeting_id  TEXT NOT NULL REFERENCES meeting_rooms(id) ON DELETE CASCADE,
    message_id  TEXT NOT NULL,
    from_agent  TEXT NOT NULL,
    to_agent    TEXT NOT NULL DEFAULT '',
    type        TEXT NOT NULL,
    content     TEXT NOT NULL DEFAULT '',
    occurred_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE meeting_transcripts ENABLE ROW LEVEL SECURITY;

CREATE INDEX idx_transcript_meeting ON meeting_transcripts (meeting_id);
