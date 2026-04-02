-- 011_hybrid_rag_mission.sql
-- Seeds a high-impact mission for the Hybrid MCP RAG Protocol into the database to fulfill Evolution Triggered exit conditions.

INSERT INTO agent_missions (id, status, payload)
VALUES (
    'hybrid-rag-protocol-sync-mission-001',
    'PENDING',
    '{"title": "Hybrid MCP RAG Protocol (Offline-to-Cloud State Sync)", "priority": "P0", "scope": "Medium", "description": "Implement the Offline-to-Cloud State Sync for Swarm Memories to ensure seamless operation switching and privacy."}'
) ON CONFLICT (id) DO NOTHING;
