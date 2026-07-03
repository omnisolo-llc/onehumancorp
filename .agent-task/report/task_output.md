issue_title: "[research] Build Centralized Inventory & Agent-Driven Mobile POS Architecture"
issue_description: |
  ## Problem Statement
  Small business owners (like Priya the Boutique Operator or Carlos the Handyman) struggle with fragmented inventory and sales systems. When they sell an item in person using a basic card reader, their online storefront doesn't automatically update, leading to double-bookings and stockouts. They need a system where an in-person transaction instantly syncs with online inventory, managed invisibly by an AI assistant so they don't have to reconcile databases manually.

  ## Research Report
  Our competitive analysis reveals that legacy platforms (Shopify, Wix) treat Point-of-Sale (POS) and online stores as separate modules requiring complex integration or expensive higher-tier plans. Current AI integrations on these platforms are mostly conversational (chatbots) rather than operational.

  **OHC Differentiation:**
  OHC must provide a unified, agent-driven architecture. The "Operations Agent" should seamlessly bridge the physical and digital divide. When an item is sold in-store, the system must use distributed locks to prevent online double-booking, update the central ledger, and trigger the agent to notify the owner if stock is low—all from a mobile-first (375px) interface.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant MobilePOS as Mobile POS (375px)
      participant API as OHC API Layer
      participant Redis as Redis (Redlock)
      participant DB as PostgreSQL (Central Ledger)
      participant Agent as Operations Agent
      participant Storefront as Online Storefront

      MobilePOS->>API: Initiate In-Store Checkout (Item X)
      API->>Redis: Acquire Lock `ohc:lock:{tenant_id}:inventory:{item_x}` (15s)
      Redis-->>API: Lock Acquired
      API->>DB: Check Availability & Reserve
      DB-->>API: Reserved
      API->>Storefront: Invalidate Edge Cache (Item X unavailable)
      API-->>MobilePOS: Proceed to Payment (Stripe Terminal)
      MobilePOS->>API: Payment Success
      API->>DB: Commit Transaction & Deduct Inventory
      API->>Redis: Release Lock
      API->>Agent: Trigger Event (Inventory Updated)
      Agent->>DB: Check Thresholds
      Agent-->>MobilePOS: Push Notification ("Item X sold out. Reorder?")
  ```

  ### Mobile UX Flow (375px First)
  1. **POS Dashboard:** Clean, UniFi-style card layout showing top-selling items and current cart. Large tap targets (≥ 44x44px).
  2. **Checkout:** Tap-to-pay integration (Stripe Terminal). Optimistic UI updates cart instantly while background lock is acquired.
  3. **Agent Intervention:** If stock hits zero, an unobtrusive "Glassmorphism" toast notification appears: "Item sold out online. Tap to review restock."

  ### AI Agent Integration Points
  - **Operations Agent:** Subscribes to `InventoryDeducted` events. Evaluates remaining stock against historical velocity to propose restock drafts.
  - **Customer Success Agent:** If an online cart is abandoned due to the in-store lock, it can draft a follow-up ("Sorry you missed it, we have similar items...").

  ### Key Design Decisions
  - **Redis Redlock for Inventory:** Crucial for rapid, cross-channel synchronization to prevent the most painful scenario: double-selling a unique item.
  - **Eventual Consistency Offline Mode:** The mobile client must cache the catalog and queue transactions if network drops, reconciling with the central ledger upon reconnection, guided by the Operations Agent if conflicts occur.

  ## Implementation Prompt
  Implement the core backend and frontend services for the Unified Mobile POS and Inventory Sync.

  **Outcome:** A non-technical owner can process an in-person sale on their phone, which instantly reserves the item and prevents an online customer from purchasing the same item simultaneously.

  **Critical User Journey (CUJ):**
  1. Log into the OHC mobile app.
  2. Add a product to the in-store POS cart.
  3. The system applies a temporary distributed lock (simulated or real Redis depending on environment).
  4. The online storefront accurately reflects the item as "In Carts / Reserved".
  5. Complete the POS checkout.
  6. The inventory count is permanently deducted in the central ledger, and the lock is released.
  7. The Operations Agent triggers a low-stock notification if applicable.

  **Acceptance Criteria:**
  - Must function on a 375px viewport.
  - Must implement the Redis locking pattern or equivalent concurrency control for inventory.
  - Include automated Playwright E2E tests covering the simultaneous purchase attempt (online vs in-store).

  **Priority:** P0
  **Estimated Scope:** Large

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
