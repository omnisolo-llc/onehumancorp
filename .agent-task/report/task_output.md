issue_title: "Architectural Gap: Unified Multi-Channel Inventory & Variant Sync (Physical & Service Portfolios)"
issue_description: |
  # Research Report: Unified Multi-Channel Inventory & Variant Sync

  ## Problem Statement
  For physical and boutique business owners like **Priya (boutique owner, 35)**, **Maya (baker, 28)**, and **Fatima (food cart, 50)**, managing inventory across multiple channels (in-store, online storefront, Instagram DMs, pre-orders) is a massive pain point. Currently, OneHumanCorp (OHC) lacks a deep, unified inventory and variant syncing architecture. If Priya sells a blue medium dress in-store via tap-to-pay, the online storefront needs to instantly reflect that the variant is sold out, and Maya needs her custom cake slots to automatically close when she reaches capacity, without manual intervention.

  Without a robust, edge-cached inventory data model, these users face the risk of overselling, leading to manual refunds, bad customer experiences, and lost trust. This capability is critical for scaling physical products, food pre-orders, and even service slots (which function as time-based inventory).

  ## Research Report
  - **Competitor Analysis:**
    - **Shopify:** Utilizes a highly robust `InventoryItem` and `InventoryLevel` architecture. Each variant is tied to an inventory item, which tracks quantities across multiple locations (online, POS, warehouse). They use eventual consistency for high scale but immediate reservation locks during checkout.
    - **Square/Wix:** Integrates catalog management directly with POS hardware and online stores. They rely heavily on real-time webhook events to sync state across mobile apps and web views.
  - **OHC Gap:** While OHC supports basic storefronts and catalogs, the underlying architecture for distributed, multi-tenant inventory locking and real-time syncing across mobile edge (offline-first) and cloud is underdeveloped. We need a system that supports complex variants (Size/Color for Priya) and time-based inventory slots (custom order capacity for Maya) with absolute safety against overselling.

  ## Design Doc

  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  erDiagram
      ORGANIZATION ||--o{ CATALOG_ITEM : owns
      CATALOG_ITEM ||--o{ ITEM_VARIANT : has
      ITEM_VARIANT ||--o{ INVENTORY_LEDGER : tracked_by
      INVENTORY_LEDGER {
          uuid id
          uuid variant_id
          int available_quantity
          int reserved_quantity
          string location_id
      }
      ORDER ||--o{ ORDER_LINE_ITEM : contains
      ORDER_LINE_ITEM }o--|| ITEM_VARIANT : references
      ORDER_LINE_ITEM }o--|| INVENTORY_LEDGER : reserves

      AI_AGENT_OPS ||--o{ INVENTORY_LEDGER : monitors
  ```

  ```mermaid
  sequenceDiagram
      participant Customer
      participant EdgeCache
      participant OHC_Core
      participant InventoryLedger
      participant AI_OpsAgent

      Customer->>EdgeCache: Add to Cart (Variant X)
      EdgeCache->>OHC_Core: Request Reservation
      OHC_Core->>InventoryLedger: Lock 1 Unit (Variant X)
      alt Unit Available
          InventoryLedger-->>OHC_Core: Success
          OHC_Core-->>EdgeCache: Reservation Confirmed
          EdgeCache-->>Customer: Proceed to Checkout
      else Sold Out
          InventoryLedger-->>OHC_Core: Error: Insufficient Stock
          OHC_Core-->>EdgeCache: Out of Stock
          EdgeCache-->>Customer: Item Unavailable
          OHC_Core->>AI_OpsAgent: Trigger Reorder/Waitlist Protocol
      end
  ```

  ### Mobile UX Flow (375px First)
  - **Inventory Management Dashboard:** Clean, modular cards showing "Low Stock", "Sold Out", and "Active Listings".
  - **Variant Editor:** Simple, tap-friendly toggles to add variants (e.g., Size, Color) with unified stock numbers.
  - **Real-Time Sync Alerts:** Non-intrusive push notifications ("Blue Medium Dress sold out online").

  ### AI Agent Integration Points
  - **Operations Agent (AI_OpsAgent):** Monitors the `INVENTORY_LEDGER`. When stock hits a threshold, it automatically drafts a reorder email to the supplier or asks the user via a quick mobile prompt: "Stock for Blue Medium Dress is low. Send reorder to vendor?"
  - **Customer Service Agent:** If an item oversells (edge case), the CS Agent automatically emails the customer apologizing, processes the refund via the Finance Agent, and offers a 10% discount code.

  ### Key Design Decisions
  1. **Ledger-Based Inventory:** Instead of simple integer counters, use a ledger system (`available_quantity`, `reserved_quantity`) to safely handle concurrent checkout requests and prevent overselling.
  2. **Unified Data Model:** Treat physical items (dresses) and time-based slots (cake pre-orders) using the same underlying ledger logic to simplify the architecture.
  3. **Zero Trust & Security:** Strict tenant isolation (`organization_id`) at the database row level for all ledger transactions.

  ## Implementation Prompt
  **Outcome:** Implement the core Ledger-Based Inventory and Variant Sync architecture to support physical and time-based products.
  **CUJ:** Priya adds a new dress with 3 sizes. She sells one in-store via POS. The online store instantly updates to reflect the new available quantity.
  **Acceptance Criteria:**
  - Create the multi-tenant data model for Variants and Inventory Ledgers.
  - Implement the reservation locking mechanism during checkout.
  - Ensure the AI Operations Agent can read ledger thresholds and trigger mock reorder notifications.
  - Verify complete data isolation between organizations.
  - Deliver a mobile-first (375px) UI component for viewing inventory status.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, architecture]
assignees: []