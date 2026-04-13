-- 045_oracle_research_missions.sql
-- Seed the validation missions to product_architecture via the DB (Phase 3).

INSERT INTO agent_missions (id, status, payload, created_at)
VALUES (
    'm-phase1-discovery',
    'PENDING',
    '{"role":"product_architecture","task":"Review Phase 1: Hybrid Cloud vs Local Competitor Audit Tool"}',
    CURRENT_TIMESTAMP
);

INSERT INTO agent_missions (id, status, payload, created_at)
VALUES (
    'm-phase2-synthesis',
    'PENDING',
    '{"role":"product_architecture","task":"Review Phase 2: Local-Private RAG with Cloud Escalation"}',
    CURRENT_TIMESTAMP
);

INSERT INTO agent_missions (id, status, payload, created_at)
VALUES (
    'm-phase4-finalize',
    'PENDING',
    '{"role":"product_architecture","task":"Review Phase 4: Hybrid Evolution Artifact Vector Ingestion"}',
    CURRENT_TIMESTAMP
);
