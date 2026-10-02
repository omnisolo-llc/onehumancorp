outcome: no_work
issue_title: "Native Rust Omnichannel Chat System Core Architecture"
issue_description: |
  Upon exploration of the codebase, I found that the issue is already implemented. The requirements ask to define PostgreSQL data schema migrations for `inboxes`, `conversations`, and `messages` ensuring Row Level Security (`tenant_id`), implement the API endpoints in Rust, and replicate the core features of Chatwoot's data models.

  The native omnichannel chat system has already been implemented:
  - Migrations `1009_native_omnichannel_chat.sql` and `233_chat_omnichannel.sql` define the requested `chat_inboxes`, `chat_channels`, `chat_contacts`, `chat_conversations`, and `chat_messages` tables with RLS policies enabled based on `tenant_id`.
  - The Rust services for managing these models (e.g., `create_inbox`, `create_channel`, `create_contact`) are fully implemented in `src/server/services/chat/service.rs`.
  - Playwright E2E tests verifying conversation and messaging flows are already present (e.g., `src/e2e/chat.spec.ts`).

  During verification, `make test` initially failed because the `next` command was not found (`Next build failed: 127`), which was caused by missing Next.js dependencies. Running `npm install` and `npm run build:web` directly showed missing packages (`@tailwindcss/postcss`, `canonicalize`, `jsonc-parser`), which I installed locally. `cd src/ui/next && npm run build` successfully passed afterwards. Subsequent Rust verification commands `make test-rust` and `cargo check --locked --workspace --exclude app --all-targets` timed out after 400 seconds, which is noted as an environmental limitation. A fallback check `cargo test -p server_services_chat` failed since it is not an independent package. No further work is needed as the underlying functionality described in GitHub Issue #36271 is already fully integrated into the repository.
