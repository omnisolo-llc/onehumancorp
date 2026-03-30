CREATE TABLE IF NOT EXISTS swarm_memory (
    key TEXT PRIMARY KEY,
    value TEXT,
    updated_at DATETIME
);
CREATE TABLE IF NOT EXISTS agent_status (
    agent_id TEXT PRIMARY KEY,
    role TEXT,
    status TEXT,
    last_heartbeat DATETIME
);
CREATE TABLE IF NOT EXISTS agent_missions (
    id TEXT PRIMARY KEY,
    role TEXT,
    task TEXT,
    status TEXT,
    assigned_to TEXT,
    created_at DATETIME,
    updated_at DATETIME
);
CREATE TABLE IF NOT EXISTS capability_plugins (
    plugin_id TEXT PRIMARY KEY,
    name TEXT,
    version TEXT,
    manifest_url TEXT,
    status TEXT,
    registered_at DATETIME
);
CREATE TABLE IF NOT EXISTS swarm_memory_embeddings (
    memory_id TEXT PRIMARY KEY,
    context TEXT,
    vector_embedding BLOB,
    source_plugin TEXT,
    created_at DATETIME
);

INSERT INTO agent_status (agent_id, role, status, last_heartbeat)
VALUES ('product_researcher_1', 'product_researcher', 'ACTIVE', CURRENT_TIMESTAMP)
ON CONFLICT(agent_id) DO UPDATE SET role=excluded.role, status=excluded.status, last_heartbeat=CURRENT_TIMESTAMP;

INSERT INTO swarm_memory (key, value, updated_at)
VALUES ('market_insight_ohc_manifest', 'Project Grounding via OHC-MANIFEST.md & Multi-Channel Sync identified as core unfair advantage.', CURRENT_TIMESTAMP)
ON CONFLICT(key) DO UPDATE SET value=excluded.value, updated_at=CURRENT_TIMESTAMP;

INSERT INTO agent_missions (id, role, task, status, assigned_to, created_at, updated_at)
VALUES ('mission_arch_001', 'product_architecture', '{"id": "mission_arch_001", "fromAgent": "product_researcher_1", "toAgent": "product_architecture", "type": "task", "content": "Implement an agent that watches OHC-MANIFEST.md locally and continuously pushes its state to the swarm_memory DB to outpace Claude Code and OpenCode grounding patterns. Extend Hub router to support Multi-Channel gateway.", "occurredAt": "2026-03-30 00:00:00"}', 'PENDING', NULL, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
ON CONFLICT(id) DO UPDATE SET task=excluded.task, updated_at=CURRENT_TIMESTAMP;
