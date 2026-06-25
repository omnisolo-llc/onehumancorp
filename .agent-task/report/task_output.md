issue_title: "Implement Centralized Inventory & Distributed POS Architecture"
issue_description: |
  # Research Report: Centralized Inventory & Distributed POS Architecture

  ## Executive Summary
  This report investigates the current landscape of small business inventory management, specifically addressing the pain points of multi-channel (online + in-store) merchants. The objective is to design a centralized inventory and distributed Point-of-Sale (POS) synchronization architecture for OneHumanCorp (OHC) that leverages our AI agents to provide a seamless, real-time experience for non-technical users.

  ## 1. Market Mapping & Competitor Discovery (Track 1)
  Competitors like Shopify dominate the e-commerce space with extensive POS capabilities but often fail micro-SMEs due to complexity. Their inventory management can be disjointed—online inventory frequently falls out-of-sync with in-person sales unless costly third-party integration tools or higher-tier plans are employed. Square and Stripe Terminal provide robust POS hardware but lack the integrated, agentic workflow automation needed to unify the business operations effortlessly.

  ## 2. OHC Gap & Pain Point Identification (Track 3)
  - **Persona Focus:** Priya (boutique owner) requires seamless inventory tracking between online (web/mobile) and in-store operations (tap-to-pay or card reader).
  - **The Gap:** Currently, OHC lacks a real-time, strongly consistent inventory locking and caching mechanism, as well as a robust distributed sync protocol for hybrid merchants. Without this, double-booking and out-of-stock scenarios occur during simultaneous online and offline purchases.

  ## 3. Deep Dive Architecture Design (Track 2 & Track 3)

  ### Data Model & Sync Protocol
  - **Central Ledger (PostgreSQL):** The ultimate source of truth for all inventory counts. We utilize row-level locking or optimistic concurrency control for critical updates.
  - **Distributed Locks (Redis Redlock):** A temporary inventory reservation system applied during the checkout process to prevent double-booking. The lock duration is dynamically tuned (e.g., 5 minutes for online carts vs. 15 seconds for rapid tap-to-pay transactions). Lock key pattern: `ohc:lock:{tenant_id}:inventory:{product_id}`.
  - **Offline/Local First POS Client:** The mobile POS client caches catalog data locally. It employs an eventual consistency mechanism to sync finalized offline sales and reconcile with the central ledger asynchronously when the network is restored.

  ```mermaid
  erDiagram
    TENANT ||--o{ PRODUCT : owns
    TENANT ||--o{ TERMINAL_SESSION : manages
    PRODUCT ||--o{ INVENTORY_LEDGER : tracks
    TERMINAL_SESSION ||--o{ TRANSACTION : processes
    PRODUCT {
      uuid id PK
      uuid tenant_id FK
      string name
    }
    INVENTORY_LEDGER {
      uuid id PK
      uuid product_id FK
      int current_count
      int reserved_count
      timestamp last_updated
    }
    TERMINAL_SESSION {
      uuid id PK
      uuid tenant_id FK
      string status
      timestamp started_at
    }
    TRANSACTION {
      uuid id PK
      uuid session_id FK
      string status
      float amount
    }
  ```

  ```mermaid
  sequenceDiagram
    participant C as Customer (Online)
    participant POS as POS Client (In-store)
    participant Redis as Redis (Redlock)
    participant DB as PostgreSQL Ledger
    participant OA as Operations Agent

    C->>Redis: Attempt to reserve 'Red Dress' (Online Cart)
    POS->>Redis: Attempt to reserve 'Red Dress' (Tap-to-pay)
    Note over Redis: POS gets lock first (15s duration)
    Redis-->>POS: Lock Acquired
    Redis-->>C: Lock Failed (Item Sold Out)
    OA->>C: Notify "Item just sold out"
    POS->>DB: Finalize Sale, Deduct Inventory
    DB-->>POS: Sale Confirmed
    POS->>Redis: Release Lock
    OA->>POS: Notify "Red Dress sold out. Draft restock order?"
  ```

  ### AI Agent Coordination
  - **Operations Agent ("The Manager"):** Actively monitors stock levels across all channels. It tracks incoming orders, triggers low-stock alerts, coordinates with the sync mechanism to reconcile conflicts, and suggests restock plans.
  - **Finance Agent ("The Accountant"):** Processes splits for Terminal transactions and correlates POS data with online purchases for unified financial reporting.
  - **Customer Success Agent ("The Ambassador"):** Automatically updates online storefront availability and notifies customers if an item in their cart becomes unavailable due to an in-store purchase.

  ### Mobile-First Implementation
  - **Mobile UX Flow (375px):**
    1.  **Dashboard Screen:** Uses a translucent glass header (`backdrop-filter: blur(30px)`) to display the current connection status to the Terminal.
    2.  **Catalog/Inventory Screen:** Uses UniFi-style card layouts. Each product card has a prominent, easily tappable (≥ 44x44px) "+ / -" counter to adjust stock directly. Changes sync optimistically.
    3.  **Checkout Flow:** A modal slides up smoothly from the bottom. Large, full-width primary action button ("Charge $X.XX") initiates the tap-to-pay sequence.
    4.  **Pending State:** While acquiring the Redis lock, the checkout button transitions to a spinner, disabling further taps.
  - Implement optimistic UI updates for inventory changes, with rollback capabilities if the Redis reservation fails.

  ## 4. Proposed Implementation Steps & Issue Prompt

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
