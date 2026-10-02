outcome: no_work
issue_title: "Native Rust Omnichannel unified inbox & chat engine (Chatwoot Replacement)"
issue_description: |
  The requested functionality is already fully implemented in the codebase:
  - Database schemas and migrations for omnichannel (inboxes, channels, contacts, conversations, messages) exist in `src/server/migrations/233_chat_omnichannel.sql`.
  - The webhook handler and identity resolution logic are implemented in `src/server/api/omnichannel_webhook.rs`.
  - The UI for reviewing and approving drafts exists in `src/ui/next/src/app/triage/page.tsx` and related files.
  - Integration with the AI agent and background jobs are present in `src/server/api/omnichannel_webhook.rs` and `src/server/api/work_triage.rs`.
  - End-to-end tests exist, e.g., `src/e2e/omni_context_subagent_routing.mock-contract.ts` and `src/e2e/omnichannel_memory_graph.mock-contract.ts`.
  No further work is required.
