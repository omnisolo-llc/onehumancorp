-- 069_modular_plugins.sql
-- Inject missions into the agent_missions table for backend_dev (Plugin Mesh) and ui_dev (Design Tokens).

INSERT INTO agent_missions (id, status, payload, organization_id, created_at, updated_at)
VALUES (
    'backend-dev-plugin-mesh-001',
    'PENDING',
    '{"type": "backend_dev", "description": "Implement Plugin Mesh as described in docs/technical/features/modular-plugins/design-doc.md"}',
    'system',
    CURRENT_TIMESTAMP,
    CURRENT_TIMESTAMP
) ON CONFLICT(id) DO NOTHING;

INSERT INTO agent_missions (id, status, payload, organization_id, created_at, updated_at)
VALUES (
    'ui-dev-design-tokens-001',
    'PENDING',
    '{"type": "ui_dev", "description": "Implement Design Tokens as described in docs/technical/features/modular-plugins/design-doc.md"}',
    'system',
    CURRENT_TIMESTAMP,
    CURRENT_TIMESTAMP
) ON CONFLICT(id) DO NOTHING;
