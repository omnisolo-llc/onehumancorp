outcome: no_work
issue_title: "Implement Native Rust Omnichannel Inbox to Replace Chatwoot Dependency"
issue_description: |
  Verified `no_work` finding. The requested Native Rust Omnichannel Inbox is already implemented and the Chatwoot dependency has been fully removed from the codebase.

  Verification Evidence:
  - `src/server/services/omnichannel_service.rs` explicitly implements `OmniChannelService::ingest_signal` to fetch tenant context and generate drafted replies natively via AI.
  - `src/server/domain/repository/omnichannel_repo.rs` confirms the native data models `CustomerProfile`, `WorkItem`, `Inbox`, and `Contact` exist, mapping to database entities.
  - `docs/reports/production_agent_optimization_report.md` explicitly documents under `CHAT-00` that Chatwoot has been "Removed from the active application and deployment graph."
  - There are no remaining references to Chatwoot in the `src/` directory.
issue_priority: P0
issue_category: operations
issue_type: feature
issue_label: []
assignees: []
