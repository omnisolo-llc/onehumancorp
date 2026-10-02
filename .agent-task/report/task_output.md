outcome: blocked
issue_title: "Build Native Rust Omnichannel Chat System (Chatwoot Replacement)"
issue_description: "The issue is partially implemented but blocked from safe completion due to architectural constraints on the frontend stack.

Satisfied acceptance criteria:
- The core Rust data models and PostgreSQL schemas (with RLS) for Tenant, Inbox, Conversation, Message, and Contact are implemented and verified in `src/server/db/migrations/1009_native_omnichannel_chat.sql`, `src/server/services/chat/models.rs`, and `src/server/services/chat/service.rs`.

Unmet or unverified criteria:
- Implementation of a generic Channel Adapter trait in Rust and mock implementation for E2E testing is missing.
- Real-time WebSocket infrastructure in Rust to push updates to the frontend is missing.
- The mobile-first (375px) Flutter UI for the unified inbox feed and conversation view is missing.
- Integration of the AI Customer Assistant to generate draft replies upon receiving new messages is only partially implemented (via webhook `message_triage` job).
- 100% unit test coverage is incomplete.
- 5 Playwright E2E tests are missing.

Why no safe, well-scoped implementation follows:
The acceptance criteria explicitly demand developing a mobile-first Flutter UI. However, the current repositorys architectural guidelines explicitly deprecate Flutter, PWA, and Slint components, stating that the maintained UI is Next.js inside the Tauri shell or a Node web deployment. Proceeding with a Flutter implementation would violate these constraints, and pivoting to Next.js would contradict the issues specific acceptance criteria, requiring a new authorized plan or scope clarification."
