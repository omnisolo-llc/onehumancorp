outcome: no_work
issue_title: "[Architecture] Native Rust Omnichannel Chat (legacy chat engine Replacement)"
issue_description: |
  The native Rust omnichannel chat system is already implemented in the codebase.
  - Database schema exists in src/server/migrations/233_chat_omnichannel.sql and src/server/db/migrations/1009_native_omnichannel_chat.sql.
  - Core entities and logic are present in src/server/services/chat/ (including service.rs and models.rs).
  - API endpoints are defined in src/server/api/widget/chat.rs.
  - Comprehensive E2E tests exist under src/e2e/playwright/.
  - Therefore, this is an explicit no-work finding as the requested feature already exists natively.
