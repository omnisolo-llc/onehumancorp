issue_title: "Architectural Gap: Centralized Inventory & Distributed POS Lock Mechanism"
issue_description: |
  ## Title
  Centralized Inventory & Distributed POS Lock Mechanism Architecture

  ## Problem Statement
  Small business owners like Priya (Boutique Operator) and Maya (Home Baker) sell goods across multiple channels: online via a web storefront and in-person via a mobile Point-of-Sale (POS) or tap-to-pay. When an item has limited stock (e.g., only 1 custom cake or 1 specific dress size left), simultaneous purchase attempts—one online and one in-store—can lead to overselling (double-booking). Currently, OHC lacks a real-time, distributed inventory reservation and locking system that handles the race condition between the fast in-person tap-to-pay flow and the slower online checkout flow, leading to angry customers and manual refunds.

  ## Research Report
  ### Competitor Analysis
  1. **Shopify**: Handles this well for merchants using their integrated POS and online store, but the sync can sometimes take seconds, leading to edge-case overselling during high-traffic drops.
  2. **Square**: Strong POS, but online integration can be clunky.
  3. **Wix/Squarespace**: Often suffer from inventory sync delays when using third-party POS integrations.

  ### OHC Gap
  OHC needs a unified data layer that guarantees strong consistency for inventory counts, utilizing a distributed lock during the checkout intent phase to reserve stock temporarily before the final payment succeeds, without degrading the sub-second performance expected in the mobile POS.

  ## Design Doc

  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  sequenceDiagram
      participant Customer as Online Buyer
      participant POS as In-Store POS (Owner)
      participant API as OHC API Layer
      participant Redis as Distributed Lock (Redlock)
      participant DB as Postgres (Central Ledger)

      Note over Customer,DB: Scenario: 1 item left in stock

      par Simultaneous Intent
          Customer->>API: Add to Cart & Checkout (Online)
          POS->>API: Tap to Pay (In-Store)
      end

      API->>Redis: Request Lock `ohc:lock:{tenant_id}:inventory:{product_id}`

      alt POS Acquires Lock First
          Redis-->>API: Lock Granted to POS
          API->>DB: Reserve Inventory (Qty: -1)
          API-->>POS: Proceed to Payment

          API->>Redis: Online Request Denied
          API-->>Customer: "Item just sold out in-store!" (Cart updated)
      else Customer Acquires Lock First
          Redis-->>API: Lock Granted to Online
          API->>DB: Reserve Inventory (Qty: -1, TTL: 5m)
          API-->>Customer: Proceed to Payment

          API->>Redis: POS Request Denied
          API-->>POS: "Item currently reserved online." (UI Alert)
      end
  ```

  ### Mobile UX Flow (375px first)
  1. **POS View**: If an item is added to the POS cart and is the last one in stock, but an online buyer is checking out, the POS immediately shows a warning toast: "⚠️ 1 unit reserved online. Payment pending." The checkout button is temporarily disabled.
  2. **Online View**: If the item sells out in-store while the online buyer is on the shipping page, an inline alert appears: "Sorry, this item just sold out at our retail location. We've removed it from your cart."
  3. **Operations Agent**: If overselling somehow occurs (e.g., offline mode sync), the Operations Agent immediately flags it in the Triage Feed: "Oversell detected: 1 Dress. [Refund Customer A] or [Refund Customer B]".

  ### Key Design Decisions
  1. **Redis Redlock for Temporary Reservations**: Postgres handles the permanent source of truth, but Redis is used for fast, TTL-based checkout reservations (e.g., 5 minutes for online, 15 seconds for POS).
  2. **Optimistic POS UI**: The POS client assumes inventory is available to keep the UI fast, but immediately reverts with a clear error if the API reservation fails.

  ## Implementation Prompt
  **For the Implementer Agent:**
  Implement the foundational data model, Redis lock logic, and basic API endpoints for Inventory Reservation.
  1. **Outcome**: The system can successfully lock and reserve a product quantity for a specific checkout session, preventing a simultaneous request from overselling the item.
  2. **CUJ**:
     - System has a product with quantity = 1.
     - API Request A (Online) attempts to reserve the item. Lock succeeds.
     - API Request B (POS) immediately attempts to reserve the same item. Lock fails with an "Out of Stock / Reserved" error.
     - API Request A completes the purchase, permanently decrementing the stock.
  3. **Acceptance Criteria**:
     - Implementation of a Redis-backed locking mechanism (e.g., using a simple SET NX EX pattern or a Redlock crate if available).
     - Strict multi-tenant isolation (`tenant_id`) in all cache keys and DB queries.
     - 100% unit test coverage for the reservation and locking logic, simulating race conditions.
     - Integration tests verifying the DB quantity is correct after successful and failed reservations.

  ## Priority
  P0

  ## Estimated Scope
  Medium
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
