-- 040_kairos_mission.sql
-- Insert a mission from Go migration 032

INSERT INTO agent_missions (id, status, payload, tenant_id, created_at, updated_at)
VALUES (
    'kairos-orchestration-001',
    'pending',
    '{"type": "kairos_orchestration", "description": "Implement the shared task list, teammate mesh, state machine, and autoDream pipelines."}',
    'system',
    CURRENT_TIMESTAMP,
    CURRENT_TIMESTAMP
);
