-- Add the high-impact mission to agent_missions
INSERT INTO agent_missions (id, status, payload, organization_id)
VALUES (
    'mission_mcp_telemetry_mesh_001',
    'PENDING',
    '{"title": "Implement Hybrid Swarm-Aware MCP Telemetry Mesh", "priority": "P0"}',
    'system'
);
