#!/bin/bash
mkdir -p ~/.openclaw
sqlite3 ~/.openclaw/ohc.db << 'SQL'
CREATE TABLE IF NOT EXISTS swarm_memory (key TEXT PRIMARY KEY, value TEXT, updated_at DATETIME);
CREATE TABLE IF NOT EXISTS agent_status (agent_id TEXT PRIMARY KEY, role TEXT, status TEXT, last_heartbeat DATETIME);
CREATE TABLE IF NOT EXISTS agent_missions (id TEXT PRIMARY KEY, role TEXT, task TEXT, status TEXT, assigned_to TEXT, created_at DATETIME, updated_at DATETIME);
CREATE TABLE IF NOT EXISTS capability_plugins (plugin_id TEXT PRIMARY KEY, name TEXT, version TEXT, manifest_url TEXT, status TEXT, registered_at DATETIME);
CREATE TABLE IF NOT EXISTS swarm_memory_embeddings (memory_id TEXT PRIMARY KEY, context TEXT, vector_embedding BLOB, source_plugin TEXT, created_at DATETIME);

INSERT OR REPLACE INTO swarm_memory (key, value, updated_at) VALUES ('sre_infra_state', 'active', datetime('now'));
INSERT OR REPLACE INTO agent_status (agent_id, role, status, last_heartbeat) VALUES ('sre-1', 'sre_infra', 'active', datetime('now'));
INSERT OR REPLACE INTO agent_missions (id, role, task, status, assigned_to, created_at, updated_at) VALUES ('m-sre-1', 'sre_infra', '{"id":"m-sre-1", "from_agent":"admin", "type":"Task", "content":"Performance Tuning"}', 'pending', 'sre-1', datetime('now'), datetime('now'));
SQL
