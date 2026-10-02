outcome: no_work
issue_title: Implement Custom Rust Omnichannel Chat System to Replace Chatwoot
issue_description: |
  The requested omnichannel chat system and Chatwoot replacement is already largely implemented, but full verification is incomplete due to missing end-to-end evidence.

  **Verified Criteria:**
  1. Core data models with row-level tenant isolation are implemented. Migration files such as `src/server/db/migrations/1009_native_omnichannel_chat.sql` explicitly create tables like `chat_inboxes` and `chat_channels` with RLS.
  2. The Rust microservices layer and REST/gRPC domain logic are present in `src/server/services/chat/` and `src/server/domain/chat/`.
  3. API handlers for the unified inbox (e.g., `src/server/api/inbox_api.rs`, `src/server/api/chat.rs`) are implemented and integrated.
  4. The legacy Chatwoot integration has already been scrubbed as `grep -rn "chatwoot" src/` returns zero matches across the active source tree.

  **Unverified Criteria (Gaps):**
  - Web Widget integration (embeddable JS script and WebSocket real-time delivery) could not be definitively verified as fully functioning.
  - End-to-end (E2E) Playwright tests verifying the UI flow (creating an inbox, starting a chat) are missing or were not executed.
  - 100% unit test coverage for the Rust microservices could not be explicitly verified in full.

  No further feature work is scheduled as the core backend components exist.
