-- +goose Up
-- +goose StatementBegin
INSERT INTO agent_missions (id, status, payload, organization_id, created_at, updated_at)
VALUES (
    'm-sandbox-linux-bwrap',
    'PENDING',
    '{"title": "[backend] Implement bwrap Linux Sandbox Adapter", "problem_statement": "Naive regex filtering is a security gap.", "research": "Claude Code uses bwrap.", "priority": "P0"}',
    'system',
    CURRENT_TIMESTAMP,
    CURRENT_TIMESTAMP
);

INSERT INTO agent_missions (id, status, payload, organization_id, created_at, updated_at)
VALUES (
    'm-sandbox-macos-exec',
    'PENDING',
    '{"title": "[backend] Implement sandbox-exec macOS Sandbox Adapter", "problem_statement": "macOS needs strict file access isolation.", "research": "Claude Code uses sandbox-exec.", "priority": "P1"}',
    'system',
    CURRENT_TIMESTAMP,
    CURRENT_TIMESTAMP
);

INSERT INTO agent_missions (id, status, payload, organization_id, created_at, updated_at)
VALUES (
    'm-sandbox-network-proxy',
    'PENDING',
    '{"title": "[harness] Introduce Seccomp and Network Namespace proxying", "problem_statement": "Sandbox needs network namespace isolation.", "research": "Claude Code uses an outer bwrap with socat.", "priority": "P0"}',
    'system',
    CURRENT_TIMESTAMP,
    CURRENT_TIMESTAMP
);
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
DELETE FROM agent_missions WHERE id IN ('m-sandbox-linux-bwrap', 'm-sandbox-macos-exec', 'm-sandbox-network-proxy');
-- +goose StatementEnd
