issue_title: "Implement Multi-Channel Inventory Sync & Distributed POS Lock (Redis Redlock) - Phase 1"
issue_description: |
  # OHC Unified Multi-Channel Inventory Sync & POS Research & Design

  ## Problem Statement
  Boutique owners and physical retail operators (like Priya) need seamless inventory tracking between online (web/mobile) and in-store operations (tap-to-pay or card reader). Currently, OHC lacks a real-time, strongly consistent inventory locking mechanism for hybrid merchants, leading to double-booking and out-of-stock scenarios when simultaneous online and offline purchases occur. The non-technical owner needs an invisible AI agent (the Operations Agent) to handle these sync conflicts gracefully.

  ## Research Report & Market Context
  - Traditional platforms (Shopify) rely on expensive tier upgrades or complex third-party tools to achieve robust online-offline inventory sync.
  - Micro-SMEs find managing disjointed inventory systems overwhelming, leading to manual errors and poor customer experiences.
  - Using a distributed lock pattern (Redis Redlock) ensures that a physical Point-of-Sale (POS) transaction reserves the item instantly, preventing concurrent online checkouts from claiming the same stock.
  - This feature directly addresses Priya's critical business need for unified commerce, eliminating the "app tax" and manual synchronization burden.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[POS Client/App] -->|Reserve Item Request| B(Inventory Service)
      C[Online Store Checkout] -->|Reserve Item Request| B
      B --> D{Redis Redlock}
      D -->|Lock Acquired| E[Process Payment]
      D -->|Lock Denied| F[Return 'Out of Stock' Error]
      E --> G[Update Postgres Ledger]
      G --> H[Operations Agent Alert]
  ```

  ### Mobile UX Flow (375px)
  1. Priya processes an in-store sale using the mobile POS interface.
  2. During checkout, an optimistic UI indicates the item is reserved.
  3. If an online customer tries to buy the same item concurrently, they receive a clean "Item just sold out" notification.
  4. Once the POS sale completes, the Operations Agent sends Priya a notification (if stock is low): "Red Dress sold out. Would you like to draft a restock order?".

  ### AI Agent Integration
  - **The Operations Agent:** Monitors real-time stock levels. It uses the `InventoryUpdated` event to trigger low-stock alerts or restock suggestions in the owner's Agent Feed.
  - **The Customer Success Agent:** In future iterations, this agent can proactively email customers if their cart items become unavailable due to an in-store sale.

  ## Implementation Prompt
  We need to strengthen the multi-channel inventory synchronization by formalizing the Redis Redlock reservation system during checkout flows and extending the Operations Agent to handle the resulting stock events.

  **Outcome:**
  A seamless inventory system where an in-store or online purchase attempts to acquire a short-lived Redis lock on a specific `product_id`. If successful, the transaction proceeds; if not, the system cleanly fails the concurrent request, preventing double-booking. The Operations Agent must monitor these stock deductions and proactively suggest restock actions to the owner when thresholds are breached.

  **Acceptance Criteria:**
  1.  **Inventory Reservation Service (Backend):** Implement or refine a Redis-backed distributed lock (Redlock pattern) for reserving inventory during checkout. The lock key should follow the pattern `ohc:lock:{tenant_id}:inventory:{product_id}`.
  2.  **Concurrency Handling:** Ensure the checkout API respects this lock, returning a graceful error if the lock cannot be acquired (e.g., "Item is currently being checked out by another customer.").
  3.  **Operations Agent Triggers:** When inventory is successfully deducted and drops below a defined threshold (e.g., <= 5), the system must generate a `LowStockAlert` task for the Operations Agent.
  4.  **Agent Feed Integration:** The Operations Agent should push an actionable item to the owner's feed (e.g., "Draft restock order for [Item]").
  5.  **Testing (MANDATORY):**
      - Unit tests for the Redis lock acquisition and release.
      - E2E Playwright test simulating concurrent checkouts (one succeeding, one failing gracefully).
      - Ensure 100% test coverage for new/modified code. No mock data in the UI; data must flow through the real stack.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
