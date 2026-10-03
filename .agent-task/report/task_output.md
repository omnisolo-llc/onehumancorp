issue_title: "Native Rust Omnichannel Chat System & Universal Inbox"
issue_description: |
  The requested Native Rust Omnichannel Chat System & Universal Inbox implementation (Issue #35934) is blocked and requires no work.

  1. The issue requests implementing Flutter mobile UI components, but according to the memory directives: "The project's frontend architecture has explicitly migrated away from the old Slint/Flutter UI; the maintained UI is Next.js inside a Tauri shell or a Node web deployment. Do not attempt to write or execute Flutter/Dart code."
  2. The core Rust Omnichannel Chat Service backend is already implemented in `src/server/integrations/omnichannel` and `src/server/services/omnichannel_service.rs`, utilizing PostgreSQL with RLS keyed by `tenant_id` for Contact, Conversations, and Messages (as explicitly verified in `src/server/db/migrations/233_chat_omnichannel.sql` and `src/server/migrations/233_chat_omnichannel.sql`).
  3. The requested E2E verification is already covered by `src/ui/next/src/e2e/omni_inbox.spec.ts`, which asserts that an owner sees incoming messages and known customers in the UI.
  4. The migration away from the "legacy ruby-on-rails external omnichannel dependency" has already taken place (the string is completely absent from the codebase, and Rust implementations exist in its place).

  Therefore, this is a verified no-work outcome as the system's current architecture strictly forbids Flutter and the backend requirements are already met natively by the Rust codebase.
issue_priority: "P0"
issue_category: ""
issue_type: ""
issue_label: ""
assignees: []
outcome: "no_work"
