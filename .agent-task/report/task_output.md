issue_title: 🔨 Forge: [blocked no-work finding: Native Rust Omnichannel Chat System Architecture]
issue_description: |
  The codebase analysis reveals that the requested feature (Native Rust Omnichannel Chat System Architecture) is already implemented in the current codebase.

  Evidence:
  1. `src/server/migrations/1009_native_omnichannel_chat.sql` exists and contains the database schema for `chat_inboxes`, `chat_channels`, `chat_contacts`, `chat_conversations`, and `chat_messages` tables with strict RLS (Row Level Security) enabled (`CREATE POLICY ... USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid)`).
  2. `src/server/services/chat/models.rs` and `src/server/services/chat/service.rs` define the backend core of the Native Rust Omnichannel Chat System, handling the DB operations for inboxes, channels, contacts, conversations, and messages using `sqlx`.
  3. No occurrences of "chatwoot" were found in `src/`, suggesting the system has already fully retired Chatwoot. Historical plans like `docs/superpowers/plans/2026-07-13-chatwoot-removal.md` reflect that the Chatwoot dependency was intentionally removed and the replacement natively implemented.

  Superpowers skill provenance: Loaded `using-superpowers` from revision `8ca22dba9a94f28898bbce59f2537ff4d87c747d` (skills/using-superpowers/SKILL.md) to understand task procedures. The codebase satisfies the required changes; any new modifications would be redundant.

  Therefore, no new work is needed as this is a blocked no-work finding because the target is already achieved in the repo.
outcome: blocked
