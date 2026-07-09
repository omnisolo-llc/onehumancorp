issue_title: "OHC Unified Multi-Channel Inventory Sync & POS"
issue_description: |
  ## Problem Statement
  Solopreneurs and small business owners like Priya (boutique owner) require seamless inventory tracking between online (web/mobile) and in-store operations (tap-to-pay or card reader). Currently, OHC lacks a real-time, strongly consistent inventory locking and caching mechanism, as well as a robust distributed sync protocol for hybrid merchants. Without this, double-booking and out-of-stock scenarios occur during simultaneous online and offline purchases, leading to poor customer experience and operational overhead.

  ## Research Report
  - **Market Context:** Competitors like Shopify dominate the e-commerce space with extensive POS capabilities but often fail micro-SMEs due to complexity. Their inventory management can be disjointed—online inventory frequently falls out-of-sync with in-person sales unless costly third-party integration tools or higher-tier plans are employed.
  - **Hardware vs Software:** Square and Stripe Terminal provide robust POS hardware but lack the integrated, agentic workflow automation needed to unify the business operations effortlessly.
  - **The Gap:** OHC needs a robust, real-time lock management system paired with an eventual consistency offline-sync architecture that gracefully handles intermittent connectivity without sacrificing inventory integrity.

  ## Design Doc
  ### Architecture Diagram (Concept)
  - **Central Ledger:** PostgreSQL (Source of truth for all inventory counts).
  - **Distributed Locks:** Redis Redlock (Temporary reservations).
  - **Client:** Mobile-first POS client (Offline-capable caching).

  ### Data Model & Sync Protocol
  - **Central Ledger (PostgreSQL):** The ultimate source of truth for all inventory counts. We utilize row-level locking or optimistic concurrency control for critical updates.
  - **Distributed Locks (Redis Redlock):** A temporary inventory reservation system applied during the checkout process to prevent double-booking. The lock duration is dynamically tuned (e.g., 5 minutes for online carts vs. 15 seconds for rapid tap-to-pay transactions). Lock key pattern: `ohc:lock:{tenant_id}:inventory:{product_id}`.
  - **Offline/Local First POS Client:** The mobile POS client caches catalog data locally. It employs an eventual consistency mechanism (utilizing PN-Counters for CRDT-like conflict resolution) to sync finalized offline sales and reconcile with the central ledger asynchronously when the network is restored.

  ### AI Agent Coordination
  - **Operations Agent ("The Manager"):** Actively monitors stock levels across all channels. It tracks incoming orders, triggers low-stock alerts, coordinates with the sync mechanism to reconcile conflicts, and suggests restock plans.
  - **Finance Agent ("The Accountant"):** Processes splits for Terminal transactions and correlates POS data with online purchases for unified financial reporting.
  - **Customer Success Agent ("The Ambassador"):** Automatically updates online storefront availability and notifies customers if an item in their cart becomes unavailable due to an in-store purchase.

  ### Mobile UX Flow
  - **Target:** 375px viewport parity.
  - Ensure the POS interface operates flawlessly on mobile. Touch targets for inventory adjustment and checkout must be ≥ 44x44px.
  - Implement optimistic UI updates for inventory changes, with rollback capabilities if the Redis reservation fails.

  ## Implementation Prompt
  **Target Persona:** Priya the Boutique Owner

  **Outcome:** A seamless inventory system where an in-store tap-to-pay purchase instantly reserves and deducts stock, preventing an online customer from double-booking the same item, all managed invisibly by the Operations Agent.

  **Critical User Journey (CUJ) & Acceptance Criteria:**
  1. Login to the OHC mobile app (POS mode) while an online customer browses the storefront.
  2. Process an in-store sale for the last "Red Dress" using the Stripe Terminal integration.
  3. Verify the system applies a 15-second Redis Redlock to reserve the item during the transaction using the `ohc:lock:{tenant_id}:inventory:{product_id}` pattern.
  4. Verify the online customer attempting to checkout the same "Red Dress" receives a graceful "Item just sold out" message, triggered by the Operations Agent.
  5. Finalize the POS transaction, update the PostgreSQL ledger, and ensure the Operations Agent sends Priya a notification: "Red Dress sold out. Would you like to draft a restock order?"

  **Actions:**
  - Implement the Redis Redlock inventory reservation service and integrate it into the checkout flow (both Stripe Terminal and online checkout sessions).
  - Refine the `TerminalSession` data schema to handle offline-sync reconciliation with the PostgreSQL central ledger using PN-Counter logic.
  - Extend the Operations Agent to monitor real-time stock levels, handle sync conflicts, and trigger low-stock push notifications.
  - Ensure zero mock data in the UI and rigorous verification via Playwright E2E tests for the 375px mobile viewport.

  **Note to Implementer:** Do not prescribe specific database schemas or API endpoints here. Focus on the user-facing outcome and the CUJ. Ensure all new code has 100% test coverage and passes all Bazel tests.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []