outcome: no_work
issue_title: "Native Rust Omnichannel Chat System"
issue_description: |
  The issue requested building the core Rust gRPC microservice and PostgreSQL database migrations for the new Omnichannel Inbox, replacing Chatwoot.
  However, inspecting the codebase reveals that the Chatwoot removal (Project 1) has been completed and the canonical conversation domain (Project 3) is already fully implemented natively.
  - The `omni_inbox_messages` table and other related entities (Contacts, Inboxes) exist with strict `tenant_id` RLS policies (e.g., `src/server/migrations/1001_create_omni_inbox_messages_and_quotes_fix.sql`, `src/server/migrations/146_omni_inbox_rls.sql`).
  - Native integrations for channels like WhatsApp, Instagram, SMS, and Email are present and actively handle messages (e.g., `src/server/domain/inbox.rs`).
  - Gated checks such as `deploy/tests/no_chatwoot_residue_test.sh` enforce the removal of Chatwoot.
  - The design aligns with the `docs/superpowers/specs/2026-07-13-native-omnichannel-chat-design.md` specifications.
  No new feature implementation is needed as the described gap is already closed and tested.
  Loaded Superpowers skills: `using-superpowers`. Revision: `8ca22dba9a94f28898bbce59f2537ff4d87c747d`.
issue_priority: "P0"
issue_category: ""
issue_type: ""
issue_label: ""
assignees: []
