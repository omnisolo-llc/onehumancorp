issue_title: "Implement Distributed Inventory Lock System for POS"
issue_description: |
  # Research Report: Implement Distributed Inventory Lock System for POS

  ## Problem Statement
  Currently, the OHC system lacks a robust mechanism to prevent double-booking or overselling inventory when simultaneous purchases occur across different channels (e.g., online checkout vs. in-store Point-of-Sale). For non-technical owners like Priya (boutique owner), this creates significant operational headaches, lost revenue, and poor customer experiences due to stockouts. There's no real-time locking mechanism that spans both online and offline operations.

  ## Research Report
  - Competitors like Shopify and Square handle this through complex backend integrations and specialized hardware, but OHC needs an agentic, seamless approach.
  - The goal is to implement a Redis-based distributed locking system (Redlock pattern) specifically for inventory reservations during the checkout phase.
  - This lock needs to be dynamic (e.g., 5-minute hold for online carts, 15-second hold for fast in-store tap-to-pay).
  - This architecture supports a "Local First" mobile POS client that synchronizes when network is available but relies on the lock to prevent immediate overselling conflicts.

  ## Design Doc
  - **Architecture:** Implement a Redis-backed distributed lock manager using the Redlock algorithm.
    - Locks will be keyed by tenant and product: `ohc:lock:{tenant_id}:inventory:{product_id}`.
    - Integration with the central PostgreSQL ledger (acting as the source of truth).
  - **Mobile UX Flow (375px):**
    - When a user taps a product to buy/reserve, the UI immediately shows an "allocating..." optimistic state.
    - If lock fails (item already reserved elsewhere), UI smoothly rolls back and displays "Item just sold out!" with an option to notify when back in stock.
  - **AI Agent Integration:**
    - The Operations Agent ("The Manager") receives events when inventory is locked/unlocked to monitor velocity and trigger low-stock alerts.

  ## Implementation Prompt
  - Build the distributed lock module in Rust (e.g., `src/server/tools/inventory_lock.rs`).
  - Create the Redis connection and lock acquisition/release logic using `redis` crate or similar.
  - Implement dynamic lock durations (e.g., `CheckoutContext::Online` vs `CheckoutContext::InStore`).
  - Ensure the solution is fully tested with unit tests simulating concurrent lock acquisition attempts.
  - Ensure API endpoint for acquiring locks is available for the Flutter/PWA client.

  ## Priority: P1
  ## Estimated Scope: Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
