issue_title: "Offline-First Multi-Location Inventory Synchronization Engine"
issue_description: |
  # Title: Offline-First Multi-Location Inventory Synchronization Engine

  ## Problem Statement
  Small business owners like Priya (boutique owner) and Maya (baker) struggle with managing inventory across multiple physical locations (e.g., in-store, pop-up events, online storefront) when internet connectivity drops. If Priya sells an item at a noisy, disconnected trade show using her phone, the online store might oversell that same item before she reconnects to Wi-Fi. Current platforms assume a persistent network connection for critical transactions, causing frustration, manual reconciliation, and overselling—leading to lost revenue and angry customers.

  ## Research Report
  *   **Current Architecture Limits:** OHC relies heavily on synchronous, online-only database writes for inventory deductions. When a mobile client loses connection, transactions either fail or are queued unsafely without global lock coordination, risking split-brain inventory states upon reconnection.
  *   **Competitor Analysis:**
      *   *Shopify:* Shopify POS works offline for cash transactions, but requires a connection for credit cards and real-time inventory sync. It lacks a true distributed offline-first ledger for SMBs.
      *   *Square:* Handles offline payments but struggles with instantaneous multi-location inventory sync when one node goes offline and comes back online.
      *   *Wix:* Highly dependent on centralized online servers.
  *   **Discovery:** OHC needs a local-first, offline-capable synchronization engine based on CRDTs (Conflict-free Replicated Data Types) or a robust event-sourcing ledger stored locally on the mobile client (SQLite), which automatically and safely merges with the central server when connectivity is restored, ensuring zero overselling and seamless multi-channel operations.

  ## Design Doc

  ### Architecture Diagram

  ```mermaid
  erDiagram
      TENANT ||--o{ INVENTORY_ITEM : "owns"
      INVENTORY_ITEM ||--o{ STOCK_LEDGER_ENTRY : "tracked_by"
      MOBILE_CLIENT ||--o{ LOCAL_EVENT_QUEUE : "persists"
      LOCAL_EVENT_QUEUE ||--o{ STOCK_LEDGER_ENTRY : "contains"

      TENANT {
          uuid id
          string name
      }
      INVENTORY_ITEM {
          uuid id
          uuid tenant_id
          string name
          int total_stock
      }
      STOCK_LEDGER_ENTRY {
          uuid id
          uuid inventory_item_id
          int delta
          timestamp created_at
          boolean is_synced
      }
      MOBILE_CLIENT {
          uuid device_id
          uuid tenant_id
          string last_sync_state
      }
  ```

  ### Mobile UX Flow (375px Viewport)
  1. **Inventory Home Card:** Large, tappable cards for top items. A subtle, non-intrusive status indicator at the top right ("Offline: Changes saved locally" in a warm amber pill).
  2. **Sales Flow:** When Priya taps to sell a "Blue Scarf", the stock decreases instantly on her screen from 5 to 4. No spinning loaders. The app feels 100% native and local.
  3. **Background Sync:** The moment the device detects 4G/Wi-Fi, the amber pill turns green ("Synced"). The sync happens invisibly in the background. If a conflict occurs (e.g., the last scarf was sold online simultaneously), an "Action Required" card appears at the top of the feed to handle the conflict or trigger a refund.

  ### AI Agent Integration Points
  *   **The Operations Agent:** Monitors sync events. If a split-brain scenario occurs (overselling), the agent automatically drafts an apologetic SMS to the customer, suggests a similar item or offers a refund, and prompts Priya with a 1-tap "Send & Refund" button on her phone.

  ### Key Design Decisions
  1.  **Local-First Ledger:** Inventory is treated as an event ledger (deltas: +5, -1) rather than absolute values. This allows safe merging of offline transactions.
  2.  **SQLite on Mobile:** The Tauri/Mobile app will use a local SQLite instance to persist the event queue safely.
  3.  **Conflict Resolution via Agent:** Instead of complex UI for conflict resolution, the AI Operations agent handles the edge cases, presenting only actionable choices to the user.

  ## Implementation Prompt
  Implement a CRDT-based or event-sourced offline inventory synchronization engine.
  - **Outcome:** A user must be able to put their phone in Airplane mode, process a sale of an inventory item, see the local stock decrease instantly, and have the transaction safely sync to the backend once Airplane mode is disabled, updating the global stock count without data loss.
  - **CUJ:** Priya at a farmer's market loses signal. She sells 3 candles. Her phone records the sale locally. She regains signal; the main database is updated, and the online store reflects 3 fewer candles.
  - **Acceptance Criteria:**
    1. The mobile client can read and mutate inventory while disconnected.
    2. The server correctly merges offline deltas upon reconnection.
    3. No blocking network calls during the critical path of the checkout flow on the mobile client.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
