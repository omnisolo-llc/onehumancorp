-- +goose Up
-- Migration 211: Add missing RLS to tables

-- Enable RLS
ALTER TABLE IF EXISTS agent_jobs ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS agent_session_summaries ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS appointments ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS auto_reply_policies ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS booking_slots ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS cart_items ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS carts ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS context_snippets ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS crdt_deltas ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS customer_loyalty_accounts ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS delivery_tasks ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS embedding_cache ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS epics ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS escalations ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS incidents ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS interaction_event_jobs ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS interaction_events ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS inventory_transactions ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS job_templates ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS ledger_reserves ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS locations ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS loyalty_programs ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS loyalty_rewards ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS loyalty_transactions ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS mutation_queue ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS newsletter_drafts ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS payment_intents ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS price_history ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS pricing_rules ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS project_tasks ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS projects ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS quote_requests ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS role_assignments ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS route_stops ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS service_requests ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS shared_task_dependencies ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS shared_tasks_decomposition ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS shared_tasks_v4 ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS shift_summaries ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS shifts ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS social_post_proposals ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS staff_availability ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS staff_tasks ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS sub_agent_queue ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS swarm_tasks ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS swarm_truth_embeddings ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS sync_conflict_queue ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS sync_events ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS task_dependencies ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS task_envelopes ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS telemetry_buffer ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS test_sync_entities ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS user_configs ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS work_intents ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS estimate_line_items ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS estimates ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS agent_session_data ENABLE ROW LEVEL SECURITY;

DO $$
DECLARE
    t_name text;
    pol_name text;
BEGIN
    FOR t_name IN
        SELECT unnest(ARRAY[
            'agent_jobs',
            'agent_session_summaries',
            'appointments',
            'auto_reply_policies',
            'booking_slots',
            'cart_items',
            'carts',
            'context_snippets',
            'crdt_deltas',
            'customer_loyalty_accounts',
            'delivery_tasks',
            'embedding_cache',
            'epics',
            'escalations',
            'incidents',
            'interaction_event_jobs',
            'interaction_events',
            'inventory_transactions',
            'job_templates',
            'ledger_reserves',
            'locations',
            'loyalty_programs',
            'loyalty_rewards',
            'loyalty_transactions',
            'mutation_queue',
            'newsletter_drafts',
            'payment_intents',
            'price_history',
            'pricing_rules',
            'project_tasks',
            'projects',
            'quote_requests',
            'role_assignments',
            'route_stops',
            'service_requests',
            'shared_task_dependencies',
            'shared_tasks_v4',
            'shift_summaries',
            'shifts',
            'social_post_proposals',
            'staff_availability',
            'staff_tasks',
            'sub_agent_queue',
            'swarm_tasks',
            'swarm_truth_embeddings',
            'sync_conflict_queue',
            'sync_events',
            'task_dependencies',
            'task_envelopes',
            'telemetry_buffer',
            'test_sync_entities',
            'user_configs',
            'work_intents',
            'estimate_line_items',
            'estimates',
            'agent_session_data'
        ])
    LOOP
        IF to_regclass(t_name) IS NOT NULL THEN
            pol_name := format('tenant_isolation_%s', t_name);
            IF NOT EXISTS (
                SELECT 1
                FROM pg_policies
                WHERE schemaname = current_schema()
                    AND tablename = t_name
                    AND policyname = pol_name
            ) THEN
                EXECUTE format(
                    'CREATE POLICY %I ON %I USING (tenant_id::text = current_setting(''app.current_tenant'', true)) WITH CHECK (tenant_id::text = current_setting(''app.current_tenant'', true))',
                    pol_name,
                    t_name
                );
            END IF;
        END IF;
    END LOOP;
END
$$;

DO $$
BEGIN
    IF to_regclass('shared_tasks_decomposition') IS NOT NULL THEN
        IF NOT EXISTS (
            SELECT 1
            FROM pg_policies
            WHERE schemaname = current_schema()
                AND tablename = 'shared_tasks_decomposition'
                AND policyname = 'tenant_isolation_shared_tasks_decomposition'
        ) THEN
            EXECUTE format(
                'CREATE POLICY tenant_isolation_shared_tasks_decomposition ON shared_tasks_decomposition USING (organization_id::text = current_setting(''app.current_tenant'', true)) WITH CHECK (organization_id::text = current_setting(''app.current_tenant'', true))'
            );
        END IF;
    END IF;
END
$$;

-- +goose Down
-- Revert RLS
DO $$
DECLARE
    t_name text;
    pol_name text;
BEGIN
    FOR t_name IN
        SELECT unnest(ARRAY[
            'agent_jobs',
            'agent_session_summaries',
            'appointments',
            'auto_reply_policies',
            'booking_slots',
            'cart_items',
            'carts',
            'context_snippets',
            'crdt_deltas',
            'customer_loyalty_accounts',
            'delivery_tasks',
            'embedding_cache',
            'epics',
            'escalations',
            'incidents',
            'interaction_event_jobs',
            'interaction_events',
            'inventory_transactions',
            'job_templates',
            'ledger_reserves',
            'locations',
            'loyalty_programs',
            'loyalty_rewards',
            'loyalty_transactions',
            'mutation_queue',
            'newsletter_drafts',
            'payment_intents',
            'price_history',
            'pricing_rules',
            'project_tasks',
            'projects',
            'quote_requests',
            'role_assignments',
            'route_stops',
            'service_requests',
            'shared_task_dependencies',
            'shared_tasks_v4',
            'shift_summaries',
            'shifts',
            'social_post_proposals',
            'staff_availability',
            'staff_tasks',
            'sub_agent_queue',
            'swarm_tasks',
            'swarm_truth_embeddings',
            'sync_conflict_queue',
            'sync_events',
            'task_dependencies',
            'task_envelopes',
            'telemetry_buffer',
            'test_sync_entities',
            'user_configs',
            'work_intents',
            'estimate_line_items',
            'estimates',
            'agent_session_data'
        ])
    LOOP
        IF to_regclass(t_name) IS NOT NULL THEN
            pol_name := format('tenant_isolation_%s', t_name);
            EXECUTE format('DROP POLICY IF EXISTS %I ON %I', pol_name, t_name);
        END IF;
    END LOOP;
END
$$;

DO $$
BEGIN
    IF to_regclass('shared_tasks_decomposition') IS NOT NULL THEN
        DROP POLICY IF EXISTS tenant_isolation_shared_tasks_decomposition ON shared_tasks_decomposition;
    END IF;
END
$$;
