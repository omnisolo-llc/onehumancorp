issue_title: "Implement Multi-Channel Inventory Sync & POS"
issue_description: |
  **Mission Queue Protocol Report**

  **1. Problem Statement**
  Micro-SMEs like Priya (boutique owner) require seamless inventory tracking between online (web/mobile) and in-store operations (tap-to-pay or card reader). Currently, OHC lacks a real-time, strongly consistent inventory locking and caching mechanism, as well as a robust distributed sync protocol for hybrid merchants. Without this, double-booking and out-of-stock scenarios occur during simultaneous online and offline purchases.

  **2. Research Report**
  - **Persona Focus:** Priya (boutique owner)
  - Competitors like Shopify dominate the e-commerce space with extensive POS capabilities but often fail micro-SMEs due to complexity. Their inventory management can be disjointed—online inventory frequently falls out-of-sync with in-person sales unless costly third-party integration tools or higher-tier plans are employed. Square and Stripe Terminal provide robust POS hardware but lack the integrated, agentic workflow automation needed to unify the business operations effortlessly.

  **3. Design Doc**
  - **Architecture Diagram (Mermaid.js):**
    ```mermaid
    graph TD
      A[Mobile POS / In-Store] -->|Reserve (Redis Redlock)| B(Central Ledger)
      C[Online Customer] -->|Check Availability| B
      B -->|Sync| D[PostgreSQL]
      B -->|Trigger Alert| E(Operations Agent)
      E -->|Notify| F(Priya)
    ```
  - **Mobile UX Flow (375px first):**
    - The POS interface must operate flawlessly on a 375px viewport. Touch targets for inventory adjustment and checkout must be ≥ 44x44px.
    - Implement optimistic UI updates for inventory changes, with rollback capabilities if the Redis reservation fails.
  - **AI Agent Integration Points:**
    - **Operations Agent ("The Manager"):** Actively monitors stock levels across all channels. It tracks incoming orders, triggers low-stock alerts, coordinates with the sync mechanism to reconcile conflicts, and suggests restock plans.
    - **Finance Agent ("The Accountant"):** Processes splits for Terminal transactions and correlates POS data with online purchases for unified financial reporting.
    - **Customer Success Agent ("The Ambassador"):** Automatically updates online storefront availability and notifies customers if an item in their cart becomes unavailable due to an in-store purchase.

  **4. Implementation Prompt**
  - **User-Facing Outcome:** A seamless inventory system where an in-store tap-to-pay purchase instantly reserves and deducts stock, preventing an online customer from double-booking the same item, all managed invisibly by the Operations Agent.
  - **CUJ:**
    1. Priya is logged into the OHC mobile app (POS mode) while an online customer browses her storefront.
    2. Priya processes an in-store sale for the last "Red Dress" using the Stripe Terminal integration.
    3. The system applies a 15-second Redis Redlock to reserve the item during the transaction.
    4. The online customer attempts to checkout the same "Red Dress" but receives a graceful "Item just sold out" message, triggered by the Operations Agent.
    5. The POS transaction finalizes, the PostgreSQL ledger is updated, and the Operations Agent sends Priya a notification: "Red Dress sold out. Would you like to draft a restock order?"
  - **Acceptance Criteria:**
    - Implement the Redis Redlock inventory reservation service and integrate it into the checkout flow.
    - Refine the `TerminalSession` data schema to handle offline-sync reconciliation with the PostgreSQL central ledger.
    - Extend the Operations Agent to monitor real-time stock levels, handle sync conflicts, and trigger low-stock push notifications.

  **Priority:** P1
  **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
