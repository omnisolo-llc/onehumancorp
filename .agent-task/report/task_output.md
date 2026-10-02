outcome: no_work
issue_title: Architect Native Rust Omnichannel Chat
issue_description: |
  **Evidence:**
  Replacing external services with a native implementation requires explicit authorization, evidence, and an expansion gate, which are currently missing according to RESEARCH.md. The current codebase already has a partial implementation (e.g. `src/server/db/migrations/1009_native_omnichannel_chat.sql` and `src/server/services/chat`), but finishing the architecture based on an older mission prompt goes against the updated usage audit constraints that suspend the native migration for unvalidated tools.
