issue_title: "Unified Multi-Channel Inventory Sync & POS (Priya's Centralized Architecture)"
issue_description: |
  # Mission Queue Protocol: Centralized Inventory & Distributed POS Architecture

  **Problem Statement:**
  Multi-channel small businesses like Priya's boutique struggle with inventory management across online and in-store channels. A tap-to-pay transaction in-store might happen simultaneously with an online checkout, causing double-bookings and out-of-stock scenarios. The existing architecture lacks real-time, strongly consistent inventory locking and a robust distributed sync protocol for POS.

  **Research Report:**
  Based on `docs/business/market_research/[research]_ohc_centralized_inventory_pos.md` and codebase analysis, Shopify dominates this space but is too complex for micro-SMEs, while Square lacks agentic integration. OHC requires a seamless inventory system using Redis Redlock for temporary checkout reservations (e.g., 15s for tap-to-pay, 5m for online), PostgreSQL as the central ledger, and background AI Agent reconciliation.

  **Design Doc:**
  - **Architecture:**
    - *Ledger:* PostgreSQL holds authoritative stock counts.
    - *Reservation:* Redis Redlock (`ohc:lock:{tenant_id}:inventory:{product_id}`) manages optimistic checkout locks to prevent double booking.
    - *AI Operations Agent:* Monitors stock levels, manages restock alerts, and resolves sync conflicts.
  - **Mobile UX Flow (375px first):**
    - The POS mobile view displays product variants with touch targets >= 44x44px.
    - During an in-store transaction (Stripe Terminal), optimistic UI immediately reserves the item locally, triggering a Redis Redlock in the background.
    - If the lock fails, the POS gracefully alerts the cashier; if successful, the ledger updates post-payment.
  - **Agent Integration:**
    - The Operations Agent coordinates with the Customer Success Agent to gracefully update online carts if an item sells out in-store.

  **Implementation Prompt for Engineering Swarm:**
  Implement the Redis Redlock inventory reservation service and integrate it into both online checkout and POS Terminal session flows. Update the `TerminalSession` data schema to handle offline-sync reconciliation with the PostgreSQL central ledger. Extend the Operations Agent logic to monitor stock levels and trigger low-stock notifications. Ensure all POS UI components render perfectly on a 375px viewport with adequate touch targets.

  **Priority:** P1
  **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
