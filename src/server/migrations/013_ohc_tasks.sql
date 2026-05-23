CREATE EXTENSION IF NOT EXISTS vector;

CREATE SCHEMA IF NOT EXISTS ohc_tasks;
CREATE SCHEMA IF NOT EXISTS ohc_memory;

CREATE TABLE IF NOT EXISTS ohc_tasks.missions (
    id UUID PRIMARY KEY,
    epic_id UUID NOT NULL,
    title VARCHAR NOT NULL,
    status VARCHAR NOT NULL,
    assigned_agent_id VARCHAR
);

CREATE TABLE IF NOT EXISTS ohc_tasks.mission_dependencies (
    id UUID PRIMARY KEY,
    mission_id UUID NOT NULL,
    depends_on_mission_id UUID NOT NULL,
    FOREIGN KEY (mission_id) REFERENCES ohc_tasks.missions(id),
    FOREIGN KEY (depends_on_mission_id) REFERENCES ohc_tasks.missions(id)
);

CREATE TABLE IF NOT EXISTS ohc_memory.autodream_vectors (
    id UUID PRIMARY KEY,
    mission_id UUID NOT NULL,
    vector_data vector(1536) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
