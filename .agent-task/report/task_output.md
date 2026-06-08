issue_title: "Implement Multi-Channel Inventory Sync & POS Conflict Resolution using Redis Redlock"
issue_description: |
  # Research Report: Implement Multi-Channel Inventory Sync & POS Conflict Resolution using Redis Redlock

  ## 1. Problem Statement
  Priya (boutique owner) requires seamless inventory tracking between online (web/mobile) and in-store operations (tap-to-pay or card reader). Currently, OHC lacks a real-time, strongly consistent inventory locking mechanism, which leads to double-booking and out-of-stock scenarios during simultaneous online and offline purchases. The Operations agent needs to actively monitor stock and manage this sync.

  ## 2. Research Report
  - **Market Context**: Platforms like Shopify have extensive POS capabilities, but the integration between online and in-store inventory can be laggy or require expensive third-party tools for smaller merchants. Stripe Terminal provides the hardware, but lacks the integrated agentic workflow to prevent double booking.
  - **The OHC Opportunity**: By using Redis Redlock to establish short-lived distributed locks on inventory items during the checkout/POS flow, we can completely eliminate double-booking without requiring Priya to think about database sync.
  - **Competitor Gaps**: Wix and Squarespace offer basic POS but lack real-time distributed locking and proactive agent-driven out-of-stock management.

  ## 3. Design Doc
  ### Architecture & Data Model
  - **Central Ledger (PostgreSQL)**: Source of truth for inventory counts, utilizing row-level locking or optimistic concurrency.
  - **Distributed Locks (Redis Redlock)**: A temporary reservation system during checkout. Lock duration varies (e.g., 5 min for online cart, 15 sec for POS).
    - Lock Key Pattern: `ohc:lock:{tenant_id}:inventory:{product_id}`
  - **Offline/Local-First POS**: Mobile POS client caches catalog, uses eventual consistency to sync offline sales when network restores.

  ### AI Integration
  - **Operations Agent**: Monitors stock levels, triggers low-stock alerts, handles sync conflicts, and suggests restock plans.
  - **Customer Success Agent**: Updates online storefront availability and notifies customers if an item in their cart becomes unavailable.

  ### Diagrams

  #### Entity-Relationship Diagram
  ```mermaid
  erDiagram
      Tenant ||--o{ InventoryItem : owns
      InventoryItem ||--o{ RedisLock : "temporarily reserved by"
      InventoryItem ||--o{ Transaction : included_in
      Transaction ||--o{ TerminalSession : processed_by

      Tenant {
          uuid id
          string name
      }
      InventoryItem {
          uuid id
          uuid tenant_id
          int stock_count
          string product_name
      }
      RedisLock {
          string key
          uuid tenant_id
          uuid inventory_item_id
          datetime expires_at
      }
      Transaction {
          uuid id
          uuid tenant_id
          string source "online | pos"
      }
      TerminalSession {
          uuid id
          uuid tenant_id
          string device_id
      }
  ```

  #### Sequence Diagram
  ```mermaid
  sequenceDiagram
      actor Customer
      actor Priya
      participant POS as POS Client
      participant Online as Online Storefront
      participant API as OHC API Layer
      participant Redis as Redis Redlock
      participant DB as PostgreSQL Ledger
      participant OpsAgent as Operations Agent

      Priya->>POS: Initiates checkout for "Red Dress"
      POS->>API: Process Transaction
      API->>Redis: Acquire Lock (ohc:lock:{tenant_id}:inventory:{product_id})
      Redis-->>API: Lock Acquired (15s TTL)

      Customer->>Online: Clicks "Checkout" for "Red Dress"
      Online->>API: Attempt to Buy
      API->>Redis: Check Lock (ohc:lock:{tenant_id}:inventory:{product_id})
      Redis-->>API: Lock Exists (Reserved)
      API-->>OpsAgent: Notify: Item Reserved
      OpsAgent-->>Online: Message: "Item just sold out"

      API->>DB: Finalize POS Sale & Deduct Inventory
      DB-->>API: Success
      API->>Redis: Release Lock
      OpsAgent-->>Priya: Notification: "Red Dress sold out. Restock?"
  ```

  ### Mobile UX Flow (375px)
  1. **POS Checkout**: Priya uses the mobile app (POS mode). During a sale, the system applies a fast Redis lock.
  2. **Conflict Resolution**: If an online customer tries to buy the same item, the Operations Agent intercepts and shows a graceful "Item just sold out" message.
  3. **Notification**: Once the transaction completes, the Operations Agent sends Priya a notification: "Item sold out. Want to reorder?"

  ## 4. Implementation Prompt
  **Feature Name**: OHC Unified Multi-Channel Inventory Sync & POS

  **Target Persona**: Priya the Boutique Owner

  **Outcome**: A seamless inventory system where an in-store tap-to-pay purchase instantly reserves and deducts stock, preventing an online customer from double-booking the same item, managed invisibly by the Operations Agent.

  **Critical User Journey (CUJ)**:
  1. Priya processes an in-store sale for the last "Red Dress" using the POS app.
  2. The system applies a Redis Redlock to reserve the item.
  3. An online customer tries to checkout the "Red Dress" but receives a graceful "Sold out" message.
  4. The POS transaction finalizes, the DB updates, and the Operations Agent notifies Priya.

  **Next Actions for Engineering**:
  1. Implement the Redis Redlock inventory reservation service and integrate into the POS/online checkout flows.
  2. Refine the data schema (e.g., `TerminalSession` and `InventoryItem`) to handle sync reconciliation with Postgres.
  3. Extend the Operations Agent to monitor stock, handle conflicts, and trigger notifications.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
