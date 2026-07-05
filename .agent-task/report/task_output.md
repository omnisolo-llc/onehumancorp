issue_title: "[architecture] Implement Centralized Inventory & Distributed POS Architecture"
issue_description: |
  # Centralized Inventory & Distributed POS Architecture

  ## Problem Statement
  Small business owners (e.g. Priya the boutique owner) struggle with disjointed inventory systems when selling across multiple channels. Specifically, when selling items online via their storefront and in-person via a POS terminal, double-booking and out-of-stock scenarios occur if an item is sold offline while someone is checking out online. Existing platforms like Shopify or Square require complex multi-app setups or lack deeply integrated agentic workflow automation.

  ## Research Report
  - **Competitor Gaps**: Competitors handle multi-channel inventory via either separated systems or require expensive enterprise tiers to provide immediate cross-channel consistency. Most small-business tools (like Wix or GoDaddy) lack robust, optimistic, and low-latency locking mechanisms necessary for high-volume or rapid tap-to-pay checkouts.
  - **The OHC Opportunity**: Integrate inventory directly into a central ledger powered by PostgreSQL, utilizing distributed locks (Redis Redlock) for temporary inventory reservations during checkout flows, and relying on AI agents (Operations & Finance) to manage alerts and auto-reconciliation.

  ## Design Doc
  - **Central Ledger**: The source of truth resides in PostgreSQL (`products` table, with `inventory_count` and `available_quantity`). Updates use row-level locking (`FOR UPDATE`) or Redis locks.
  - **Distributed Locks**: Use Redis Redlock for temporary inventory reservation. Key pattern: `ohc:lock:{tenant_id}:inventory:{product_id}`. Timeouts are dynamically tuned (e.g. 5 minutes for online, 15 seconds for POS).
  - **Offline/Local First Client**: Mobile POS client caches catalog locally, processing actions optimistically via the `OperationIntent` mechanism, and syncing back asynchronously when online.
  - **AI Coordination**:
    - **Operations Agent ("The Manager")**: Monitors stock, tracks orders, triggers low-stock alerts and drafts restock plans.
    - **Customer Success Agent ("The Ambassador")**: Updates online storefront and notifies customers if an item in their cart becomes unavailable due to an offline purchase.

  ### Mobile UX Flow (375px)
  1. Priya processes an in-store sale via Stripe Terminal in the OHC mobile app.
  2. The system acquires a 15-second Redis lock to reserve the item.
  3. Meanwhile, an online customer attempting to buy the identical item sees a graceful "Item just sold out" notice.
  4. The transaction finalizes, the PostgreSQL ledger updates, and the Operations Agent sends Priya a "Red Dress sold out. Restock?" push notification.

  ## Implementation Prompt
  **Feature Name**: OHC Unified Multi-Channel Inventory Sync & POS
  **Target Persona**: Priya the Boutique Owner
  **Outcome**: Prevent double-booking across online and offline channels via synchronized inventory locking.

  **Next Actions**:
  1. Implement Redis Redlock inventory reservation service and integrate it into both the online checkout and POS terminal flows.
  2. Enhance the `TerminalSession` schema and offline sync handlers (`OperationIntent`) to reconcile offline terminal sales with the PostgreSQL central ledger, using the reservation logic.
  3. Extend the Operations Agent to trigger real-time push notifications when stock drops below thresholds or hits zero, using the Teammate Mesh to broadcast updates.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
