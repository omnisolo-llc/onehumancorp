issue_title: "Implement Distributed POS Sync & Redis Inventory Locking"
issue_description: |
  # Research Report: Centralized Inventory & Distributed POS Architecture

  ## Problem Statement
  Priya, a boutique owner using OHC, struggles with inventory discrepancies when selling online and in-store simultaneously. The current system lacks a real-time, strongly consistent inventory locking mechanism, leading to out-of-stock scenarios and double-booking.

  ## Research Report
  - Competitors like Shopify use robust POS capabilities, but lack integrated AI workflow automation.
  - OHC needs to ensure a seamless, multi-channel inventory tracking mechanism between online (web/mobile) and in-store operations.
  - The solution must include Redis Redlock to act as a temporary reservation system during checkout and prevent double booking.

  ## Design Doc
  - **Data Model & Sync Protocol**:
    - Central Ledger: PostgreSQL acts as the ultimate source of truth, utilizing row-level locking.
    - Distributed Locks: Redis Redlock reserves inventory during the checkout process (e.g. 5 minutes for online vs 15 seconds for POS). Lock pattern: `ohc:lock:{tenant_id}:inventory:{product_id}`.
    - Offline-first client support for the mobile POS to cache catalog data and eventually sync with the central ledger.
  - **AI Coordination**:
    - Operations Agent ("The Manager"): Monitors stock, triggers low-stock alerts, suggests restock plans.
    - Customer Success Agent ("The Ambassador"): Updates online storefront availability and notifies customers if an item in their cart becomes unavailable due to an in-store purchase.
  - **Mobile UX Flow**: POS interface must flawlessly operate on 375px viewports with touch targets >= 44x44px. Include optimistic UI updates for inventory changes.

  ## Implementation Prompt
  Implement the OHC Unified Multi-Channel Inventory Sync & POS feature.
  - **Target Persona**: Priya the Boutique Owner.
  - **CUJ**: Priya uses OHC mobile app (POS mode) while an online customer browses the storefront. She processes an in-store sale. The system applies a 15-second Redis Redlock to reserve the item. The online customer attempts to checkout the same item but receives an "Item just sold out" message. The POS transaction finalizes, the PostgreSQL ledger is updated, and the Operations Agent notifies Priya.
  - **Acceptance Criteria**:
    - Implement Redis Redlock inventory reservation service and integrate it into the checkout flow.
    - Refine the TerminalSession data schema to handle offline-sync reconciliation.
    - Extend Operations Agent to monitor real-time stock levels, handle sync conflicts, and trigger low-stock push notifications.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
