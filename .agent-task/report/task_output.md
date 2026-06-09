issue_title: "OHC Multi-Channel Inventory Sync & POS Architecture Integration"
issue_description: |
  **Problem Statement:**
  Currently, OneHumanCorp (OHC) lacks a robust, real-time distributed synchronization mechanism for inventory between online storefronts and in-store Point-of-Sale (POS) clients. This leads to double-booking out-of-stock items when simultaneous online and offline purchases occur, especially under flaky network conditions.

  **Research Report:**
  - *Competitor Analysis:* Shopify and Square handle this through complex proprietary POS hardware and third-party tools. For micro-SMEs, this setup is excessively expensive and technically burdensome.
  - *Data:* The current OHC `pos_sync_worker.go` and `offline_sync.go` handles offline sales sync by deducting inventory, but lacks the robust row-level concurrency protection and distributed locking necessary for true real-time hybrid operations.
  - *Opportunity:* By leveraging Redis Redlock for short-lived reservation locks during checkout and an eventual-consistency offline sync queue, OHC can ensure absolute inventory accuracy across all channels seamlessly.

  **Design Doc:**
  - *Architecture:*
    - **Central Ledger (PostgreSQL):** Source of truth. Uses row-level locking (`FOR UPDATE`) for critical stock decrements.
    - **Distributed Locks (Redis Redlock):** 15-second reservation applied during tap-to-pay checkouts. Lock key: `ohc:lock:{tenant_id}:inventory:{product_id}`.
    - **Offline/Local First POS Client:** Caches catalog, eventually syncs finalized offline sales (`offline_pos_sync` job queue) and reconciles with the central ledger.
  - *Mobile UX Flow:* A boutique owner processes a tap-to-pay sale. The item is temporarily locked. If an online customer attempts to buy the same item concurrently, they receive a graceful "Item just sold out" message.
  - *AI Agent Integration:* The Operations Agent monitors stock levels. If an offline sale causes a sync conflict with an online cart, the agent drafts a polite explanatory email to the online customer and prompts the owner to restock.

  **Implementation Prompt:**
  - Implement a Redis Redlock reservation system in the checkout flow to prevent double-booking.
  - Ensure the PostgreSQL database implements optimistic concurrency control or row-level locks for inventory updates.
  - Refine the `offline_pos_sync` worker to handle sync reconciliation robustly, including rollback/compensation logic on failures.
  - Connect the Operations Agent to trigger an alert/restock draft when stock falls below threshold or when an offline-online conflict occurs.

  **Priority:** P1
  **Estimated Scope:** Large

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
