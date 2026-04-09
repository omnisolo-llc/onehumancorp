-- 032_hybrid_mcp_rag_sync_mission.sql
-- Seed the Hybrid MCP RAG State Sync protocol mission

INSERT INTO agent_missions (id, status, payload, created_at)
VALUES (
    'm-hybrid-mcp-rag-sync',
    'PENDING',
    '{"role":"product_architecture","task":"Implement Hybrid Context Synchronizer Daemon for RAG offline-to-cloud sync"}',
    CURRENT_TIMESTAMP
);
