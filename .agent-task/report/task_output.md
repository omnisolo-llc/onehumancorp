issue_title: "[Architecture] Omnichannel Tap-to-Pay and Inventory Sync Engine"
issue_description: |
  # Research Report: Omnichannel Tap-to-Pay and Inventory Sync Engine

  ## Problem Statement
  Small business owners like Priya (boutique owner) and Fatima (food cart) sell products both in-person and online. Currently, the local terminal sessions (tap-to-pay) and global multi-tenant inventory ledger are disconnected. When a product is sold offline, the online store isn't immediately updated, leading to overselling and inventory confusion.

  They need an invisible, real-time inventory synchronization engine that instantly updates the online catalog whenever a local tap-to-pay or offline POS event occurs, preventing double-selling without manual intervention.

  ## Research Report
  **Competitor Analysis:**
  - **Shopify:** Provides Shopify POS that handles omnichannel inventory relatively well, but requires small businesses to buy into their entire hardware and app ecosystem. Setting up seamless tap-to-pay without expensive third-party plugins is complex.
  - **Square:** Dominates offline POS with excellent hardware and tap-to-pay support. However, their online e-commerce builder is rigid and lacks the flexible agentic workflow automation OHC provides.
  - **Wix:** Offers Wix Owner POS but offline/omnichannel sync capabilities are limited, and it suffers from the "two system problem" where online and offline inventory states frequently clash.

  **Market Needs:**
  Merchants expect inventory to simply be accurate, regardless of the sales channel. A non-technical owner like Priya cannot be expected to manage manual inventory reconciliations. OHC can differentiate by natively unifying local tap-to-pay events with the global inventory cache, using background AI agents to resolve state conflicts seamlessly.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD;
      subgraph Mobile POS Client
          App[OHC Mobile App 375px] --> LocalDB[(Local Cache)];
          App --> TapToPay[Native Tap-to-Pay SDK];
      end

      App -- Event: Local Sale --> Gateway[OHC API Gateway];

      subgraph Cloud Infrastructure
          Gateway --> EventQueue[AI Job Queue / Event Bus];
          Gateway --> LockSystem[Redis Redlock: ohc:lock:{tenant_id}:inventory:{product_id}];
          EventQueue --> MainDB[(Cloud Postgres Ledger)];
      end

      subgraph Agent Swarm
          EventQueue --> OpsAgent[Operations Agent];
          OpsAgent --> OnlineStore[Online Storefront Cache];
          OpsAgent -- Notify --> Dashboard[Owner Notification System];
      end
  ```

  ### Mobile UX Flow (375px First)
  1. **POS Dashboard:** Priya opens the OHC mobile app. The primary action is a macOS-style Translucent Glass "Charge" button.
  2. **Cart Building:** She selects items from her visual inventory.
  3. **Payment Execution:** She taps "Charge" and selects "Tap to Pay". The native OS Tap-to-Pay UI takes over.
  4. **Instant Sync UI:** Upon successful payment, the app immediately updates the local inventory count optimistically and displays a subtle confirmation toast.
  5. **Conflict Handling:** If the item was simultaneously sold online, the app alerts Priya *before* the charge is initiated, stating: "This item was just purchased online. Inventory is depleted."

  ### AI Agent Integration Points
  - **Operations Agent ("The Manager"):** Listens to webhook/sync events from the tap-to-pay module. It dynamically adjusts global stock levels, resolves edge-case sync conflicts asynchronously, and updates the online storefront cache to prevent double-booking.
  - **Finance Agent:** Correlates POS terminal session data with online purchases for unified financial reporting.
  - **Customer Success Agent:** If an online customer's cart contains an item that was just sold out via tap-to-pay, this agent automatically updates their cart state with a graceful explanation message.

  ### Key Design Decisions
  - **Redis Redlock:** Employed to create short-lived (e.g., 15-second) inventory reservations during checkout flows to prevent race conditions between simultaneous online and offline purchases.
  - **Optimistic UI Updates:** The mobile POS client updates its local state immediately for a snappy user experience, while the actual global ledger update occurs asynchronously.
  - **Event-Driven Architecture:** Tap-to-pay transactions emit events to a central queue (PostgreSQL SKIP LOCKED or similar) rather than blocking HTTP calls to update the database, ensuring resilience against network flakiness.

  ## Implementation Prompt
  **Goal:** Implement the backend architecture to synchronize inventory across online storefronts and mobile tap-to-pay POS sessions in real-time.

  **Critical User Journey (CUJ):**
  1. An online customer has the last "Red Dress" in their checkout flow.
  2. Simultaneously, the merchant processes an in-store tap-to-pay sale for the same "Red Dress" using the mobile app.
  3. The system must use a distributed locking mechanism (e.g., Redis Redlock) to ensure only one transaction succeeds.
  4. Once the tap-to-pay transaction completes, the online storefront inventory must instantly update to reflect the sold-out status.
  5. The merchant receives a confirmation of the sale and an automated prompt suggesting a restock order if inventory drops below a threshold.

  **Acceptance Criteria:**
  - Implement a distributed locking service for inventory items (`ohc:lock:{tenant_id}:inventory:{product_id}`).
  - Create the API endpoints and event handlers required to process tap-to-pay completion events.
  - Integrate the Operations Agent to listen for these events, update the central inventory ledger, and push state changes to connected online clients.
  - Ensure the solution is multi-tenant safe and performs reliably under concurrent load.
  - Provide complete E2E Playwright test coverage verifying the conflict resolution between simultaneous online and offline purchase attempts.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
