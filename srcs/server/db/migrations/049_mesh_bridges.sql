-- +goose Up
-- +goose StatementBegin
CREATE TABLE IF NOT EXISTS mesh_bridges (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id VARCHAR NOT NULL,
    remote_swarm_url VARCHAR NOT NULL,
    remote_organization_id VARCHAR NOT NULL,
    bridge_type VARCHAR NOT NULL DEFAULT 'P2P',
    status VARCHAR NOT NULL DEFAULT 'INACTIVE',
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX idx_mesh_bridges_org ON mesh_bridges(organization_id);
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
DROP INDEX IF EXISTS idx_mesh_bridges_org;
DROP TABLE IF EXISTS mesh_bridges;
-- +goose StatementEnd
