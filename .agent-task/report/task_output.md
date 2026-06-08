issue_title: "Implement Centralized Inventory & Distributed POS Architecture"
issue_description: |
  # Mission Queue Protocol: Centralized Inventory & Distributed POS Architecture

  ## Problem Statement
  Priya, a boutique owner, operates both a physical storefront and an online webstore. Currently, OHC lacks a unified, real-time inventory synchronization mechanism. If Priya sells the last item in-store via a tap-to-pay terminal, an online customer might simultaneously purchase the same item online, leading to double-booking and out-of-stock scenarios. The inventory management needs to be strongly consistent, mobile-first, and invisibly managed by AI agents.

  ## Research Report
  Based on competitive analysis (Shopify, Square, Stripe Terminal), the most robust solution for hybrid merchants involves a central ledger combined with distributed locks for active checkout sessions, and local-first POS clients with eventual consistency for offline support.

  *   **Persona:** Priya (boutique owner).
  *   **Gap:** Lack of real-time inventory locking and caching mechanism, leading to double-booking during simultaneous online and offline sales.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant App as Mobile POS (Priya)
      participant Web as Webstore Client
      participant API as OHC API Layer
      participant Redis as Redis (Redlock)
      participant DB as PostgreSQL (Ledger)
      participant OpsAgent as Operations Agent

      App->>API: Initiate Terminal Checkout (Item X)
      API->>Redis: Acquire Lock (ohc:lock:tenant_id:inventory:item_id)
      Redis-->>API: Lock Granted (15s TTL)
      Web->>API: Attempt Online Checkout (Item X)
      API->>Redis: Attempt Lock (ohc:lock:tenant_id:inventory:item_id)
      Redis-->>API: Lock Denied
      API-->>Web: Graceful "Sold Out" message (Customer Success Agent)
      App->>API: Finalize Terminal Sale
      API->>DB: Update Ledger (Deduct Inventory)
      API->>Redis: Release Lock
      DB-->>OpsAgent: Trigger Low Stock Event
      OpsAgent-->>App: Notification: "Item X sold out. Draft restock?"
  ```

  ### Mobile UX Flow (375px First)
  1.  **POS Interface:** Priya views a cleanly designed POS terminal on her 375px mobile screen. Touch targets for inventory items are large (≥ 44x44px).
  2.  **Checkout:** She taps "Checkout" for an item. The UI optimisticially shows the item as processing.
  3.  **Conflict Handling:** If the item is already locked by an online transaction, the UI gracefully reverts and informs Priya the item is currently in another customer's cart.
  4.  **Completion:** Upon successful tap-to-pay, the item is instantly deducted. The UI updates seamlessly.
  5.  **Agent Notification:** A push notification drops down from the Operations Agent: "Item X is now out of stock. Want me to draft a restock email to the supplier?"

  ### AI Agent Integration Points
  *   **Operations Agent:** Listens to `tenant.inventory.updated` events. If inventory drops to zero, drafts a restock order and notifies the owner.
  *   **Customer Success Agent:** If an online checkout fails due to a POS lock, the agent crafts a polite, helpful message explaining the situation and offering alternatives (e.g., "Would you like to be notified when this is back in stock?").

  ### Key Design Decisions
  *   **Redis Redlock:** Used for distributed locking during the checkout phase to ensure atomic inventory reservations across all channels (online webstore, in-store POS). This prevents double-booking.
  *   **PostgreSQL Central Ledger:** The absolute source of truth for inventory counts. Updates are made only after successful payment or finalized transactions.
  *   **Event-Driven Agent Triggers:** Using the existing event system to decouple inventory updates from the agent logic, ensuring the checkout flow remains fast while still providing proactive AI assistance.

  ## Implementation Prompt

  **Objective:** Implement the core backend infrastructure for unified multi-channel inventory synchronization using Redis distributed locks, and expose the necessary API endpoints for the mobile POS client to reserve and deduct inventory safely.

  **Critical User Journey (CUJ):**
  1. Priya is logged into the OHC mobile app (POS mode). An online customer is browsing her webstore.
  2. Priya initiates an in-store sale for the last unit of "Red Dress".
  3. The backend successfully acquires a Redis lock for the "Red Dress" inventory item.
  4. The online customer attempts to checkout the same "Red Dress". The backend rejects the request because the lock is held, and returns a graceful error.
  5. Priya's POS transaction completes. The backend deducts the inventory in PostgreSQL and releases the Redis lock.
  6. The Operations Agent notices the inventory is now 0 and triggers a notification/draft for restocking.

  **Acceptance Criteria:**
  *   A distributed locking mechanism is implemented using Redis (Redlock pattern) for inventory reservations.
  *   API endpoint(s) for initiating checkout (which acquires the lock) and finalizing checkout (which updates the DB and releases the lock) are created or updated.
  *   The system correctly rejects simultaneous checkout attempts for an item with only 1 unit remaining.
  *   Inventory changes emit the appropriate events (`tenant.inventory.updated`) to trigger the Operations Agent.
  *   Full unit and Playwright E2E tests are implemented to verify the concurrent checkout prevention and the resulting agent behavior.
  *   Zero mock data in the UI; all data flows through the real backend and DB.
  *   The architecture adheres to multi-tenant isolation rules.

  **Priority:** P1
  **Estimated Scope:** Large

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
