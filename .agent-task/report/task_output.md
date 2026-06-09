issue_title: "Implement Centralized Inventory & Distributed POS Architecture"
issue_description: |
  # Centralized Inventory & Distributed POS Architecture

  ## Problem Statement
  Small business owners like Priya (boutique operator) need seamless inventory tracking between their online store and in-person tap-to-pay sales. Currently, OHC lacks a real-time, strongly consistent inventory locking mechanism. This leads to double-booking when an item is purchased online and in-store simultaneously, causing frustration and lost sales. We need an architecture that unifies these channels invisibly.

  ## Research Report
  - **Market Dynamics:** Legacy platforms like Shopify require complex third-party tools or high-tier plans to synchronize offline POS with online inventory effectively. Square is great for POS but lacks agentic workflow automation.
  - **OHC Opportunity:** By utilizing an Operations Agent ("The Manager") alongside a distributed lock system (Redis Redlock), we can guarantee inventory consistency. If an item is reserved in-store, it becomes instantly unavailable online. The agent handles low-stock alerts and restock drafting without requiring the owner to look at dashboards.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Online Checkout] --> B(Inventory Lock Service)
      C[Mobile POS / Tap-to-Pay] --> B
      B -->|Request Lock| D[Redis Redlock]
      D -->|Lock Granted| E[Central Ledger - PostgreSQL]
      E --> F[Operations Agent - The Manager]
      F -->|Alert/Action| G[Owner Mobile Feed]
      F -->|Update Storefront| H[Storefront UI Cache]
  ```

  ### Mobile UX Flow (375px First)
  - **POS Interface:** Clean, large touch targets (>= 44x44px) for adding items to the cart and tapping to pay.
  - **Real-time Updates:** If stock is running low, a non-intrusive badge appears on the item in the catalog.
  - **Conflict Resolution:** If an online customer tries to buy a locked item, they see a graceful "Item just sold out" state.
  - **Owner Notification:** The owner receives an actionable feed item: "Red Dress sold out. Tap to draft a restock order."

  ### AI Agent Integration Points
  - **The Manager (Operations Agent):** Monitors the central ledger. When stock hits a threshold, it drafts a restock order. If an online checkout fails due to a POS lock, the agent can optionally trigger "The Ambassador" to offer the customer a similar item.

  ### Key Design Decisions
  - **Redis Redlock:** Used for short-term reservations (e.g., 15 seconds for POS, 5 minutes for online carts) to prevent race conditions.
  - **Eventual Consistency Offline:** The mobile client caches catalog data. If offline, transactions are queued and reconciled with the central ledger when connectivity returns.

  ## Implementation Prompt
  **User-Facing Outcome:** Priya can ring up a customer in her boutique for the last "Red Dress". The moment she starts the transaction, the dress shows as "Sold Out" on her website, preventing double-booking. After the sale, her agent suggests ordering more.
  **CUJ & Acceptance Criteria:**
  1. Set up the Redis Redlock mechanism for inventory reservation.
  2. Integrate the lock into the online checkout and POS transaction flows.
  3. Ensure the Operations Agent is triggered on inventory changes to check thresholds and draft restock tasks.
  4. Create Playwright E2E tests demonstrating a simultaneous online/POS purchase attempt where the POS transaction successfully locks the item and the online checkout gracefully handles the out-of-stock state.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
