issue_title: "Implement Centralized Inventory & Distributed POS Architecture"
issue_description: |
  # Research Report: Centralized Inventory & Distributed POS Architecture

  ## Problem Statement
  Small business owners (e.g., Priya the boutique owner) need seamless inventory tracking between their online storefronts and in-store Point-of-Sale (POS) systems. Currently, OHC lacks a real-time, strongly consistent inventory locking and caching mechanism. Without this, simultaneous online and offline purchases lead to double-booking and out-of-stock scenarios, resulting in poor customer experience and manual reconciliation work for the owner.

  ## Research Report
  Our analysis of the competitor landscape (Shopify, Wix, Square, Stripe Terminal) indicates that while competitors provide POS integrations, their multi-channel inventory synchronization often requires expensive third-party apps or higher-tier plans and complex setup. OHC's differentiation lies in providing an integrated, agent-managed architecture that works invisibly out-of-the-box.
  We must implement a centralized inventory ledger with robust distributed locking to prevent race conditions during checkouts across all channels. Additionally, we need to bridge the gap between offline POS activity (e.g., via Stripe Terminal) and the centralized ledger.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Online Checkout] -->|Reserve| B(Redis Redlock)
      C[In-Store POS / Terminal] -->|Reserve| B
      B -->|Lock Acquired| D[Inventory Service]
      D -->|Commit| E[(PostgreSQL Ledger)]
      D -->|Failure/Low Stock| F[Operations Agent]
      F -->|Notify| G[Owner Mobile App]
      C -.->|Offline Sync| E
  ```

  ### Mobile UX Flow (375px First)
  - **POS Interface:** A clean, touch-friendly grid of products. Tapping a product adds it to the cart. The interface must be responsive and fast.
  - **Checkout:** The checkout flow seamlessly integrates with Stripe Terminal.
  - **Offline Mode:** If the network is unavailable, the POS interface caches transactions locally and syncs them automatically when connectivity is restored, providing optimistic UI updates.
  - **Notifications:** When inventory drops below a threshold, the Operations Agent sends a push notification to the owner suggesting a restock order.

  ### AI Agent Integration Points
  - **Operations Agent ("The Manager"):** Monitors inventory levels. Triggered when an item's stock drops low or hits zero to suggest a restock action. Handles potential conflicts from delayed offline syncs.
  - **Customer Success Agent ("The Ambassador"):** Updates the online storefront availability automatically and can notify customers if an item in their cart becomes unavailable due to an in-store purchase.

  ### Key Design Decisions
  - **Central Ledger:** PostgreSQL remains the single source of truth for inventory counts.
  - **Distributed Locking:** Redis Redlock is used for temporary inventory reservations during the checkout process (both online and POS) to prevent double-booking. The lock TTL will be short to minimize holding times.
  - **Optimistic UI & Offline Sync:** The mobile POS client will use local caching and eventual consistency to ensure in-store operations are not blocked by transient network issues.

  ## Implementation Prompt
  **User-Facing Outcome:** As a business owner, I can sell an item in my physical store using the POS, and the inventory is instantly and reliably deducted from my online store, preventing any accidental overselling. If I'm offline, the app saves the sale and syncs it when I reconnect.

  **Critical User Journey (CUJ) & Acceptance Criteria:**
  1. A user (owner) is logged into the OHC POS app.
  2. The user initiates a checkout for a product with 1 item in stock.
  3. The system acquires a distributed lock via Redis.
  4. While the lock is held, an attempt to purchase the same item online fails gracefully.
  5. The POS transaction completes, and the inventory count in the central PostgreSQL database is decremented.
  6. The lock is released.
  7. If the inventory reaches a low threshold, the Operations Agent generates a task/feed item for the owner to consider restocking.
  8. Write tests that ensure lock contention correctly fails the secondary checkout attempt.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
