issue_title: "🔨 Forge: [blocked no-work finding: Native Rust Omnichannel Chat System]"
issue_description: |
  The instruction requests replacing a "legacy ruby-on-rails external omnichannel dependency" with a native Rust omnichannel engine. However, the current codebase has already implemented the omnichannel backend system natively in Rust.

  Code evidence includes:
  - `src/server/services/chat/` handles the core chat microservice including `chat_inboxes`, `chat_channels`, `chat_contacts`, `chat_conversations`, and `chat_messages` tables with PostgreSQL Row-Level Security (RLS) tenant isolation.
  - `src/server/integrations/omnichannel/` and `src/server/domain/repository/omnichannel_repo.rs` provide further implementation details.
  - There is no legacy Ruby on Rails application to migrate from within the OHC platform.
  - The instruction requests building a "Flutter UI", but the repository contract strictly states: "The project's frontend architecture has explicitly migrated away from the old Slint/Flutter UI; the maintained UI is Next.js inside a Tauri shell or a Node web deployment. Do not attempt to write or execute Flutter/Dart code." The existing Next.js frontend already implements the unified inbox in `src/ui/next/src/app/inbox/page.tsx` and covers it with E2E tests in `src/ui/next/src/e2e/omni_inbox.spec.ts`.

  Therefore, the core requirements of this issue (native Rust implementation, PostgreSQL RLS schema, removing the external ruby-on-rails dependency, and Flutter UI constraints) are either already completed or explicitly forbidden by current architectural guardrails. This results in a no-work finding.

  Resolves #35934
issue_priority: ""
issue_category: ""
issue_type: ""
issue_label: ""
assignees: []
outcome: "no_work"
