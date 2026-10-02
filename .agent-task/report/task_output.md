outcome: blocked
issue_title: "Implement Native Omnichannel Unified Inbox & Agentic Customer Triage"
issue_description: |
  **Role:** Principal Software Engineer & Distributed Systems Architect (L7)

  **Summary:**
  The requested feature (native Rust omnichannel chat engine) is already
  fully implemented in the codebase. The data models and DB migrations
  exist in `src/server/services/chat/models.rs` and
  `src/server/db/migrations/1009_native_omnichannel_chat.sql`. The service
  layer is implemented in `src/server/services/chat/service.rs` with tests
  verifying the exact requirements (RLS isolation, multi-tenancy).
  Additionally, there is an integration layer in
  `src/server/integrations/omnichannel` and `src/server/integrations/whatsapp_cloud`.

  As instructed by the prompt ("Verify current code and tests. If the issue is already complete or requires unavailable authorization/evidence, return an explicit no_work or blocked outcome. Do not invent follow-up features or dummy changes"), I am returning an explicit `blocked` (no_work) outcome report and making no unnecessary changes to the existing functional system.

  **design rationale**: N/A for no-work finding
  **test report**: N/A for no-work finding
