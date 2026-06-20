issue_title: "[Research] Implement Centralized Inventory & Distributed POS Architecture"
issue_description: |
  # Research Report: Centralized Inventory & Distributed POS Architecture

  ## Problem Statement
  Small business owners (like Priya the boutique owner) require seamless inventory tracking between online (web/mobile) and in-store operations (tap-to-pay or card reader). Currently, OHC lacks a real-time, strongly consistent inventory locking and caching mechanism, as well as a robust distributed sync protocol for hybrid merchants. Without this, double-booking and out-of-stock scenarios occur during simultaneous online and offline purchases. This causes significant operational pain, lost revenue, and poor customer experiences for non-technical owner/operators.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Shopify:** Dominates the e-commerce space with extensive POS capabilities but often fails micro-SMEs due to complexity. Their inventory management can be disjointed—online inventory frequently falls out-of-sync with in-person sales unless costly third-party integration tools or higher-tier plans are employed.
  - **Square & Stripe Terminal:** Provide robust POS hardware but lack the integrated, agentic workflow automation needed to unify the business operations effortlessly.
  - **OHC Opportunity:** Implement a centralized inventory and distributed Point-of-Sale (POS) synchronization architecture that leverages our AI agents (like "The Manager" and "The Ambassador") to provide a seamless, real-time experience for non-technical users.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Online Checkout] -->|Reserve| B(Redis Redlock)
      C[In-Store POS] -->|Reserve| B
      B --> D{Inventory Ledger}
      D -->|Commit/Rollback| E[PostgreSQL DB]
      E --> F[Operations Agent]
      F -->|Alerts/Restock| G[Action Required Queue]
      G --> H[Mobile App Feed 375px]
  ```

  ### Key Design Decisions
  - **Central Ledger (PostgreSQL):** The ultimate source of truth for all inventory counts, utilizing row-level locking or optimistic concurrency control for critical updates.
  - **Distributed Locks (Redis Redlock):** A temporary inventory reservation system applied during the checkout process to prevent double-booking. The lock duration is dynamically tuned (e.g., 5 minutes for online carts vs. 15 seconds for rapid tap-to-pay transactions). Lock key pattern: `ohc:lock:{tenant_id}:inventory:{product_id}`.
  - **Offline/Local First POS Client:** The mobile POS client caches catalog data locally and employs an eventual consistency mechanism to sync finalized offline sales and reconcile with the central ledger asynchronously when the network is restored.
  - **AI Agent Coordination:**
    - **Operations Agent:** Monitors stock levels, tracks incoming orders, triggers low-stock alerts, and suggests restock plans.
    - **Finance Agent:** Processes splits for Terminal transactions and correlates POS data with online purchases.
    - **Customer Success Agent:** Automatically updates online storefront availability and notifies customers if an item in their cart becomes unavailable.

  ### Mobile UX Flow
  - Ensure the POS interface operates flawlessly on a 375px viewport.
  - Touch targets for inventory adjustment and checkout must be ≥ 44x44px.
  - Implement optimistic UI updates for inventory changes, with rollback capabilities if the Redis reservation fails.

  ## Implementation Prompt
  **User Facing Outcome:** A seamless inventory system where an in-store tap-to-pay purchase instantly reserves and deducts stock, preventing an online customer from double-booking the same item, all managed invisibly by the Operations Agent.
  **CUJ:** Priya makes an in-store sale of a dress. The inventory is instantly reserved via Redis. Simultaneously, an online customer tries to buy the same dress but is notified it's out of stock. Priya sees the updated inventory and a restock suggestion in her mobile feed.
  **Acceptance Criteria:**
  - Online checkout and in-store POS share a unified inventory reservation system.
  - Double-booking is prevented using distributed locks (Redis Redlock).
  - The Operations Agent surfaces low-stock alerts and restock suggestions in the owner's feed.
  - E2E tests verify the reservation and commit logic.
  - UI components follow the 375px mobile-first design system and 44x44px touch targets.

  **Priority**: P1 (High)
  **Estimated Scope**: Large

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
