CREATE TABLE IF NOT EXISTS mesh_bridges (
    id TEXT PRIMARY KEY,
    organization_id VARCHAR NOT NULL, -- The local org owning this bridge
    remote_swarm_url VARCHAR NOT NULL,
    remote_organization_id VARCHAR NOT NULL,
    bridge_type VARCHAR NOT NULL DEFAULT 'P2P', -- P2P, RELAY, HIERARCHICAL
    status VARCHAR NOT NULL DEFAULT 'INACTIVE',
    metadata TEXT DEFAULT '{}', -- Stores allowed topics, rate limits, etc.
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_mesh_bridges_org ON mesh_bridges(organization_id);
