issue_title: "Implement Centralized Inventory Sync & Distributed Agentic POS Architecture"
issue_description: |
  # Research Report: Centralized Inventory & Distributed Agentic POS Architecture

  ## Problem Statement
  Service-based and multi-channel small business owners (e.g., Priya the boutique operator) struggle with disjointed inventory management between online sales and in-person operations. Current solutions like Shopify are too complex or expensive for micro-SMEs, often leading to double-booking and out-of-stock scenarios during simultaneous online and offline purchases. Wix and Squarespace lack proactive agent-driven management. The OHC platform must provide a centralized inventory and distributed Point-of-Sale (POS) synchronization architecture that leverages AI agents to handle conflicts invisibly.

  ## Research Report
  - **Market Context**: Traditional platforms like Shopify have robust POS capabilities but fail micro-SMEs on setup complexity and "app taxes". Platforms like Square and Stripe Terminal provide robust POS hardware but lack the integrated, agentic workflow automation needed to unify business operations effortlessly.
  - **The OHC Opportunity**: By introducing a real-time inventory locking/caching mechanism and a distributed sync protocol, OHC can leverage the Operations Agent to proactively manage the calendar and stock, eliminating the need for third-party sync apps.
  - **Competitor Gaps**:
    - *Shopify*: Disjointed inventory without costly third-party integrations; complex.
    - *Wix/Squarespace*: Passive systems with limited operational sync for physical tap-to-pay and online checkout.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  erDiagram
      LEDGER ||--o{ INVENTORY : "tracks"
      INVENTORY ||--o{ REDIS_LOCK : "reserves"
      POS_CLIENT }o--|| LEDGER : "syncs offline sales"
      OPERATIONS_AGENT ||--o{ INVENTORY : "monitors & alerts"
  ```
  ### Data Model & Invariants
  - **Central Ledger (PostgreSQL)**: Ultimate source of truth for all inventory counts. Uses row-level locking (`SELECT ... FOR UPDATE`) or optimistic concurrency control for critical updates.
  - **Distributed Locks (Redis Redlock)**: A temporary inventory reservation system applied during the checkout process to prevent double-booking. Lock key pattern: `ohc:lock:{tenant_id}:inventory:{product_id}`. Duration is tuned dynamically (e.g., 5 minutes for online carts vs. 15 seconds for tap-to-pay).
  - **TerminalSession & Offline Sync**: The mobile POS client caches catalog data locally and syncs finalized offline sales asynchronously when network is restored.

  ### Mobile UX Flow (375px)
  1. **Customer Checkout**: Clean, touch-friendly online checkout. If an item is reserved by POS, the UI gracefully displays an optimistic "Item just sold out" message.
  2. **Owner POS View (Priya)**: A fast tap-to-pay interface with large touch targets (≥ 44x44px) that applies Redis locks instantly. The owner receives push notifications from the Operations Agent for low-stock alerts and restock drafting.

  ### AI Agent Integration
  - **Operations Agent ("The Manager")**: Monitors stock levels. Handles sync conflicts, triggers low-stock alerts, and drafts restock orders.
  - **Customer Success Agent ("The Ambassador")**: Automatically notifies online customers if an item in their cart becomes unavailable due to an in-store purchase.

  ## Implementation Prompt
  **Feature Name**: OHC Unified Multi-Channel Inventory Sync & POS
  **Target Persona**: Priya the Boutique Operator
  **Outcome**: A seamless inventory system where an in-store tap-to-pay purchase instantly reserves and deducts stock, preventing an online customer from double-booking, all managed invisibly by the Operations Agent.

  **Critical User Journey (CUJ)**:
  1. Priya processes an in-store sale for the last "Red Dress" using the mobile POS interface.
  2. The system applies a 15-second Redis Redlock (`ohc:lock:{tenant_id}:inventory:{product_id}`) to reserve the item.
  3. An online customer attempts to checkout the same "Red Dress" but receives a graceful "Item just sold out" message.
  4. The POS transaction finalizes, PostgreSQL ledger is updated, and the Operations Agent sends Priya a notification to draft a restock order.

  **Acceptance Criteria**:
  - Implement Redis Redlock inventory reservation service and integrate into the checkout flow.
  - Design the `TerminalSession` PostgreSQL schema for offline-sync reconciliation.
  - Extend the Operations Agent to trigger notifications on low stock.
  - Build the mobile-first (375px) POS interface with ≥ 44x44px touch targets.
  - MUST include automated E2E Playwright tests verifying the double-booking prevention flow.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
