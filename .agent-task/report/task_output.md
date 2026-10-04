outcome: no_work
issue_title: Agentic Tap-to-Pay Terminal Architecture for Mobile Point of Sale (mPOS)
issue_description: |
  Upon review of the current OHC codebase, the foundational components required for the "Agentic Tap-to-Pay Terminal Architecture for Mobile Point of Sale (mPOS)" (Issue #32045) have already been successfully implemented and tested.

  **Verification Evidence:**
  1. The UI/frontend structure is in place at `src/ui/next/src/app/pos/mpos/page.tsx` with offline-sync support for mPOS scenarios and Stripe terminal handling via `StripeTerminalClient`. Playwright E2E tests for offline-tolerant tap-to-pay are verified in `src/ui/next/src/e2e/playwright/tap-to-pay.mock-contract.ts`.
  2. The `TerminalSession` data schema has been correctly defined in `src/server/domain/repository/models.rs` and the required PostgreSQL table `pos_terminal_sessions` with tenant-isolation logic is properly established in the database migration `src/server/db/migrations/027_pos_terminal_sessions.sql`.
  3. The requested Stripe terminal API integration endpoints—including `create_payment_intent_handler` and `capture_payment_intent_handler` with dynamic pricing and agent integrations—are fully integrated into the backend router at `src/server/api/terminal_api.rs`.
  4. The required idempotent backend logic explicitly handles optimistic locking using Redis through `InventoryService.reserve_inventory`, commits stock via `InventoryService.commit_inventory` after payment capture, and systematically triggers AI agent actions via inserting into `agent_action_requests` for both Sales and Operations and into `agent_feed_items` for receipt generation (`src/server/api/terminal_api.rs:1149`).
  5. The backend testing (`src/server/api/terminal_api_test.rs`) covers the terminal API behavior under various scenarios, including `test_create_payment_intent_authenticated` and `test_capture_payment_intent_authenticated`.

  Since all components requested in the issue (API endpoints, Redis Redlock inventory reservation, multi-tenant `TerminalSession` schema, webhook/confirmation logic with agent trigger sequences, and basic tests) are already natively integrated in the Rust workspace, no further work is needed.
