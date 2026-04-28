-- 041_mcp_telemetry_mesh_mission.sql
-- Add mission from Go migration 032

INSERT INTO agent_missions (id, status, payload, tenant_id)
VALUES (
    'mission_mcp_telemetry_mesh_001',
    'PENDING',
    '{"title": "Implement Hybrid Swarm-Aware MCP Telemetry Mesh", "priority": "P0"}',
    'system'
);
