-- Migration to enforce RLS on all remaining tables that missed it.

DO $$
DECLARE
    t_name text;
BEGIN
    FOR t_name IN
        SELECT unnest(ARRAY[
            'api_keys',
            'assistant_connectors',
            'assistant_memory_records',
            'assistant_skills',
            'harness_actor_bindings',
            'harness_actors',
            'harness_artifacts',
            'harness_attempts',
            'harness_bindings',
            'harness_capability_snapshots',
            'harness_capsule_record_batches',
            'harness_capsules',
            'harness_command_inbox',
            'harness_command_outbox',
            'harness_compactions',
            'harness_content_parts',
            'harness_descriptors',
            'harness_error_records',
            'harness_events',
            'harness_goals',
            'harness_handoff_loss_reports',
            'harness_handoff_operations',
            'harness_inference_admissions',
            'harness_inference_final_responses',
            'harness_inference_requests',
            'harness_inference_stream_records',
            'harness_interaction_responses',
            'harness_interactions',
            'harness_leases',
            'harness_messages',
            'harness_model_bindings',
            'harness_model_capacity_leases',
            'harness_model_descriptors',
            'harness_model_runtime_capabilities',
            'harness_model_runtime_workers',
            'harness_model_runtimes',
            'harness_native_record_sets',
            'harness_native_records',
            'harness_operation_fences',
            'harness_plan_steps',
            'harness_plans',
            'harness_portable_context_checkpoints',
            'harness_process_chunks',
            'harness_processes',
            'harness_resume_checkpoints',
            'harness_runtime_configs',
            'harness_sessions',
            'harness_tasks',
            'harness_todos',
            'harness_tool_calls',
            'harness_tool_definition_snapshots',
            'harness_tool_progress',
            'harness_tool_results',
            'harness_turns',
            'harness_usage_records',
            'harness_workspace_descriptors',
            'harness_workspace_snapshots',
            'usage_idempotency_keys',
            'user_usage_logs',
            'help_articles',
            'job_locations',
            'tooltips',
            'video_tutorials',
            'walkthrough_steps'
        ])
    LOOP
        -- Create policies based on column existence
        -- Check if tenant_id exists
        IF EXISTS (
            SELECT 1 FROM information_schema.columns
            WHERE table_name = t_name AND column_name = 'tenant_id'
        ) THEN
            EXECUTE format('ALTER TABLE IF EXISTS %I ENABLE ROW LEVEL SECURITY;', t_name);
            EXECUTE format('
                DO $inner$
                BEGIN
                    IF NOT EXISTS (SELECT 1 FROM pg_policies WHERE tablename = %L AND policyname = ''tenant_isolation_'' || %L) THEN
                        CREATE POLICY %I ON %I FOR ALL USING (tenant_id::text = current_setting(''app.current_tenant'', true)) WITH CHECK (tenant_id::text = current_setting(''app.current_tenant'', true));
                    END IF;
                END
                $inner$;
            ', t_name, t_name, 'tenant_isolation_' || t_name, t_name);
        -- Check if organization_id exists
        ELSIF EXISTS (
            SELECT 1 FROM information_schema.columns
            WHERE table_name = t_name AND column_name = 'organization_id'
        ) THEN
            EXECUTE format('ALTER TABLE IF EXISTS %I ENABLE ROW LEVEL SECURITY;', t_name);
            EXECUTE format('
                DO $inner$
                BEGIN
                    IF NOT EXISTS (SELECT 1 FROM pg_policies WHERE tablename = %L AND policyname = ''tenant_isolation_'' || %L) THEN
                        CREATE POLICY %I ON %I FOR ALL USING (organization_id::text = current_setting(''app.current_tenant'', true)) WITH CHECK (organization_id::text = current_setting(''app.current_tenant'', true));
                    END IF;
                END
                $inner$;
            ', t_name, t_name, 'tenant_isolation_' || t_name, t_name);
        END IF;
    END LOOP;
END
$$;
