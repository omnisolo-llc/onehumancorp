issue_title: "Implement Native Rust Omnichannel Chat & AI Triage Workflows"
issue_description: |
  # Blocked: No-Work Finding

  The requested feature, "Implement Native Rust Omnichannel Chat & AI Triage Workflows", has already been fully implemented natively within the Rust backend and the Next.js frontend, following the architecture defined in the native omnichannel chat design spec (`docs/superpowers/specs/2026-07-13-native-omnichannel-chat-design.md`).

  ## Verification Evidence
  - **Data Models**: The unified omnichannel database schema is present (`unified_threads`, `unified_messages`, `unified_triage_actions`) via `src/server/db/migrations/150_unified_inbox_triage.sql` and the associated models in `src/server/services/inbox/service.rs`.
  - **Webhook & Gateway**: The API endpoint for ingesting omnichannel webhooks is fully implemented in `src/server/api/unified_inbox_webhook.rs`. It handles tenant isolation, identity resolution, agent message triage (drafting responses), and job queueing. It directly inserts `Draft Reply` actions into `unified_triage_actions`.
  - **AI Agent Triage Integration**: The message triage workflow is actively processed by `src/server/workers/message_triage_worker.rs`, which calls the LLM, crafts context summaries, and inserts triage items and actions into `triage_items` and `triage_proposed_actions`.
  - **Frontend Triage UI**: The "Triage Feed" view (a mobile-responsive 375px-first card layout) has been successfully implemented in `src/ui/next/src/app/triage/page.tsx`. It correctly reads from the native unified schemas via `/api/v1/ui/triage` and handles "Review Draft", "Approve & Send", and edits for `unified_triage_actions`.

  There is no new implementation work or architecture migration left to satisfy the core requirements of this issue without overstepping bounds or modifying working production functionality.
outcome: no_work
