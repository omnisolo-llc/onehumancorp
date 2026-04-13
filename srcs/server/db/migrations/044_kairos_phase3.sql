-- +goose Up
INSERT INTO agent_missions (id, status, payload, organization_id, created_at, updated_at)
VALUES (
    'kairos-phase3-autodream',
    'PENDING',
    '{"role":"kairos","task":"Implement AutoDream long-term memory data pipelines"}',
    'system',
    CURRENT_TIMESTAMP,
    CURRENT_TIMESTAMP
);

-- +goose Down
DELETE FROM agent_missions WHERE id = 'kairos-phase3-autodream';
