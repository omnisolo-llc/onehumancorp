issue_title: "Architecture Gap: Centralized Inventory & Distributed POS Sync"
issue_description: |
  # Research Report: Unified Offline-First Tap-to-Pay & POS Architecture

  ## Problem Statement
  For non-technical business owners running physical stores and online shops simultaneously, maintaining real-time inventory synchronization is a critical pain point. Currently, OneHumanCorp (OHC) lacks a real-time, strongly consistent inventory locking and caching mechanism, as well as a robust distributed sync protocol for hybrid merchants.

  This architectural gap directly impacts our target persona, **Priya (Boutique Operator)**. Priya requires seamless inventory tracking between online (web/mobile) and in-store operations (tap-to-pay or card reader). Without this synchronization, Priya faces severe double-booking and out-of-stock scenarios when simultaneous online and offline purchases occur. Operating the current OHC app flow proves that in-store point-of-sale actions are disconnected from online reservations, meaning the product cannot support a true omni-channel operation safely.

  ## Research Report & Competitor Analysis
  - **Shopify POS:** Offers extensive POS capabilities and omni-channel syncing. However, their inventory management often becomes disjointed, and offline-to-online sync requires costly third-party tools or higher-tier plans for micro-SMEs.
  - **Square & Stripe Terminal:** Provide robust, hardware-backed offline capabilities and rapid tap-to-pay checkout but lack the integrated, agentic workflow automation necessary to unify the business operations effortlessly (like intelligent restock notifications).
  - **OHC Differentiator:** OHC must close this gap by introducing an invisible, autonomous synchronization architecture that handles the complexity of distributed caching and eventual consistency for offline sales without any technical setup from the user.

  ## Design Doc

  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  sequenceDiagram
      participant App as POS Client (Priya)
      participant API as OHC API
      participant Redis as Redis (Redlock)
      participant DB as PostgreSQL (Ledger)
      participant Agent as Operations Agent
      participant Web as Online Customer

      App->>API: Process in-store sale ("Red Dress")
      API->>Redis: Acquire 15s lock `ohc:lock:{tenant_id}:inventory:{product_id}`
      Redis-->>API: Lock acquired
      Web->>API: Attempt checkout ("Red Dress")
      API->>Redis: Check lock
      Redis-->>API: Locked
      API-->>Web: Graceful error: "Item just sold out"
      API->>DB: Finalize POS sale & deduct stock
      API->>Redis: Release lock
      API->>Agent: Event: "Red Dress" stock depleted
      Agent-->>App: Push Notification: "Red Dress sold out. Draft restock order?"
  ```

  ### Data Model & Sync Protocol
  - **Central Ledger (PostgreSQL):** The ultimate source of truth. Utilize row-level locking or optimistic concurrency control for critical updates.
  - **Distributed Locks (Redis Redlock):** A temporary inventory reservation system. Lock duration is dynamically tuned (e.g., 5 minutes for online carts vs. 15 seconds for rapid tap-to-pay).
  - **Offline-First POS Client:** The mobile POS client caches catalog data locally and employs an eventual consistency mechanism to sync finalized offline sales and reconcile with the central ledger asynchronously when the network is restored.

  ### Mobile UX Flow (375px First)
  1. Priya is on the mobile POS screen (375px viewport). She taps the 44x44px target for "Red Dress" and proceeds to Tap-to-Pay.
  2. The UI optimistically updates the inventory to reflect the pending sale, locking the item via Redis.
  3. If the sale completes, the state commits.
  4. If the Redis reservation fails (e.g., online customer beat her by milliseconds), the UI rolls back gracefully with a clear error: "Item reserved online".

  ### AI Agent Integration
  - **Operations Agent:** Monitors stock levels across channels, triggers low-stock alerts, and handles sync conflicts invisibly.
  - **Customer Success Agent:** Updates online storefront availability instantly and notifies customers of out-of-stock items.
  - **Finance Agent:** Reconciles the Terminal transaction data with the unified ledger.

  ## Implementation Prompt
  **Role:** Implementer Agent

  **Objective:** Implement the backend synchronization and Redis Redlock reservation system to support Priya's omni-channel boutique.

  **Critical User Journey (CUJ) & Acceptance Criteria:**
  1. Implement a Redis Redlock inventory reservation service and integrate it into both the online checkout and the new in-store POS checkout API flows.
  2. The lock key pattern MUST be `ohc:lock:{tenant_id}:inventory:{product_id}`.
  3. Ensure that when a 15-second reservation is active for a POS transaction, an incoming online checkout request for the same last remaining item gracefully fails with an informative message.
  4. Refine the data schema (e.g., `TerminalSession` or similar) to ensure finalized offline sales sync securely to the PostgreSQL central ledger.
  5. Extend the Operations Agent to trigger a low-stock event and notification when the final item is successfully sold.
  6. **UI Requirement:** All new POS interfaces must function fully on a 375px mobile screen with minimum 44x44px touch targets. ZERO mock data should be used; ensure testing is done against the real Redis and DB local stack.

  **Priority:** P1
  **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
