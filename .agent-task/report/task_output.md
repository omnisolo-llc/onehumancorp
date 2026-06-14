issue_title: "Implement Unified Multi-Channel Inventory Sync & POS using Redis Redlock"
issue_description: |
  **Problem Statement**:
  Priya, a boutique owner, manages inventory across an in-store POS and an online storefront. Currently, OHC lacks strong consistency and reservation mechanisms for multi-channel sales. This leads to double-booking when an item is simultaneously sold in-store via tap-to-pay and online, frustrating customers and causing manual reconciliation work.

  **Research Report**:
  - Competitors like Shopify and Square offer POS/e-commerce inventory sync, but often at higher complexity/cost tiers and lack the agentic oversight needed for our non-technical owners.
  - A distributed reservation system using Redis Redlock and a PostgreSQL central ledger is needed to provide rapid locking during the fast POS checkout flow, preventing concurrent online purchases of the same stock.
  - Note: Codebase investigation indicates initial skeleton implementations of `ohc:lock:{tenant_id}:inventory:{product_id}` in `src/server/services/inventory/service.rs` and initial offline-sync reconciliation in `src/server/api/offline_sync.rs`. These need to be robustly extended and integrated into a unified agent-driven flow.

  **Design Doc**:
  - **Architecture**:
    - **Central Ledger (PostgreSQL)**: Source of truth for inventory counts with row-level locks on finalization. `products` table handles `inventory_count`, `locked_quantity` and `available_quantity`.
    - **Distributed Locks (Redis)**: `ohc:lock:{tenant_id}:inventory:{product_id}` to reserve items briefly during the checkout/terminal intent phase.
    - **Offline/Local First POS Client**: Eventual consistency for offline sales handled by `pos_terminal_sessions` and `pos_offline_transactions`.
  - **Mobile UX Flow**:
    - 375px first interface. In-store tap-to-pay transaction instantly triggers the lock via `StartTerminalSessionRequest`. If an online user attempts checkout, they see an immediate "Item just sold out" block instead of a post-payment failure.
  - **AI Integration**:
    - **Operations Agent ("The Manager")**: Actively monitors stock levels, tracks incoming orders, triggers low-stock alerts, and coordinates sync mechanism to reconcile conflicts and propose restock plans (e.g. `Reorder` action).

  **Implementation Prompt**:
  Design and implement the robust backend inventory reservation system extending the current Redis Redlock implementation in `src/server/services/inventory/service.rs`. Seamlessly integrate it with `terminal_api.rs` flows and offline-sync reconciliation to form a unified multi-channel POS architecture. Ensure the Operations Agent is fully hooked into these state changes to provide automated notifications and restock prompts. The Critical User Journey must verify that an active in-store Terminal transaction locking the inventory prevents an overlapping online checkout.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
