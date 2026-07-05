issue_title: "Implement Multi-Channel Inventory Caching with Redis Redlock"
issue_description: |
  # Research Report: Centralized Inventory & Distributed POS Architecture

  ## Problem Statement
  Priya (boutique owner) requires seamless inventory tracking between online (web/mobile) and in-store operations (tap-to-pay or card reader). Currently, OHC lacks a real-time, strongly consistent distributed locking mechanism. Simultaneous online carts and offline checkouts can lead to double-booking or overselling.

  ## Research Report
  Competitors like Shopify dominate the e-commerce space with extensive POS capabilities but often fail micro-SMEs due to complexity. Their inventory management can be disjointed. Square and Stripe Terminal provide robust POS hardware but lack the integrated, agentic workflow automation.

  ## Design Doc
  - **Architecture Diagram**:
    ```mermaid
    graph TD
        ClientOnline -->|Add to Cart| APIServer
        ClientPOS -->|Tap to Pay| APIServer
        APIServer -->|Acquire Lock| RedisRedlock
        APIServer -->|Finalize Sale| PostgresLedger
    ```
  - **Mobile UX Flow (375px)**: The POS screen (mobile) should display a clear, non-technical "Item reserved in online cart" translucent glass badge if the item is locked.
  - **AI Agent Integration Points**: The Operations Agent ("The Manager") actively monitors stock levels across all channels, triggers low-stock alerts, and suggests restock plans.
  - **Key Design Decisions**:
    - Central Ledger in PostgreSQL (`inventory_items` table).
    - Distributed Locks in Redis using Redlock pattern. Lock Key Pattern: `ohc:lock:{tenant_id}:inventory:{product_id}`.

  ## Implementation Prompt
  **Feature Name**: Multi-Channel Inventory Locking
  **Target Persona**: Priya the Boutique Operator
  **Outcome**: Priya can confidently sell online and in-store simultaneously without fear of double-selling limited items.

  **Acceptance Criteria**:
  1. Implement a Redis Redlock utility module.
  2. Expose an API endpoint for acquiring and releasing inventory locks.
  3. Ensure the Redis connection pool is properly initialized.
  4. Ensure POS interface operates flawlessly on a 375px viewport with optimistic UI updates.
  5. E2E tests for concurrency control.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
