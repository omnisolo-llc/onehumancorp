-- +goose Up
INSERT INTO agent_missions (id, status, payload, organization_id, created_at, updated_at)
VALUES (
    'kairos-phase2-teammate-mesh',
    'PENDING',
    '{"role":"kairos","task":"Implement Realtime Teammate Mesh APIs"}',
    'system',
    CURRENT_TIMESTAMP,
    CURRENT_TIMESTAMP
);

-- +goose Down
DELETE FROM agent_missions WHERE id = 'kairos-phase2-teammate-mesh';
