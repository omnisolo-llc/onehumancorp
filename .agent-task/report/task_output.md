issue_title: "Implement Unified Multi-Channel Inventory Sync & POS using Redis Redlock and TerminalSession sync"
issue_description: |
  **Problem Statement**
  Currently, OHC lacks a robust, real-time mechanism to synchronize inventory across online storefronts and in-store Point-of-Sale (POS) systems. This gap often leads to double-booking and out-of-stock scenarios when simultaneous online and offline purchases occur. This is a critical pain point for hybrid merchants like Priya (boutique owner) who rely on real-time inventory tracking to prevent disappointing customers and losing sales.

  **Research Findings**
  Competitors like Shopify provide POS capabilities but often require expensive third-party integration tools for seamless inventory sync, making them inaccessible to micro-SMEs. OHC can differentiate by offering a natively integrated, real-time inventory locking mechanism using Redis Redlock for temporary reservations and a central PostgreSQL ledger for eventual consistency during offline POS transactions.

  **Design Doc**

  **Architecture Diagram**
  ```mermaid
  graph TD
      A[Online Store Checkout] -->|Request Lock| B(Redis Redlock: 5 min)
      C[Mobile POS / Tap-to-Pay] -->|Request Lock| B(Redis Redlock: 15 sec)
      B --> D{Lock Acquired?}
      D -->|Yes| E[Process Payment via Stripe]
      D -->|No| F[Return 'Out of Stock' Error]
      E --> G[Update Central Ledger: PostgreSQL]
      G --> H[Operations Agent: The Manager]
      H -->|Low Stock Alert| I[Notify Owner / Update UI]
  ```

  **UI Wireframes & Mobile UX Flow (375px viewport)**
  1. **POS Main Screen:** Clean catalog grid of products. Tapping a product immediately adds it to the cart. Large, high-contrast text and 44x44px minimum touch targets. Translucent Glass styling.
  2. **Checkout Initiated:** When the user hits the "Charge" button, a subtle loading spinner appears indicating the system is reserving inventory (Redis Redlock).
  3. **Payment Terminal Screen:** The screen displays "Tap or insert card" while interfacing with the Stripe Terminal SDK.
  4. **Conflict State:** If the item was just sold online (Redlock fails), a graceful error toast appears: "Item sold out online just now." The transaction is aborted before payment.
  5. **Success State & Async Sync:** Upon payment success, a green confirmation screen appears. The POS client updates its local cache and asynchronously pushes the finalized transaction to PostgreSQL to permanently deduct the inventory.

  **Implementation Prompt**
  Implement the Redis Redlock inventory reservation service and integrate it into both the online checkout and the new mobile POS flow.
  - The POS client must seamlessly interface with Stripe Terminal for payments.
  - Ensure the `TerminalSession` data schema handles offline-sync reconciliation with the PostgreSQL central ledger.
  - Extend the Operations Agent to monitor real-time stock levels, handle sync conflicts, and trigger low-stock push notifications.
  - **Acceptance Criteria:** E2E Playwright tests must verify the complete inventory reservation flow, proving that simultaneous online and offline checkouts cannot double-book a single item.

  **Estimated Scope:** Large

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
