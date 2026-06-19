issue_title: "Implement Centralized Inventory & Distributed POS Architecture"
issue_description: |
  # Research Report: Centralized Inventory & Distributed POS Architecture

  ## Title
  Implement Centralized Inventory & Distributed POS Architecture

  ## Problem Statement
  Priya (boutique owner) requires seamless inventory tracking between online (web/mobile) and in-store operations (tap-to-pay or card reader). Currently, OHC lacks a real-time, strongly consistent inventory locking and caching mechanism, as well as a robust distributed sync protocol for hybrid merchants. Without this, double-booking and out-of-stock scenarios occur during simultaneous online and offline purchases.

  ## Research Report
  - **Market Context:** Competitors like Shopify dominate the e-commerce space with extensive POS capabilities but often fail micro-SMEs due to complexity. Their inventory management can be disjointed—online inventory frequently falls out-of-sync with in-person sales unless costly third-party integration tools or higher-tier plans are employed. Square and Stripe Terminal provide robust POS hardware but lack the integrated, agentic workflow automation needed to unify the business operations effortlessly.
  - **OHC Gap:** Currently, OHC lacks a real-time, strongly consistent inventory locking and caching mechanism, as well as a robust distributed sync protocol for hybrid merchants. Without this, double-booking and out-of-stock scenarios occur during simultaneous online and offline purchases.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Mobile POS Client (Offline First)] -->|Sync Sales / Reconcile| B(Central Ledger - PostgreSQL)
      C[Online Storefront] -->|Checkout Process| B
      B --> D{Redis Redlock (Distributed Locks)}
      D -->|15s Lock| E[In-Store Transaction]
      D -->|5m Lock| F[Online Cart Checkout]
      G[Operations Agent] -->|Monitor / Alert| B
      G -->|Update Availability| C
  ```

  ### Mobile UX Flow
  1. **POS Dashboard (375px):** A clean, simple interface displaying the product catalog with large touch targets (≥ 44x44px) for quick selection.
  2. **Transaction Flow:** Priya taps "Red Dress", selects "Charge via Terminal".
  3. **Optimistic UI:** The app immediately deducts the item locally and displays "Processing...".
  4. **Background Sync:** The system attempts to acquire a 15-second Redis Redlock. If successful, it finalizes the transaction. If the lock fails (e.g., online customer just bought it), the UI rolls back with a clear error message.

  ### AI Agent Integration
  - **Operations Agent ("The Manager"):** Actively monitors stock levels across all channels. It tracks incoming orders, triggers low-stock alerts, coordinates with the sync mechanism to reconcile conflicts, and suggests restock plans.
  - **Finance Agent ("The Accountant"):** Processes splits for Terminal transactions and correlates POS data with online purchases for unified financial reporting.
  - **Customer Success Agent ("The Ambassador"):** Automatically updates online storefront availability and notifies customers if an item in their cart becomes unavailable due to an in-store purchase.

  ## Implementation Prompt
  **User-Facing Outcome:** Priya the boutique owner can process an in-store purchase via the POS client, which instantly reserves inventory using a robust distributed lock, preventing online double-bookings.
  **CUJ & Acceptance Criteria:**
  1. An online customer begins checkout for the last item in stock.
  2. Priya simultaneously processes an in-store sale for the same item.
  3. The Redis Redlock system guarantees only one transaction succeeds, with the other gracefully declining.
  4. The Central Ledger is updated consistently without race conditions.
  5. The Operations Agent correctly detects the stockout and generates an alert.

  Implement the Centralized Inventory & Distributed POS Architecture.
  1. Design and implement the Central Ledger in PostgreSQL with row-level locking for critical updates.
  2. Implement the Redis Redlock mechanism for temporary inventory reservation during checkout (configurable durations for POS vs. Online).
  3. Develop the core API endpoints for the POS client to sync offline sales and reconcile with the central ledger.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
