CREATE SCHEMA IF NOT EXISTS ohc_tasks;
CREATE SCHEMA IF NOT EXISTS ohc_memory;

CREATE TABLE IF NOT EXISTS ohc_tasks.missions (
    id UUID PRIMARY KEY,
    epic_id UUID NOT NULL,
    title VARCHAR(255) NOT NULL,
    status VARCHAR(50) NOT NULL,
    assigned_agent_id UUID
);

CREATE TABLE IF NOT EXISTS ohc_tasks.mission_dependencies (
    id UUID PRIMARY KEY,
    mission_id UUID NOT NULL REFERENCES ohc_tasks.missions(id) ON DELETE CASCADE,
    depends_on_mission_id UUID NOT NULL REFERENCES ohc_tasks.missions(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS ohc_memory.autodream_vectors (
    id UUID PRIMARY KEY,
    embedding vector(1536),
    payload JSONB
);
