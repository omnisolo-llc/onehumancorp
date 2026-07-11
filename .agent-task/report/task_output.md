issue_title: "Architectural Gap: Unified Multi-Channel Inventory Sync & POS"
issue_description: |
  ## Mission Queue Protocol

  ### 1. Problem Statement
  Priya (boutique owner) requires seamless inventory tracking between online (web/mobile) and in-store operations (tap-to-pay or card reader). Currently, OHC lacks a real-time, strongly consistent inventory locking and caching mechanism, as well as a robust distributed sync protocol for hybrid merchants. Without this, double-booking and out-of-stock scenarios occur during simultaneous online and offline purchases.

  ### 2. Research Report
  - **Market Mapping & Competitor Discovery (Track 1):** Competitors like Shopify dominate the e-commerce space with extensive POS capabilities but often fail micro-SMEs due to complexity. Their inventory management can be disjointed—online inventory frequently falls out-of-sync with in-person sales unless costly third-party integration tools or higher-tier plans are employed. Square and Stripe Terminal provide robust POS hardware but lack the integrated, agentic workflow automation needed to unify the business operations effortlessly.

  ### 3. Design Doc (Track 2 & Track 3)

  **Architecture Diagram**
  ```mermaid
  sequenceDiagram
      participant Customer as Online Customer
      participant Storefront as OHC Online Storefront
      participant Redis as Redis (Distributed Lock)
      participant Priya as Priya (Mobile POS UI)
      participant SyncManager as Background Sync Queue
      participant API as OHC API Gateway
      participant OpsAgent as Operations Agent
      participant DB as Postgres (Central Ledger)

      Priya->>Priya: Tap-to-Pay for "Red Dress"
      Priya->>API: Reserve Inventory (POS)
      API->>Redis: Acquire Redlock (15s)
      Redis-->>API: Lock Acquired
      API-->>Priya: Reservation Success

      Customer->>Storefront: Attempt to checkout "Red Dress"
      Storefront->>API: Checkout Request
      API->>Redis: Check Reservation
      Redis-->>API: Item Locked
      API->>OpsAgent: Trigger Conflict Resolution
      OpsAgent-->>Storefront: Push "Item just sold out" grace message

      Priya->>API: Finalize POS Transaction
      API->>Redis: Release Lock
      API->>DB: Update Central Ledger
      API->>OpsAgent: Evaluate Stock Level
      OpsAgent-->>Priya: Push Notification: "Restock needed"
  ```

  **Data Model & Sync Protocol**
  - **Central Ledger (PostgreSQL):** The ultimate source of truth for all inventory counts. We utilize row-level locking or optimistic concurrency control for critical updates.
  - **Distributed Locks (Redis Redlock):** A temporary inventory reservation system applied during the checkout process to prevent double-booking. The lock duration is dynamically tuned (e.g., 5 minutes for online carts vs. 15 seconds for rapid tap-to-pay transactions). Lock key pattern: `ohc:lock:{tenant_id}:inventory:{product_id}`.
  - **Offline/Local First POS Client:** The mobile POS client caches catalog data locally. It employs an eventual consistency mechanism to sync finalized offline sales and reconcile with the central ledger asynchronously when the network is restored.

  **AI Agent Coordination**
  - **Operations Agent ("The Manager"):** Actively monitors stock levels across all channels. It tracks incoming orders, triggers low-stock alerts, coordinates with the sync mechanism to reconcile conflicts, and suggests restock plans.
  - **Finance Agent ("The Accountant"):** Processes splits for Terminal transactions and correlates POS data with online purchases for unified financial reporting.
  - **Customer Success Agent ("The Ambassador"):** Automatically updates online storefront availability and notifies customers if an item in their cart becomes unavailable due to an in-store purchase.

  **Mobile-First Implementation (375px) & Wireframes**
  - **Wireframe Flow:**
    - *Screen 1 (POS Catalog):* Clean, translucent grid of product images. Touch targets are generous (≥ 44x44px). The owner taps the "Red Dress".
    - *Screen 2 (Checkout):* Quick summary view with a large "Tap to Pay" CTA. The UI shows an optimistic "Reserving..." state briefly.
    - *Screen 3 (Success/Agent Alert):* Success animation, followed immediately by an Operations Agent notification card at the top of the screen: "Red Dress sold out. Would you like to draft a restock order?"
  - Ensure the POS interface operates flawlessly on a 375px viewport. Touch targets for inventory adjustment and checkout must be ≥ 44x44px.
  - Implement optimistic UI updates for inventory changes, with rollback capabilities if the Redis reservation fails.

  ### 4. Implementation Prompt

  **Feature Name:** OHC Unified Multi-Channel Inventory Sync & POS

  **Target Persona:** Priya the Boutique Owner

  **Outcome:** A seamless inventory system where an in-store tap-to-pay purchase instantly reserves and deducts stock, preventing an online customer from double-booking the same item, all managed invisibly by the Operations Agent.

  **Critical User Journey (CUJ):**
  1. Priya is logged into the OHC mobile app (POS mode) while an online customer browses her storefront.
  2. Priya processes an in-store sale for the last "Red Dress" using the Stripe Terminal integration.
  3. The system applies a 15-second Redis Redlock to reserve the item during the transaction.
  4. The online customer attempts to checkout the same "Red Dress" but receives a graceful "Item just sold out" message, triggered by the Operations Agent.
  5. The POS transaction finalizes, the PostgreSQL ledger is updated, and the Operations Agent sends Priya a notification: "Red Dress sold out. Would you like to draft a restock order?"

  **Next Actions for Engineering:**
  - **Step 1:** Implement the Redis Redlock inventory reservation service and integrate it into the checkout flow.
  - **Step 2:** Refine the `TerminalSession` data schema to handle offline-sync reconciliation with the PostgreSQL central ledger.
  - **Step 3:** Extend the Operations Agent to monitor real-time stock levels, handle sync conflicts, and trigger low-stock push notifications.

  **Priority:** P1
  **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
