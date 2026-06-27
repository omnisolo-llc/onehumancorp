-- Seed the mock agent feed item for the mobile MVP CUJ
INSERT INTO agent_feed_items (
    id,
    tenant_id,
    event_source,
    context_payload,
    proposed_action,
    lifecycle_state,
    created_at,
    updated_at
) VALUES (
    'mock-customer-service-draft',
    'default',
    'customer_service',
    '{"description": "Customer Service Agent drafted response to inquiry.", "feature_type": "customer_service"}'::jsonb,
    '{"message": "Customer Service Agent drafted response to inquiry.", "action_type": "draft_response", "feature_type": "customer_service"}'::jsonb,
    'PENDING_APPROVAL',
    NOW(),
    NOW()
) ON CONFLICT (id) DO NOTHING;
