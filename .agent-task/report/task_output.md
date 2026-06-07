issue_title: "Implement Unified Tap-to-Pay POS with Redis Redlock & Mobile-First Agent Approval UI"
issue_description: |
  # Research Report: Implement Unified Tap-to-Pay POS with Redis Redlock & Mobile-First Agent Approval UI

  ## Problem Statement
  Hybrid merchants (like Priya the Boutique Owner) currently suffer from disjointed inventory between online stores and physical Point-of-Sale (POS). A transaction via in-store tap-to-pay does not instantly reserve online inventory, leading to out-of-stock double bookings. Furthermore, complex backend inventory controls require desktop navigation, directly contradicting the OHC mobile-first philosophy.

  ## Research Report & Gap Analysis
  - **The Mobile Management Gap**: As shown in `ohc_smb_mobile_first_design_research.md`, 100% of business operations for an SMB should be doable from a 375px mobile screen. Legacy platforms require falling back to a desktop to resolve inventory conflicts.
  - **The Inventory Gap**: As outlined in `[research]_ohc_centralized_inventory_pos.md`, OHC lacks a robust distributed lock mechanism during checkout to prevent double-booking.
  - **Solution Strategy**: Implement a Central Ledger in PostgreSQL supported by a distributed locking mechanism (Redis Redlock) for ephemeral cart reservations. Operations agents should seamlessly handle sync conflicts and trigger mobile-first Approval UI cards for low-stock restock drafting.

  ## Design Doc
  - **Central Ledger (PostgreSQL)**: Serves as the ultimate source of truth for inventory counts.
  - **Distributed Locks (Redis Redlock)**: Applies temporary reservations (e.g. 15 seconds for tap-to-pay, 5 minutes for online carts) using keys like `ohc:lock:{tenant_id}:inventory:{product_id}`.
  - **Offline/Local First POS Client**: Mobile client employs eventual consistency for offline sales and syncs back asynchronously.
  - **Mobile UX Flow (375px First)**: Instead of complex admin screens, if a sync conflict or restock event occurs, the Operations Agent produces a single "Action Card" to the mobile feed. The user simply taps "Approve Restock" (touch target > 44px).
  - **AI Agent Integration**:
    - **Operations Agent**: Monitors stock levels, coordinates with Redlock, alerts on sync conflicts.
    - **Customer Success Agent**: Refreshes online storefront availability proactively.

  ## Implementation Prompt
  Implement the Unified Multi-Channel Inventory Sync & POS flow.
  The primary Critical User Journey (CUJ) is for Priya the Boutique Owner:
  1. She processes an in-store sale for an item using Stripe Terminal.
  2. The system applies a Redis Redlock during the transaction to prevent online customers from adding the item to their cart.
  3. The system gracefully updates PostgreSQL and sends an "Action Card" to the 375px mobile feed suggesting a restock if inventory hits zero.
  4. Build the core Redis reservation service and the Mobile-first UI agent notification card without dictating exact database columns.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
