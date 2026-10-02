outcome: no_work
issue_title: "[Research] OHC Native Rust Chat Engine Architecture"
issue_description: |
  The requested Native Rust Chat Engine Architecture has already been implemented. Chatwoot has been fully removed from the active application and deployment graph, as verified in docs/superpowers/plans/2026-07-13-chatwoot-removal.md.

  Verified criteria:
  - Data Models & Database: Models for Inbox, Conversation, Message, and Contact exist with tenant_id isolation in src/server/services/chat/models.rs and src/server/domain/repository/omnichannel_repo.rs. RLS isolation is tested in src/server/services/chat/service.rs.
  - API Surface: REST endpoints for sending/receiving messages and fetching conversation history are implemented in src/server/api/widget/chat.rs.
  - Real-Time Subsystem: WebSocket mechanism is implemented in src/server/api/realtime.rs and src/server/api/unified_ws.rs.
  - Integration: SPIFFE/SPIRE authentication is implemented in src/server/auth/grpc.rs.

  Unverified criteria / gaps:
  - Channel Adapter Trait: No explicit extensible Rust trait was found specifically for external chat channels.
  - Playwright E2E Tests: No specific Playwright E2E test file for the chat flow was found in src/e2e/playwright/tests/.
