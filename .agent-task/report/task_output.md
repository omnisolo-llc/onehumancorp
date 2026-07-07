issue_title: "Implement Real-time Inventory Sync and Distributed Locks for Hybrid POS"
issue_description: |
  # Research Report: Centralized Inventory & Distributed POS Architecture

  ## 1. Problem Statement
  Small business owners like Priya (boutique operator) manage both online storefronts and in-person tap-to-pay sales. Currently, OHC lacks real-time inventory synchronization across these channels. When a customer buys the last item in-store concurrently with an online checkout, double-booking occurs, leading to cancelled orders, lost revenue, and poor customer experience. We need a strongly consistent, offline-tolerant inventory management system to guarantee stock accuracy across all channels.

  ## 2. Research Report
  - **Market Context**: Platforms like Shopify require higher-tier plans or complex third-party tools to achieve true omnichannel inventory sync. Square and Stripe Terminal provide robust POS hardware but lack agentic workflows that automatically notify owners of stock-outs or draft restock orders.
  - **The Gap**: OHC requires an inventory reservation system using distributed locks (Redis Redlock) and a central ledger (PostgreSQL) to ensure consistency during concurrent checkouts, combined with an AI operations agent to manage the aftermath (restocking, notifications).
  - **Competitive Differentiation**: By making the Operations Agent aware of real-time inventory and distributed locks, OHC can instantly notify the owner of low stock and proactively draft reorders without the owner navigating complex inventory dashboards.

  ## 3. Design Doc

  ### Architecture
  - **Central Ledger (PostgreSQL)**: The ultimate source of truth for inventory. Uses row-level locking or optimistic concurrency control (`version` column) for critical updates.
  - **Distributed Locks (Redis Redlock)**: A temporary inventory reservation system applied during checkout.
    - Online checkout lock duration: 5 minutes.
    - In-person tap-to-pay lock duration: 15 seconds.
    - Lock key pattern: `ohc:lock:{tenant_id}:inventory:{product_id}`.
  - **Offline/Local First POS Client**: The mobile POS client caches catalog data locally and syncs finalized sales asynchronously.
  - **Operations Agent Integration**: Monitors stock levels and lock acquisitions. Triggers low-stock alerts and coordinates sync conflicts.

  ### Mobile UX Flow (375px first)
  1. Priya opens the OHC app (POS mode) and selects "Red Dress" for a customer.
  2. The UI immediately shows a subtle "Reserving..." state (optimistic update).
  3. Under the hood, a Redis lock is acquired for 15 seconds.
  4. The online storefront updates in real-time (via WebSockets/SSE) to show "Out of Stock" for the "Red Dress" to any browsing customers.
  5. Priya taps to pay, the transaction finalizes, the lock is released, and the PostgreSQL ledger is permanently updated.
  6. The Operations Agent sends Priya a clean, translucent glass notification card: "Red Dress sold out. Tap to draft a restock order."

  ## 4. Implementation Prompt
  **Feature Name**: OHC Unified Multi-Channel Inventory Sync
  **Target Persona**: Priya the Boutique Owner
  **Outcome**: Guarantee that an in-store tap-to-pay purchase instantly reserves stock and prevents online double-booking, seamlessly managed by the Operations Agent.

  **Acceptance Criteria**:
  1. Implement Redis Redlock inventory reservation in the checkout/POS service layer.
  2. Implement an optimistic concurrency control mechanism in the PostgreSQL inventory table.
  3. Ensure the mobile POS interface handles the reservation state optimistically and functions flawlessly on a 375px viewport (minimum 44x44px touch targets).
  4. Integrate the Operations Agent to trigger a "low stock" or "sold out" notification card with a "draft restock" action when an item hits zero.

  ## 5. Priority & Scope
  - **Priority**: P1 (High)
  - **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
