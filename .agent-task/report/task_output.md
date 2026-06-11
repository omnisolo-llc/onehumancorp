issue_title: "OHC Unified Multi-Channel Inventory Sync & POS"
issue_description: |
  # Research Report: Centralized Inventory & Distributed POS Architecture

  ## Problem Statement
  Priya, a boutique owner, requires seamless inventory tracking between online (web/mobile) and in-store operations (tap-to-pay or card reader). Currently, OHC lacks a real-time, strongly consistent inventory locking and caching mechanism, as well as a robust distributed sync protocol for hybrid merchants. Without this, double-booking and out-of-stock scenarios occur during simultaneous online and offline purchases. If Priya sells the last "Red Dress" in-store using Tap-to-Pay, an online customer browsing at the exact same moment might still be able to add it to their cart and purchase it, leading to a canceled order, an unhappy customer, and lost trust.

  ## Research Report
  **Findings:**
  - **Competitor Systems Audit:** Platforms like Shopify offer extensive POS capabilities but often require costly add-ons or higher-tier plans for true multi-channel real-time sync. Square provides robust POS hardware but lacks integrated AI agents to manage the holistic operations (e.g., automatically drafting a restock order when inventory drops).
  - **Codebase Audit:** OHC currently has basic inventory tables (`products` with `inventory_count`, `locked_quantity`), a `pos_offline_transactions` table for offline resilience, and a `pos_terminal_sessions` table. There are existing Redis-based locks in `InventoryService` (`src/server/services/inventory/service.rs`). However, the end-to-end multi-channel sync (unifying online carts with offline POS tap-to-pay) is incomplete or not exposed seamlessly to the non-technical owner via the mobile UI and the AI Operations Agent.

  ## Design Doc

  ### Architecture Diagram (Mermaid)
  ```mermaid
  erDiagram
      PRODUCT {
          string id
          string tenant_id
          int inventory_count
          int locked_quantity
          int available_quantity
      }
      POS_TERMINAL_SESSION {
          string id
          string tenant_id
          string device_id
          string status
          datetime last_synced_at
          int offline_changes_count
      }
      POS_OFFLINE_TRANSACTION {
          string id
          string tenant_id
          string client_id
          int amount_cents
          string status
      }
      REDIS_LOCK {
          string lock_key
          string session_id
          int ttl_seconds
      }

      PRODUCT ||--o{ REDIS_LOCK : "locks via Redis during checkout"
      POS_TERMINAL_SESSION ||--o{ POS_OFFLINE_TRANSACTION : "generates when offline"
  ```

  ### System Design & Agent Coordination
  - **Data Model:** Central Postgres database for the ultimate truth (`products`, `pos_terminal_sessions`). Redis Redlock (`ohc:lock:{tenant_id}:inventory:{product_id}`) used for distributed short-term locks during transactions (15s for tap-to-pay, 5m for online carts).
  - **AI Coordination:**
    - *Operations Agent ("The Manager"):* Monitors `InventoryUpdated` and `LowStockAlert` events from the event bus (via `department_tasks`). Notifies the owner and suggests a restock action.
    - *Customer Success Agent ("The Ambassador"):* Can alert an online customer if an item in their cart is no longer available due to a recent in-store purchase.
  - **Mobile UX Flow (375px):**
    - The POS UI must operate seamlessly on a 375px width.
    - Touch targets for items and the "Tap to Pay" button must be at least 44x44px.
    - Uses optimistic UI updates for stock levels. If offline, purchases are recorded locally and synced to `pos_offline_transactions` when reconnected.

  ## Implementation Prompt

  **Feature Name:** OHC Unified Multi-Channel Inventory Sync & POS

  **Target Persona:** Priya the Boutique Owner

  **Outcome:** A seamless inventory system where an in-store tap-to-pay purchase instantly reserves and deducts stock, preventing an online customer from double-booking the same item, all managed invisibly by the Operations Agent.

  **Critical User Journey (CUJ) / Acceptance Criteria:**
  1. Priya is logged into the OHC mobile app (POS mode) while an online customer browses her storefront.
  2. Priya processes an in-store sale for the last "Red Dress" using the Tap-to-Pay integration.
  3. The system applies a 15-second Redis Redlock to reserve the item during the transaction.
  4. The online customer attempts to checkout the same "Red Dress" but receives a graceful "Item just sold out" message.
  5. The POS transaction finalizes, the PostgreSQL ledger is updated, and the Operations Agent sends Priya a notification: "Red Dress sold out. Would you like to draft a restock order?"

  **Developer Instructions:**
  - Build out the E2E Playwright test covering this CUJ using real browser interactions and the local database/Redis stack. Ensure no mocking of the backend.
  - Expose the Redis Redlock reservation mechanism appropriately in the frontend POS checkout flow.
  - Ensure the Operations Agent correctly picks up the stock depletion event and surfaces the notification to the owner.
  - Verify all touch targets on the mobile POS view are 44x44px minimum and styling uses OHC Glassmorphism standards.

  **Priority:** P1
  **Estimated Scope:** Large

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []