issue_title: "Implement Multi-Channel Centralized Inventory & Distributed POS Architecture"
issue_description: |
  # Centralized Inventory & Distributed POS Architecture

  ## Problem Statement
  Small business owners like Priya (boutique operator) need seamless inventory tracking between online (web/mobile) and in-store operations (tap-to-pay). Currently, OHC lacks a real-time, strongly consistent inventory locking mechanism, causing double-booking and out-of-stock scenarios during simultaneous online and offline purchases.

  ## Research Report
  - **Shopify POS:** Offers extensive multi-channel sync but can be overly complex and expensive for micro-SMEs, often requiring third-party tools.
  - **Square & Stripe Terminal:** Robust hardware and payment processing, but lacking built-in agentic workflow automation for unifying the full business lifecycle.
  - **OHC Opportunity:** By leveraging centralized PostgreSQL ledgers with distributed Redis locks, and integrating with OHC's Operations and Finance Agents, we can offer a zero-configuration, strongly consistent multi-channel POS that effortlessly prevents double-booking while updating all customer touchpoints instantly.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Mobile POS Client 375px] -->|Offline-first/Sync| B(API Layer Gateway)
      C[Online Storefront] -->|Checkout| B
      B --> D{Redis Redlock distributed lock}
      D -->|Lock acquired| E[PostgreSQL Central Ledger]
      D -->|Lock failed| F[Optimistic UI Rollback & Notify]
      E --> G[Operations Agent The Manager]
      G -->|Low stock alert| H[Owner Work Feed]
      E --> I[Finance Agent The Accountant]
      I -->|Update Reports| J[Unified Dashboard]
      E --> K[Customer Success Agent]
      K -->|Update Online Catalog| C
  ```

  ### Mobile UX Flow (375px first)
  1. **POS Home:** A 375px optimized grid of products. Touch targets >= 44x44px.
  2. **Checkout/Tap-to-Pay:** Owner taps product. System optimistically updates UI, requesting a quick Redis lock (~15s) while payment processes.
  3. **Sync & Resolution:** If online sale happens simultaneously, Redis lock prevents double booking. The losing transaction shows an immediate error overlay ("Item Out of Stock").
  4. **Offline Mode:** If offline, POS queues transaction. Upon reconnection, syncs to ledger with eventual consistency.

  ### AI Agent Integration Points
  - **Operations Agent:** Monitors Redis/Postgres for low stock after transaction commits. Suggests restock actions to the owner feed.
  - **Finance Agent:** Processes split payments from Terminal, matches POS data with online sales for consolidated revenue tracking.
  - **Customer Success Agent:** Instantly pushes out-of-stock updates to the online storefront and draft emails for pre-order waitlists.

  ### Key Design Decisions
  - **Redis Redlock for temporary reservations:** Ensures high-speed locking for concurrent checkouts across online and offline channels without permanently blocking rows in Postgres.
  - **Local-first Mobile Client:** POS must remain responsive; pessimistic locking only occurs during actual checkout, not item browsing.

  ## Implementation Prompt
  Implement the distributed inventory locking system using Redis Redlock for OHC.
  1. Add Redis lock logic to the checkout and POS cart addition flows (lock key pattern: `ohc:lock:{tenant_id}:inventory:{product_id}`).
  2. Create a rollback mechanism for the POS mobile client (Flutter) if a lock acquisition fails, displaying a friendly "Out of Stock" UI state.
  3. Ensure the Operations Agent is triggered when inventory hits the low-stock threshold.
  4. Write Playwright E2E tests simulating simultaneous purchases of a 1-stock item from the POS and online storefront to verify double-booking prevention.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
