issue_title: "Unified Multi-Channel Inventory Sync & POS"
issue_description: |
  # Unified Multi-Channel Inventory Sync & POS

  ## Problem Statement
  Currently, OneHumanCorp lacks a real-time, strongly consistent inventory locking and caching mechanism, as well as a robust distributed sync protocol for hybrid merchants. Without this, double-booking and out-of-stock scenarios occur during simultaneous online and offline purchases. Priya (the boutique owner persona) requires seamless inventory tracking between online (web/mobile) and in-store operations (tap-to-pay or card reader) without needing to configure complex third-party tools.

  ## Research Report
  Our competitive analysis (Track 1) found that tools like Shopify excel at POS integrations but often fail micro-SMEs due to complexity and disjointed inventory updates between channels without expensive addons. We explored Stripe Terminal, which provides the hardware interface but lacks the integrated agentic workflow required. This feature directly targets this gap, as defined in `docs/business/market_research/[research]_ohc_centralized_inventory_pos.md`.

  ## Design Doc
  **Architecture Overview**
  1. **Central Ledger (PostgreSQL)**: Serves as the ultimate source of truth. Uses row-level locking or optimistic concurrency.
  2. **Distributed Locks (Redis Redlock)**: A reservation system applied during checkout. `ohc:lock:{tenant_id}:inventory:{product_id}`.
  3. **Offline/Local-First POS Client**: Mobile POS client caches catalog locally, with eventual consistency and asynchronous reconciliation upon network restoration.
  4. **AI Coordination**:
     - *Operations Agent ("The Manager")*: Monitors stock levels, alerts on low stock, and suggests restocks.
     - *Customer Success Agent ("The Ambassador")*: Updates online storefront availability instantly.

  ```mermaid
  sequenceDiagram
      participant App as POS Client (Mobile)
      participant Term as Stripe Terminal
      participant API as OHC API Layer
      participant Redis as Redis (Redlock)
      participant DB as PostgreSQL Ledger
      participant Ops as Operations Agent

      App->>Term: Initiate Tap-to-Pay for "Red Dress"
      App->>API: Reserve Inventory (`reserve_inventory`)
      API->>Redis: Acquire Redlock `ohc:lock:{tenant_id}:inventory:{product_id}`
      Redis-->>API: Lock Acquired
      API-->>App: Reservation Confirmed
      Term-->>App: Payment Success
      App->>API: Commit Inventory (`commit_inventory`)
      API->>DB: Update `inventory_count`, release lock
      DB-->>API: Ledger Updated
      API->>Ops: Trigger Event (Stock Deducted)
      Ops-->>App: Push Notification (Restock Alert if low)
  ```

  **Mobile UX Flow (375px Target)**
  - Touch targets for POS checkout are ≥ 44x44px.
  - Optimistic UI updates when processing an in-store sale.
  - Push notifications on mobile device when inventory reaches zero (triggering restock prompt).

  ## Implementation Prompt
  **Outcome**: A seamless inventory system where an in-store tap-to-pay purchase instantly reserves and deducts stock, preventing an online customer from double-booking the same item, all managed invisibly by the Operations Agent.

  **CUJ**:
  1. Priya is logged into the OHC mobile app (POS mode).
  2. Priya processes an in-store sale using Stripe Terminal.
  3. System applies a Redis Redlock reservation.
  4. Online customer attempting to checkout the same item receives "Item just sold out".
  5. POS transaction finalizes, PostgreSQL ledger updates, Operations Agent notifies Priya to restock.

  **Acceptance Criteria**:
  - Implement Redis Redlock inventory reservation service.
  - Integrate reservation into the checkout flow.
  - Update `TerminalSession` data schema to handle offline-sync reconciliation.
  - Extend Operations Agent to monitor real-time stock levels, handle sync conflicts, and trigger low-stock push notifications.
  - 100% unit test coverage for new functionality. E2E Playwright tests verifying the CUJ.

  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
