-- +goose Up
INSERT INTO agent_missions (id, status, payload, organization_id, created_at, updated_at)
VALUES (
    'kairos-phase1-shared-tasks',
    'PENDING',
    '{"role":"kairos","task":"Implement Shared Task List Database designs and sequence diagrams"}',
    'system',
    CURRENT_TIMESTAMP,
    CURRENT_TIMESTAMP
);

-- +goose Down
DELETE FROM agent_missions WHERE id = 'kairos-phase1-shared-tasks';
