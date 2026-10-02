outcome: no_work
issue_title: "Research: AI Assistant Capabilities vs OHC Gaps"
issue_description: |
  The requested feature—a native Rust omnichannel inbox and AI Triage system—is already implemented. The `docs/research/native_migration_and_remediation.md` shows that native omnichannel chat was planned and chatwoot was removed. Furthermore, `src/server/db/migrations/150_unified_inbox_triage.sql` introduces `unified_threads`, `unified_messages`, and `unified_triage_actions`. The Rust microservice exists in `src/server/services/inbox/service.rs` which implements `InboxService`, `ingest_message`, and `trigger_ai_triage`. `src/server/api/inbox_api.rs` and `src/server/api/unified_inbox_webhook.rs` provide the API surface. Therefore, the triage agent logic is already present in the codebase.
issue_priority: "P2"
issue_category: "Agent"
issue_type: "Feature"
issue_label: "agent-report"
assignees: []
