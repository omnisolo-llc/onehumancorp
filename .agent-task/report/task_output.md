issue_title: "Implement Centralized Inventory & Distributed POS Architecture"
issue_description: |
  # Architecture Implementation Brief: OHC Unified Multi-Channel Inventory Sync & POS

  ## Mission
  Design and implement a robust, real-time centralized inventory and distributed Point-of-Sale (POS) synchronization architecture for OneHumanCorp. This is essential for micro-SME personas like "Priya the Boutique Owner" who need seamless inventory tracking between online storefronts and in-store sales (via tap-to-pay/card readers) to prevent double-booking.

  ## Problem Statement
  Currently, OHC lacks a real-time, strongly consistent inventory locking mechanism and an effective distributed sync protocol for hybrid merchants. Without these, double-booking and out-of-stock scenarios occur when items are simultaneously purchased online and offline.

  ## Research Findings & Gap
  Competitors like Shopify require expensive 3rd-party apps for cohesive omnichannel POS integration, and their default inventory systems are complex for a micro-SME. OHC must provide this out-of-the-box using:
  - Central Postgres ledger with locking.
  - Distributed Redis Redlocks to hold inventory reservations during checkout.
  - Eventual consistency sync for offline POS devices once network connectivity returns.

  ## Design & Architecture (High-Level)

  ### Data Model
  * **Central Ledger:** PostgreSQL tables (e.g. `inventory_items`, `terminal_sessions`). Use row-level locking for critical write paths.
  * **Distributed Locks:** Redis Redlock pattern (key: `ohc:lock:{tenant_id}:inventory:{product_id}`). Dynamic TTL based on channel (e.g. 5m for online carts, 15s for tap-to-pay).
  * **Client Sync:** Mobile POS clients cache catalog data. Offline sales generate local transactions synced asynchronously when the network is restored.

  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  sequenceDiagram
      participant C as Online Customer
      participant M as POS Client (Mobile)
      participant R as Redis (Redlock)
      participant P as Postgres (Ledger)
      participant A as Operations Agent

      M->>R: Request lock (tap-to-pay intent)
      R-->>M: Lock Acquired (15s TTL)
      C->>R: Request checkout (online cart)
      R-->>C: Lock Denied
      C->>A: Trigger Cart Error
      A-->>C: "Item just sold out in-store"
      M->>P: Finalize POS transaction
      P-->>M: Inventory updated
      M->>R: Release lock
      A->>P: Check inventory level
      A-->>M: Push Notification: "Item sold out. Restock?"
  ```

  ### Mobile UX Flow (375px)
  * **Screen 1: POS Catalog.** Clean, grid layout showing items. Large tap targets (>= 44x44px).
  * **Screen 2: Checkout / Tap-to-Pay.** Minimalist UI. Large central button indicating "Tap Card". Translucent glass materials. As soon as this screen is opened, optimistic locking starts in the background.
  * **Screen 3: Online Storefront (Conflict View).** If the item sells offline while a user has it in their online cart, the screen gracefully transitions the "Checkout" button to a disabled gray "Sold Out" state, with an overlay note from the Customer Success Agent explaining the item just sold locally.

  ### AI Agent Integration
  * **Operations Agent:** Monitors inventory levels. Upon low stock or sync conflict, it triggers an alert and suggests restock plans.
  * **Finance/Customer Success Agent:** Notifies customers if carts expire or items sell out before checkout completion.

  ## Implementation Prompt (For Implementer Agent)
  1. Implement the Redis Redlock inventory reservation mechanism in the relevant checkout and POS endpoints.
  2. Implement/Refine the schema for handling offline POS sync reconciliation with the central PostgreSQL ledger.
  3. Wire the Operations Agent to monitor inventory and notify the business owner of stock-outs or low inventory via push notifications.
  4. Ensure end-to-end functionality across web and mobile viewports (375px), prioritizing graceful UX for "item sold out" scenarios.
  5. Include full Playwright E2E tests validating the hybrid online/offline double-booking prevention flow.

  ## Scope & Priority
  * **Priority:** P1
  * **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
