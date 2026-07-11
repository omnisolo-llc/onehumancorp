issue_title: "Implement Distributed Locks & Sync for POS Terminal Inventory"
issue_description: |
  # Research Report: Centralized Inventory & Distributed POS Architecture

  ## Problem Statement
  For multi-channel business owners like Priya (Boutique Operator) who sell online and in-store simultaneously, managing inventory is painful. The OHC platform currently lacks a robust mechanism to prevent double-booking during checkout. When an item is sold via a physical point-of-sale (POS) terminal, an online user can theoretically check out the same final item concurrently because there is no robust lock and sync protocol tying terminal sessions to central Postgres inventory.

  ## Research Findings
  Based on the `docs/business/market_research/[research]_ohc_centralized_inventory_pos.md` and codebase audits:
  - There is a recognized need to track central inventory closely across terminal taps and web carts.
  - The system requires a temporary inventory reservation logic (e.g. Redis Redlock) during checkout processing to prevent double-booking.
  - While offline POS architectures allow for local caching, hybrid systems must perform eventual consistency with a centralized ledger immediately on network restore, or synchronously lock when online.
  - Missing Data Model: `TerminalSession` is loosely referenced but its schema does not natively exist in our models, preventing proper offline-sync and reconciliation logic.

  ## Design Doc
  - **Architecture Diagram (Mental Model)**:
    - POS Client <-> OHC API <-> Redis (Redlock: `ohc:lock:{tenant_id}:inventory:{product_id}`)
    - OHC API <-> PostgreSQL (Central `LedgerEntry` or `Product` / `Inventory` counts)
  - **Mobile UX Flow**:
    - At 375px viewport width, Priya selects an item to charge via Terminal.
    - System shows an "acquiring lock" invisible state (or instant success if online).
    - If the item was just sold online, an error displays immediately: "Item just sold out. Please refresh inventory."
  - **AI Agent Integration**:
    - The Operations Agent ("The Manager") is triggered when a low inventory count or a sync conflict occurs, advising the owner via a pushed action card.
  - **Key Design Decisions**:
    - **Redis for distributed locking**: Rapid locking (e.g. 15 seconds) keeps sales fast but strictly serializes contention for limited stock.
    - **TerminalSession entity**: To track offline or pending terminal operations, a schema change must be made to properly represent a `TerminalSession` that contains offline transaction state for reconciliation later.

  ## Implementation Prompt
  **Title**: Implement Distributed Inventory Locks & TerminalSession entity.

  **Outcome**: Provide the backend structural support to prevent double-booking between online carts and POS terminals.

  **Instructions for Implementer Agent**:
  1. Add a `TerminalSession` data schema to `src/server/domain/repository/models.go` and the corresponding SQL initialization if necessary.
  2. Implement a `Redis Redlock` based distributed lock function for reserving inventory during checkout. This should use a lock key pattern like `ohc:lock:{tenant_id}:inventory:{product_id}` with an explicit timeout.
  3. Ensure that when a POS checkout initiates, it attempts to acquire the lock. If successful, proceed; if it fails (because the web cart is finalizing it), return a meaningful error that the UI can surface gracefully.

  **Acceptance Criteria**:
  - `TerminalSession` struct exists.
  - A Redis-backed locking mechanism is implemented and testable.
  - E2E or Integration tests prove that concurrent requests for the final item succeed for one and fail gracefully for the other.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
