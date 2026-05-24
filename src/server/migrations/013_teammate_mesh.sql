-- Create schemas if they don't exist
CREATE SCHEMA IF NOT EXISTS ohc_tasks;
CREATE SCHEMA IF NOT EXISTS ohc_memory;

-- ohc_tasks.missions
CREATE TABLE IF NOT EXISTS ohc_tasks.missions (
    id UUID PRIMARY KEY,
    epic_id UUID NOT NULL,
    title TEXT NOT NULL,
    status VARCHAR(50) NOT NULL,
    assigned_agent_id UUID,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
);

-- ohc_tasks.mission_dependencies
CREATE TABLE IF NOT EXISTS ohc_tasks.mission_dependencies (
    id UUID PRIMARY KEY,
    mission_id UUID NOT NULL REFERENCES ohc_tasks.missions(id) ON DELETE CASCADE,
    depends_on_mission_id UUID NOT NULL REFERENCES ohc_tasks.missions(id) ON DELETE CASCADE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE INDEX IF NOT EXISTS mission_dependencies_mission_id_idx ON ohc_tasks.mission_dependencies(mission_id);
CREATE INDEX IF NOT EXISTS mission_dependencies_depends_on_mission_id_idx ON ohc_tasks.mission_dependencies(depends_on_mission_id);

-- Ensure pgvector is available
CREATE EXTENSION IF NOT EXISTS vector;

-- ohc_memory.autodream_vectors
CREATE TABLE IF NOT EXISTS ohc_memory.autodream_vectors (
    id UUID PRIMARY KEY,
    mission_id UUID NOT NULL REFERENCES ohc_tasks.missions(id) ON DELETE CASCADE,
    embedding vector(1536) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE INDEX IF NOT EXISTS autodream_vectors_mission_id_idx ON ohc_memory.autodream_vectors(mission_id);
