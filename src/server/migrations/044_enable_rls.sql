-- 044_enable_rls.sql
-- Enable Row-Level Security (RLS) for multi-tenant data protection.
-- Every business is a tenant. Row-level tenant isolation in PostgreSQL using organization_id column.

-- 1. Function to get the current organization ID from session settings
CREATE OR REPLACE FUNCTION current_organization_id() RETURNS TEXT AS $$
    SELECT current_setting('ohc.current_organization_id', true);
$$ LANGUAGE sql STABLE;

-- 2. Define the RLS Policy Template
-- Users table
ALTER TABLE users ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS users_isolation_policy ON users;
CREATE POLICY users_isolation_policy ON users
    USING (organization_id = current_organization_id() OR current_organization_id() = 'system');

-- Tasks table
ALTER TABLE tasks ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tasks_isolation_policy ON tasks;
CREATE POLICY tasks_isolation_policy ON tasks
    USING (organization_id = current_organization_id() OR current_organization_id() = 'system');

-- Agent Memories
ALTER TABLE agent_memories ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS agent_memories_isolation_policy ON agent_memories;
CREATE POLICY agent_memories_isolation_policy ON agent_memories
    USING (organization_id = current_organization_id() OR current_organization_id() = 'system');

-- Agent Session Data
ALTER TABLE agent_session_data ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS agent_session_data_isolation_policy ON agent_session_data;
CREATE POLICY agent_session_data_isolation_policy ON agent_session_data
    USING (organization_id = current_organization_id() OR current_organization_id() = 'system');

-- Swarm Truth Embeddings
ALTER TABLE swarm_truth_embeddings ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS swarm_truth_embeddings_isolation_policy ON swarm_truth_embeddings;
CREATE POLICY swarm_truth_embeddings_isolation_policy ON swarm_truth_embeddings
    USING (organization_id = current_organization_id() OR current_organization_id() = 'system');

-- Agent Missions
ALTER TABLE agent_missions ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS agent_missions_isolation_policy ON agent_missions;
CREATE POLICY agent_missions_isolation_policy ON agent_missions
    USING (organization_id = current_organization_id() OR current_organization_id() = 'system');

-- Telemetry Buffer
ALTER TABLE telemetry_buffer ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS telemetry_buffer_isolation_policy ON telemetry_buffer;
CREATE POLICY telemetry_buffer_isolation_policy ON telemetry_buffer
    USING (organization_id = current_organization_id() OR current_organization_id() = 'system');

-- Shared Tasks
ALTER TABLE shared_tasks ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS shared_tasks_isolation_policy ON shared_tasks;
CREATE POLICY shared_tasks_isolation_policy ON shared_tasks
    USING (organization_id = current_organization_id() OR current_organization_id() = 'system');

-- Autodream Memories
ALTER TABLE autodream_memories ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS autodream_memories_isolation_policy ON autodream_memories;
CREATE POLICY autodream_memories_isolation_policy ON autodream_memories
    USING (organization_id = current_organization_id() OR current_organization_id() = 'system');

-- Agent Status
ALTER TABLE agent_status ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS agent_status_isolation_policy ON agent_status;
CREATE POLICY agent_status_isolation_policy ON agent_status
    USING (organization_id = current_organization_id() OR current_organization_id() = 'system');

-- Swarm Memory
ALTER TABLE swarm_memory ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS swarm_memory_isolation_policy ON swarm_memory;
CREATE POLICY swarm_memory_isolation_policy ON swarm_memory
    USING (organization_id = current_organization_id() OR current_organization_id() = 'system');

-- Capability Plugins
ALTER TABLE capability_plugins ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS capability_plugins_isolation_policy ON capability_plugins;
CREATE POLICY capability_plugins_isolation_policy ON capability_plugins
    USING (organization_id = current_organization_id() OR current_organization_id() = 'system');

-- Swarm Memory Embeddings
ALTER TABLE swarm_memory_embeddings ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS swarm_memory_embeddings_isolation_policy ON swarm_memory_embeddings;
CREATE POLICY swarm_memory_embeddings_isolation_policy ON swarm_memory_embeddings
    USING (organization_id = current_organization_id() OR current_organization_id() = 'system');

-- Meeting Rooms
ALTER TABLE meeting_rooms ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS meeting_rooms_isolation_policy ON meeting_rooms;
CREATE POLICY meeting_rooms_isolation_policy ON meeting_rooms
    USING (organization_id = current_organization_id() OR current_organization_id() = 'system');

-- Agent Inbox
ALTER TABLE agent_inbox ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS agent_inbox_isolation_policy ON agent_inbox;
CREATE POLICY agent_inbox_isolation_policy ON agent_inbox
    USING (organization_id = current_organization_id() OR current_organization_id() = 'system');

-- Agents
ALTER TABLE agents ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS agents_isolation_policy ON agents;
CREATE POLICY agents_isolation_policy ON agents
    USING (organization_id = current_organization_id() OR current_organization_id() = 'system');

-- Consolidated Memory
ALTER TABLE consolidated_memory ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS consolidated_memory_isolation_policy ON consolidated_memory;
CREATE POLICY consolidated_memory_isolation_policy ON consolidated_memory
    USING (organization_id = current_organization_id() OR current_organization_id() = 'system');

-- Meeting Transcripts
ALTER TABLE meeting_transcripts ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS meeting_transcripts_isolation_policy ON meeting_transcripts;
CREATE POLICY meeting_transcripts_isolation_policy ON meeting_transcripts
    USING (EXISTS (SELECT 1 FROM meeting_rooms WHERE meeting_rooms.id = meeting_transcripts.meeting_id) OR current_organization_id() = 'system');

-- Scheduled Tasks
ALTER TABLE scheduled_tasks ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS scheduled_tasks_isolation_policy ON scheduled_tasks;
CREATE POLICY scheduled_tasks_isolation_policy ON scheduled_tasks
    USING (organization_id = current_organization_id() OR current_organization_id() = 'system');
