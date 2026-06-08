issue_title: "Implement Multi-Channel Inventory Reservation & POS Sync (Priya Persona)"
issue_description: |
  # Research Report: Centralized Inventory & Distributed POS Architecture

  ## 1. Problem Statement
  Service-based small business owners and hybrid retailers (like Priya the Boutique Owner) struggle with fragmented inventory and sales systems. When selling both online and in-store, inventory often falls out of sync. Without a strong centralized lock and eventual consistency mechanism for offline POS sales, merchants face double-bookings, overselling, and manual reconciliation nightmares.

  ## 2. Research Report
  - **Market Context**: Platforms like Shopify handle multi-channel well but require premium tiers and can be complex. Simpler platforms like Square offer great POS but weak native eCommerce inventory sync.
  - **The OHC Opportunity**: By introducing a robust Redis-backed inventory lock during checkout and a robust offline-capable POS sync queue to the central PostgreSQL ledger, OHC can offer an enterprise-grade capability hidden behind a simple interface.
  - **Competitor Gaps**:
    - *Shopify*: Good multi-channel but requires expensive plans or third-party apps for deep sync.
    - *Square*: Excellent POS, weaker native eCommerce.
    - *Wix*: Passive inventory sync.

  ## 3. Design Doc
  ### Data Model (PostgreSQL)
  - `inventory_levels`: Tracks stock per product and location ('online', 'in-store').
  - `agent_action_requests`: Tracks agent-initiated actions like restocking drafts when inventory is low.
  - `pos_terminal_sessions` & `pos_offline_transactions`: Existing tables managing terminal states and queued transactions.

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant MobilePOS as OHC Mobile POS
      participant OnlineCart as OHC Online Cart
      participant Redis as Redis (Redlock)
      participant Postgres as Central Ledger (PostgreSQL)
      participant Agent as Operations Agent

      MobilePOS->>Redis: ReserveItem(product_id, 15s lock)
      Redis-->>MobilePOS: Lock Acquired
      MobilePOS->>Postgres: Commit Transaction (deduct inventory)
      Postgres-->>MobilePOS: Success
      OnlineCart->>Redis: Attempt ReserveItem(product_id)
      Redis-->>OnlineCart: Lock Denied (Item in cart)
      Postgres->>Agent: Low Stock Alert Triggered
      Agent->>Postgres: Create Draft Restock Order
  ```

  ### Mobile UX Flow (375px)
  1. **POS View**: Clean, touch-friendly product catalog. Tapping an item instantly reserves it (optimistic UI update).
  2. **Online View**: If an item is reserved by the POS, the online storefront instantly shows a "1 left - in someone's cart" or "Sold Out" state.
  3. **Owner Dashboard**: The owner receives a push notification and a drafted agent action card to restock the low-inventory item.

  ### AI Integration
  - **Operations Agent**: Monitors the `pos_offline_transactions` queue and stock levels. Upon detecting low stock after a commit, it generates a draft restock order in `agent_action_requests`.

  ## 4. Implementation Prompt
  **Feature Name**: OHC Multi-Channel Inventory Sync & Reservation
  **Target Persona**: Priya the Boutique Owner
  **Outcome**: Priya can sell an item in-store via the mobile POS and have it instantly reserved, preventing an online double-sale. The AI Operations Agent automatically drafts a restock order when inventory drops.

  **Next Actions**:
  1. Ensure the `inventory_levels` and `agent_action_requests` tables are properly migrated and integrated into the data model with RLS.
  2. Implement the Redis reservation logic correctly in the cart/checkout flow.
  3. Update the Operations Agent worker to process `pos_offline_transactions`, reconcile inventory, and generate `agent_action_requests` for low stock.
  4. Ensure robust E2E coverage for the reservation and sync flow.

  **Priority**: P1
  **Estimated Scope**: Large

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
