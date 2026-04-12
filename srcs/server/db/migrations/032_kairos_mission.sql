-- +goose Up
INSERT INTO agent_missions (id, status, payload, organization_id, created_at, updated_at)
VALUES (
    'kairos-orchestration-001',
    'pending',
    '{"type": "kairos_orchestration", "description": "Implement the shared task list, teammate mesh, state machine, and autoDream pipelines."}',
    'system',
    CURRENT_TIMESTAMP,
    CURRENT_TIMESTAMP
);

-- +goose Down
DELETE FROM agent_missions WHERE id = 'kairos-orchestration-001';
