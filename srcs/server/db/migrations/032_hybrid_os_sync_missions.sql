-- 032_hybrid_os_sync_missions.sql
-- Insert the market-disrupting missions identified by the Research Agent.

INSERT INTO agent_missions (id, status, payload, created_at, updated_at)
VALUES (
    'm-local-rag-1',
    'PENDING',
    '{"role":"product_architecture","task":"Implement Standalone SQLite Vector Storage for Local RAG"}',
    CURRENT_TIMESTAMP,
    CURRENT_TIMESTAMP
),
(
    'm-autodream-sync-1',
    'PENDING',
    '{"role":"product_architecture","task":"Implement Offline-to-Cloud AutoDream Synchronization Protocol"}',
    CURRENT_TIMESTAMP,
    CURRENT_TIMESTAMP
) ON CONFLICT (id) DO NOTHING;
