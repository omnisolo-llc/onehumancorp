issue_title: "Implement Centralized Inventory & Distributed POS Architecture"
issue_description: |
  # Research Report: Centralized Inventory & Distributed POS Architecture

  ## Problem Statement
  Currently, OHC lacks a real-time, strongly consistent inventory locking and caching mechanism, as well as a robust distributed sync protocol for hybrid merchants. Without this, double-booking and out-of-stock scenarios occur during simultaneous online and offline purchases. Our merchants, like Priya (boutique owner), require seamless inventory tracking between online (web/mobile) and in-store operations (tap-to-pay or card reader).

  ## Research Report & Gap Analysis
  Competitors like Shopify dominate the e-commerce space with extensive POS capabilities but often fail micro-SMEs due to complexity. Their inventory management can be disjointed—online inventory frequently falls out-of-sync with in-person sales unless costly third-party integration tools or higher-tier plans are employed. Square and Stripe Terminal provide robust POS hardware but lack the integrated, agentic workflow automation needed to unify the business operations effortlessly.

  The gap in OHC is the lack of real-time inventory locking (Redis Redlock) and seamless distributed POS sync with the PostgreSQL central ledger. This architecture gap restricts OHC's ability to act as the single source of truth for omnichannel merchants.

  ## Design Doc
  ### Data Model & Sync Protocol
  - **Central Ledger (PostgreSQL):** The ultimate source of truth for all inventory counts. Utilize row-level locking or optimistic concurrency control for critical updates.
  - **Distributed Locks (Redis Redlock):** A temporary inventory reservation system applied during the checkout process to prevent double-booking. The lock duration is dynamically tuned (e.g., 5 minutes for online carts vs. 15 seconds for rapid tap-to-pay transactions). Lock key pattern: `ohc:lock:{tenant_id}:inventory:{product_id}`.
  - **Offline/Local First POS Client:** The mobile POS client caches catalog data locally. It employs an eventual consistency mechanism to sync finalized offline sales and reconcile with the central ledger asynchronously when the network is restored.

  ### AI Agent Coordination
  - **Operations Agent ("The Manager"):** Actively monitors stock levels across all channels. It tracks incoming orders, triggers low-stock alerts, coordinates with the sync mechanism to reconcile conflicts, and suggests restock plans.
  - **Finance Agent ("The Accountant"):** Processes splits for Terminal transactions and correlates POS data with online purchases for unified financial reporting.
  - **Customer Success Agent ("The Ambassador"):** Automatically updates online storefront availability and notifies customers if an item in their cart becomes unavailable due to an in-store purchase.

  ### Mobile-First Implementation
  - Ensure the POS interface operates flawlessly on a 375px viewport. Touch targets for inventory adjustment and checkout must be ≥ 44x44px.
  - Implement optimistic UI updates for inventory changes, with rollback capabilities if the Redis reservation fails.
  - Adhere to the OHC Premium Translucent Glass styling for UI elements.

  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  graph TD;
      POS[Mobile POS App - Offline Capable] -->|Sync| Server[OHC Backend Server];
      Web[Web Storefront] -->|Checkout| Server;
      Server -->|Inventory Reservation| Redis[Redis Redlock];
      Server -->|Final Commit| DB[(PostgreSQL Ledger)];
      Redis -.->|Lock Expiry| Server;
      Server -->|Notify| OpsAgent[Operations Agent];
      Server -->|Notify| CSAgent[Customer Success Agent];
      OpsAgent -->|Low Stock Alert| Dashboard[Owner Dashboard];
      CSAgent -->|Cart Update| Web;
  ```

  ## Implementation Prompt
  Implement the Unified Multi-Channel Inventory Sync & POS architecture.

  **Target Persona:** Priya the Boutique Owner
  **Outcome:** A seamless inventory system where an in-store tap-to-pay purchase instantly reserves and deducts stock, preventing an online customer from double-booking the same item, all managed invisibly by the Operations Agent.

  **Critical User Journey (CUJ):**
  1. Priya is logged into the OHC mobile app (POS mode) while an online customer browses her storefront.
  2. Priya processes an in-store sale for the last "Red Dress" using the Stripe Terminal integration.
  3. The system applies a 15-second Redis Redlock to reserve the item during the transaction.
  4. The online customer attempts to checkout the same "Red Dress" but receives a graceful "Item just sold out" message, triggered by the Operations Agent.
  5. The POS transaction finalizes, the PostgreSQL ledger is updated, and the Operations Agent sends Priya a notification: "Red Dress sold out. Would you like to draft a restock order?"

  **Acceptance Criteria:**
  - Implement Redis Redlock inventory reservation service and integrate it into the checkout and POS flows.
  - Create the `TerminalSession` data schema to handle offline-sync reconciliation with the PostgreSQL central ledger.
  - Ensure 100% unit test coverage for new backend services.
  - Add E2E Playwright tests verifying the concurrent booking scenario and the UI updates on a 375px viewport.
  - Follow the Translucent Glass UI guidelines for any mobile POS screens.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
