outcome: no_work
issue_title: "[Architect] Design Native Rust Chatwoot Replacement (Omnichannel unified Inbox)"
issue_description: |
  The implementation of the Native Rust Chatwoot Replacement is already complete in the codebase.
  - The `src/server/api/omni_inbox_webhook.rs` file explicitly documents `// Insert into chat_messages for Native Omnichannel (Chatwoot Replacement)` and natively handles webhooks without external Chatwoot APIs.
  - Multitenant DB schema with RLS is present in `src/server/db/migrations/233_chat_omnichannel.sql`.
  - E2E webhook ingestion testing is handled in `src/e2e/omni_inbox_webhook.mock-contract.ts`.
  Because all core requirements outlined in issue #35713 are actively present in the codebase, no further architectural design or implementation changes are required.
issue_priority: P0
issue_category: "Backend"
issue_type: "Feature"
issue_label: "ohc:lane:revenue"
assignees: []
