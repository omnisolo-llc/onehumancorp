issue_title: "Research: Distributed Inventory & POS Architecture for SMB Operations"
issue_description: |
  # Research Report: Centralized Inventory & Distributed POS Architecture

  ## Problem Statement
  Small business owners (like Priya, the boutique operator) struggle to maintain synchronized inventory across their physical and digital storefronts. Current systems often fail to lock items during in-store checkout, resulting in double-booking conflicts when online purchases happen concurrently. They require an invisible, robust inventory system that just works without manual intervention or technical administration.

  ## Research Report
  - Competitors such as Shopify and BigCommerce provide robust backend systems but often abstract POS integrations behind expensive third-party tools or enterprise-tier features.
  - Solutions like Square focus heavily on POS hardware and offline sync, but their deep integration with generic website builders (Weebly, Squarespace) lacks native, AI-agentic automation.
  - OHC differentiation: Our Operations Agent handles real-time syncing between offline tap-to-pay (using Stripe Terminal or similar protocols) and the online catalog invisibly.
  - Critical Gap: Currently, OHC does not enforce cross-channel inventory locking during checkout, meaning a product sold in-store can still be purchased online until asynchronous batch sync completes.

  ## Design Doc
  - **Architecture:**
    - A **Central Ledger (PostgreSQL)** acts as the source of truth, enforcing row-level locking for final commits.
    - A **Distributed Lock Service (Redis Redlock)** is utilized to reserve inventory items immediately upon checkout initiation (either POS tap or online cart addition). Locks should expire quickly (e.g., 15-30 seconds for POS, a few minutes for online).
    - **Local Caching:** POS client keeps an eventually consistent local view but validates immediately against Redis for high-contention items.
  - **AI Agent Integration:** The Operations Agent monitors lock failures and sync conflicts. If an item is oversold due to network partitions, the Customer Success Agent automatically drafts an apology and refund proposal for the owner's review via the Agent Feed.
  - **Mobile UX Flow:**
    - Priya scans/taps an item in the POS (375px viewport optimized).
    - Optimistic UI updates the local count.
    - An underlying request secures the Redis lock.
    - Payment is processed.
    - Central PostgreSQL ledger is updated, and the Redis lock is released.
    - *If lock fails:* The item shows a "Just Sold Out Online" status tag locally, and Priya is notified instantly.

  ## Implementation Prompt
  Implement the distributed inventory locking mechanism for the checkout flow.
  - **CUJ:** As an owner operating the mobile POS, when I initiate checkout for an item, the system should acquire a short-lived distributed lock for that inventory unit. If an online customer attempts to purchase the same unit simultaneously, they should be informed it is temporarily reserved or out of stock.
  - Create the Redis locking utilities and integrate them into the backend checkout service.
  - Ensure the POS UI reflects real-time lock status (optimistic or loading state).
  - Add E2E tests validating that concurrent purchases of the final stock unit result in one success and one graceful rejection. Do not use mocked database or Redis interactions.

  ## Priority: P1
  ## Estimated Scope: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
