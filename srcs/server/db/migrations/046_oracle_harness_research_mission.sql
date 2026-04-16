-- +goose Up
-- +goose StatementBegin
INSERT INTO agent_missions (id, status, payload, organization_id, created_at, updated_at)
VALUES (
    'oracle-harness-research-proxy',
    'PENDING',
    '{"role":"implementer","task":{"role":"user","content":"Implement `socat` proxy bridging for `bwrap` in Local Desktop Mode. Ensure strict network isolation."}}',
    'system',
    CURRENT_TIMESTAMP,
    CURRENT_TIMESTAMP
);
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
DELETE FROM agent_missions WHERE id = 'oracle-harness-research-proxy';
-- +goose StatementEnd
