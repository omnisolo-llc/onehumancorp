issue_title: "OHC Unified Multi-Channel Inventory Sync & Distributed POS"
issue_description: |
  # Research Report & Design Doc: Centralized Inventory & Distributed POS Architecture

  ## Problem Statement
  Service-based small business owners and boutique operators (like Priya) require a seamless, centralized inventory system to manage stock across both online (web/mobile) and in-store operations (tap-to-pay/Stripe Terminal). Currently, OHC lacks a real-time, strongly consistent inventory locking and caching mechanism, leading to double-booking and out-of-stock scenarios during simultaneous online and offline purchases. SMBs cannot afford complex, enterprise-grade multi-channel systems, so OHC must handle this orchestration autonomously.

  ## Research Report
  - **Market Context:** Traditional platforms like Shopify require expensive apps or higher-tier plans to synchronize offline POS with online inventory effectively. Square is strong on POS but weak on agentic workflows.
  - **The OHC Differentiator:** The OHC platform must provide this synchronization invisibly. An in-store tap-to-pay purchase should instantly reserve and deduct stock across all channels, seamlessly preventing online customers from double-booking. This eliminates the "app tax" and provides an enterprise-grade solution out of the box.

  ## Design Doc
  ### Architecture Diagram (Conceptual)
  ```mermaid
  graph TD
    A[Online Storefront] -->|Checkout Request| C(Distributed Lock - Redis Redlock)
    B[Mobile POS / Terminal] -->|In-store Sale| C
    C -->|Acquire Lock: ohc:lock:tenant_id:inventory:product_id| D[(Central Ledger - PostgreSQL)]
    D -->|Update Inventory| E[Operations Agent]
    E -->|Alert/Draft Re-order| F[Mobile Owner App]
    E -->|Update Storefront| A
  ```

  ### Mobile UX Flow (375px)
  1. The owner is logged into the OHC mobile app (POS mode).
  2. The owner processes an in-store sale using the Stripe Terminal integration.
  3. Optimistic UI instantly updates the POS view to reflect the sale.
  4. Concurrently, an online customer attempting to purchase the same item receives a graceful "Item just sold out" message.
  5. The owner receives an unobtrusive agent card notification: "Item sold out. Would you like to draft a restock order?"

  ### AI Agent Integration
  - **Operations Agent ("The Manager"):** Actively monitors stock levels, tracks incoming POS orders, coordinates with Redis locks to reconcile conflicts, and triggers low-stock alerts.
  - **Customer Success Agent ("The Ambassador"):** Updates the online storefront availability instantly.

  ### Key Design Decisions
  - **Central Ledger:** PostgreSQL serves as the absolute source of truth, utilizing row-level locking or optimistic concurrency for updates.
  - **Distributed Locks:** Redis Redlock temporarily reserves inventory (e.g., 5 mins for online carts, 15 seconds for rapid POS tap-to-pay) to prevent race conditions.
  - **Offline-First POS Client:** The client caches catalog data and uses eventual consistency to sync offline sales when network connectivity is restored.

  ## Implementation Prompt
  **Target Persona:** Priya the Boutique Owner
  **Outcome:** Implement the backend synchronization and Redis lock mechanism to ensure real-time inventory consistency between online checkouts and the POS terminal, managed seamlessly by the Operations Agent.

  **Critical User Journey (CUJ) & Acceptance Criteria:**
  1. Set up the Redis Redlock inventory reservation service.
  2. Integrate the lock mechanism into both the online checkout flow and the Stripe Terminal POS transaction flow.
  3. Ensure a 15-second reservation is applied during a POS sale.
  4. Verify that an online checkout attempt during this reservation gracefully fails with a stockout message.
  5. Extend the Operations Agent to monitor these stock changes and trigger a low-stock notification to the owner.

  Do not prescribe specific code structures, but ensure the solution adheres strictly to multi-tenant isolation rules (tenant_id). Ensure all features work perfectly on a 375px mobile view.

  **Priority:** P1
  **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []