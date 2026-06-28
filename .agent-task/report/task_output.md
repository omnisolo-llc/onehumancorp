issue_title: "Implement OHC Unified Multi-Channel Inventory Sync & POS"
issue_description: |
  **Feature Name:** OHC Unified Multi-Channel Inventory Sync & POS

  **Problem Statement:**
  Small business owners like Priya (boutique owner) require seamless inventory tracking between online (web/mobile) and in-store operations (tap-to-pay or card reader). Currently, OHC lacks a real-time, strongly consistent inventory locking and caching mechanism, as well as a robust distributed sync protocol for hybrid merchants. Without this, double-booking and out-of-stock scenarios occur during simultaneous online and offline purchases.

  **Research Report:**
  Competitors like Shopify dominate the e-commerce space with extensive POS capabilities but often fail micro-SMEs due to complexity. Their inventory management can be disjointed—online inventory frequently falls out-of-sync with in-person sales unless costly third-party integration tools or higher-tier plans are employed. Square and Stripe Terminal provide robust POS hardware but lack the integrated, agentic workflow automation needed to unify the business operations effortlessly.

  **Design Doc:**
  - **Architecture Diagram (Mermaid.js)**:
    ```mermaid
    graph TD
      A[In-Store POS / Stripe Terminal] -->|Reserve Stock| B(Redis Redlock)
      C[Online Storefront Checkout] -->|Reserve Stock| B
      B -->|Lock Successful| D{Payment Processing}
      D -->|Payment Success| E[PostgreSQL Central Ledger]
      E --> F[Operations Agent]
      F -->|Low Stock Alert / Restock Draft| G[Owner Mobile Feed]
      C -.->|Lock Failed| H[Out of Stock Notification]
    ```
  - **Mobile UX Flow (375px First)**:
    - POS interface optimized for 375px with large touch targets (≥ 44x44px) for rapid inventory adjustment and tap-to-pay.
    - Optimistic UI updates for inventory changes, gracefully rolling back and notifying the user if the Redis reservation fails.
  - **AI Agent Integration Points**:
    - **Operations Agent ("The Manager")**: Actively monitors stock levels across all channels. It tracks incoming orders, triggers low-stock alerts, coordinates with the sync mechanism to reconcile conflicts, and suggests restock plans to the owner.
    - **Finance Agent ("The Accountant")**: Processes splits for Terminal transactions and correlates POS data with online purchases for unified financial reporting.
    - **Customer Success Agent ("The Ambassador")**: Automatically updates online storefront availability and notifies customers if an item in their cart becomes unavailable due to an in-store purchase.

  **Implementation Prompt:**
  - Develop the unified multi-channel inventory synchronization architecture.
  - Integrate a distributed locking mechanism (e.g., Redis Redlock) to manage temporary inventory reservations during both POS and online checkouts, preventing double-booking.
  - Ensure the central ledger (PostgreSQL) is correctly updated after successful transactions, maintaining strict multi-tenant isolation.
  - Connect the Operations Agent to monitor real-time stock levels, handle sync conflicts, and proactively push low-stock notifications and restock suggestions to the owner's mobile feed.
  - The implementation must support a mobile-first (375px) POS interface. Do NOT prescribe exact database schemas or API endpoints; focus on fulfilling the Critical User Journey where an in-store sale prevents a simultaneous online sale of the same final item.

  **Top 5 things that do not make sense in the repository (to be fixed later):**
  1. The `ohc-core` directory and Bazel configurations point to missing dependencies or unstructured module rules (e.g., download errors for rules_perl).
  2. The mobile layouts have inconsistent CSS media queries that don't all follow the 375px-first strict rule.
  3. The `ohc.inventory` protobuf namespace is declared but missing concrete implementations for distributed locks.
  4. There is no clear local `docker-compose` mock for Stripe Terminal during E2E testing, which may block full testing of the POS flow.
  5. The `src/server/ohc/src` directory is missing, indicating a broken structural convention between Cargo and Bazel build targets for the server module.

  **Priority:** P1
  **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
