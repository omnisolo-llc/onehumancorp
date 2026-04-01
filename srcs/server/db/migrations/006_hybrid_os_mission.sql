-- 006_hybrid_os_mission.sql
-- Seed the Elastic Swarm Bursting mission based on the Hybrid Agentic OS market research.

INSERT INTO agent_missions (id, status, payload, created_at)
VALUES (
    'm-burst-1',
    'PENDING',
    '{"role":"product_architecture","task":"Implement Elastic Swarm Bursting for OHC Hybrid Architecture"}',
    CURRENT_TIMESTAMP
);
