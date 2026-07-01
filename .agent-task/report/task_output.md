issue_title: "Implement Multi-Channel Unified Inventory Sync with Offline POS Support"
issue_description: |
  ## Problem Statement
  Boutique owners and local merchants (like Priya) need to sell online and in-store simultaneously without over-selling limited inventory. Current platforms either require complex plugins or fail to provide true real-time synchronization between an online cart and a physical Point-of-Sale (POS) tap-to-pay transaction. When network connectivity drops, merchants lose the ability to reliably reserve inventory, resulting in double-booking scenarios. OHC needs a robust, real-time inventory locking mechanism combined with an offline-tolerant mobile POS client.

  ## Research Report
  - **Competitor Analysis**: Shopify POS handles inventory sync but often requires the Advanced plan for robust multi-location/multi-channel locking. Square is strong offline but disconnected from native online agentic workflows.
  - **The OHC Opportunity**: By leveraging a Redis Redlock mechanism for rapid transaction reservations and a robust Postgres central ledger, OHC can provide enterprise-grade inventory consistency to small businesses.
  - **Persona Focus**: Priya (boutique operator) needs to tap a customer's card in-store and instantly make that unique item unavailable on her website, even during high-traffic online sales.

  ## Design Doc
  ### Architecture Diagram (Mental Model)
  - **Central Ledger (Postgres)**: Source of truth for `inventory_levels`. Requires row-level tenant isolation.
  - **Reservation Cache (Redis)**: Uses Redlock pattern (`ohc:lock:{tenant_id}:inventory:{product_id}`) to temporarily reserve items during the checkout flow (e.g., 5 mins for online cart, 15 seconds for POS).
  - **Offline Client (Flutter/Tauri)**: Caches catalog locally. Uses eventual consistency for offline sales, reconciling with the central ledger upon reconnection.

  ### Mobile UX Flow (375px)
  1. **POS Mode**: Owner opens the OHC app in POS mode. Large touch targets (>= 44px) for products.
  2. **Transaction**: Owner taps to checkout. The app attempts a Redis lock. If successful, the item is reserved.
  3. **Offline Mode**: If offline, the app records a pending transaction locally and flags the item visually as "pending sync".
  4. **Online Conflict Resolution**: If a conflict occurs upon reconnection, the Operations Agent ("The Manager") surfaces an action card in the Agent Feed asking the owner how to resolve it (e.g., refund online order or backorder).

  ### AI Agent Integration
  - **Operations Agent**: Monitors inventory levels. If an item sells out in-store while sitting in an online customer's active cart, the agent intercepts the online checkout and notifies the customer gracefully.

  ## Implementation Prompt
  **Target Persona**: Priya the Boutique Operator
  **User-Facing Outcome**: Priya can confidently sell her last remaining items in-store without fear of a simultaneous online purchase causing an oversell.

  **Acceptance Criteria**:
  1. Implement the Redis Redlock inventory reservation service in the Rust backend (`src/server/services/inventory/lock.rs`).
  2. Update the `inventory_levels` Postgres schema to support robust locking and reconciliation.
  3. Ensure the checkout flow (both API and proposed POS endpoints) respects these locks.
  4. The solution must handle network degradation gracefully, preferring to reject an online sale rather than double-book a physical item if state is uncertain.

  ## Priority: P1
  ## Estimated Scope: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
