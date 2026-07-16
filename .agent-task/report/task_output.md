issue_title: "Implement Centralized Inventory & Distributed POS Architecture"
issue_description: |
  # Research Report: Centralized Inventory & Distributed POS Architecture

  ## 1. Problem Statement
  Small business owners with both online and in-person operations (like Priya the Boutique Owner) struggle with inventory synchronization. Competitors like Shopify require expensive plugins or complex setups to prevent double-booking items between the online store and physical point-of-sale (POS). Without a strongly consistent locking mechanism, simultaneous purchases lead to overselling and poor customer experiences.

  ## 2. Research Report
  - **Market Context**: Platforms like Shopify dominate e-commerce but fail micro-SMEs in omnichannel setups due to complexity. Square and Stripe Terminal provide robust POS hardware but lack the integrated, agentic workflow automation needed to unify business operations effortlessly.
  - **The OHC Opportunity**: Implementing a real-time, strongly consistent inventory locking and caching mechanism, alongside a robust distributed sync protocol for hybrid merchants. This ensures that an in-store tap-to-pay purchase instantly reserves and deducts stock, preventing an online customer from double-booking the same item.
  - **Competitor Gaps**: Disjointed inventory management requiring third-party tools; lack of proactive, agent-driven management.

  ## 3. Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD;
      OnlineCheckout[Online Checkout] --> Redis[Redis Redlock];
      POSClient[Mobile POS Client] --> Redis;
      Redis --> Postgres[Central Ledger PostgreSQL];
      POSClient -.-> OfflineCache[Local Offline Cache];
      OfflineCache -.-> Postgres;
      OpsAgent[Operations Agent] --> Postgres;
      OpsAgent --> CustomerNotifications[Customer Notifications];
  ```

  ### Data Model (PostgreSQL)
  - `Central Ledger`: Source of truth for inventory counts, using optimistic concurrency control.
  - `TerminalSession`: Handles offline-sync reconciliation.

  ### Distributed Locks (Redis Redlock)
  - A temporary inventory reservation system. Lock key pattern: `ohc:lock:{tenant_id}:inventory:{product_id}`.

  ### AI Agent Coordination
  - **Operations Agent**: Monitors stock levels, handles sync conflicts, and triggers low-stock push notifications.

  ### Mobile UX Flow (375px)
  - The POS interface must operate flawlessly on a 375px viewport with touch targets ≥ 44x44px.
  - Optimistic UI updates for inventory changes, with rollback capabilities if the Redis reservation fails.

  ## 4. Implementation Prompt
  **Target Persona**: Priya the Boutique Owner
  **Outcome**: A seamless inventory system where an in-store tap-to-pay purchase instantly reserves and deducts stock, preventing an online customer from double-booking the same item, all managed invisibly by the Operations Agent.

  **Critical User Journey (CUJ)**:
  1. Priya is logged into the OHC mobile app (POS mode) while an online customer browses her storefront.
  2. Priya processes an in-store sale for the last "Red Dress" using the Stripe Terminal integration.
  3. The system applies a 15-second Redis Redlock to reserve the item during the transaction.
  4. The online customer attempts to checkout the same "Red Dress" but receives a graceful "Item just sold out" message, triggered by the Operations Agent.
  5. The POS transaction finalizes, the PostgreSQL ledger is updated, and the Operations Agent sends Priya a notification: "Red Dress sold out. Would you like to draft a restock order?"

  **Acceptance Criteria**:
  - Redis Redlock inventory reservation service implemented and integrated into the checkout flow.
  - `TerminalSession` schema handles offline-sync reconciliation.
  - Operations Agent monitors real-time stock levels and handles conflicts.
  - POS interface optimized for 375px viewport.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
