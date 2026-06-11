issue_title: "Implement Centralized Inventory & Distributed POS Architecture"
issue_description: |
  # Centralized Inventory & Distributed POS Sync Protocol

  **Problem Statement**
  Currently, Priya (the boutique owner persona) struggles with keeping her in-store inventory and online storefront in perfect synchronization. If she makes an in-person sale using the Stripe Terminal app (Offline/Local First POS Client) while an online user simultaneously tries to checkout the same item on the web, out-of-stock and double-booking errors can occur.

  The system needs a more robust and resilient integration of our Redis Redlock mechanism to temporarily hold (reserve) stock during the Stripe Terminal session to prevent collisions. Additionally, the POS application needs to fully synchronize its cached product catalog and offline transaction log using optimistic concurrency when restoring the connection, while leveraging the AI Operations Agent to smoothly handle conflict resolutions and trigger automatic restock orders.

  **Research Report**
  As found in `docs/business/market_research/[research]_ohc_centralized_inventory_pos.md`, while Shopify and Square dominate POS scenarios, they are incredibly complex for micro-SMEs and often disjointed from online operations unless merchants adopt extensive plugins or third-party integrators.

  OHC's agentic workflow eliminates this "App Tax". The Operations Manager agent acts as a silent mediator. For hybrid online and offline commerce:
  1. We need row-level locking or optimistic concurrency for ultimate truth updates via PostgreSQL (Central Ledger).
  2. We need a Distributed Locking mechanism (`ohc:lock:{tenant_id}:inventory:{product_id}`) utilizing Redis Redlock to establish short-lived 15-second holds during tap-to-pay.
  3. The local POS UI must present truthful offline and syncing states.

  **Design Doc**
  *Architecture Highlights:*
  - **Redis Reservation Layer:** Update checkout pathways to perform an atomic `SET EX NX` against `ohc:lock:<product>`.
  - **Ledger Commit Path:** Finalizing a Stripe Terminal sale triggers `COMMIT`, resolving the Redis Lock, decrementing the `products.inventory_count` within a single PostgreSQL transaction, and writing an immutable ledger entry.
  - **Offline Sync & Mesh Replay:** If the POS device disconnects, it uses `pos_offline_transactions`. A background sync worker (`pos_sync_worker.rs`) continuously polls these, applies them to the master ledger, and resolves the local optimistic state.
  - **AI Agent Handoff:** In the rare case of a split-brain collision resulting in oversold stock upon offline sync reconciliation, the "Operations Manager" agent queues a `POS_INVENTORY_CONFLICT_RESOLUTION` task to dynamically draft an email to the customer with an apology and discount code, resolving the human intervention step autonomously.
  - **Mobile UX Flow:**
    - The POS view strictly scales from 375px wide. Touch targets (Quick Charge, Product selection) are strictly >= 44x44px.
    - Glassmorphism overlays are displayed to reflect 'Syncing...' or 'Offline Mode' banners accurately to the user.

  *Data Model ER Mapping (Conceptual):*
  `Tenant (1)` <-> `(Many) Products`
  `Tenant (1)` <-> `(Many) pos_terminal_sessions`
  `pos_terminal_sessions (1)` <-> `(Many) pos_offline_transactions`
  `Products (1)` <-> `(Many) Redis Redlock Holds`

  **Implementation Prompt**
  Implement the robust Centralized Inventory & POS Sync workflow covering frontend and backend:
  1. Ensure the `pos_sync_worker.rs` and its backend tests strictly comply with the latest `pos_offline_transactions` database migration schema. Clean up any invalid legacy field references (e.g. `transaction_id` instead of `id` and missing mandatory columns).
  2. Implement comprehensive full-stack E2E Playwright tests simulating Priya's Critical User Journey (CUJ): starting the POS session, completing a transaction (offline and online), observing Redlock inventory holds, and seeing the "Syncing" state transition to resolved.
  3. Integrate the UI POS Terminal (`StripeTerminalClient.tsx`) with truthful, robust optimistic UI updates, ensuring that while the transaction syncs to the server, the user's interface remains unblocked and responsive on mobile devices.

  **Priority:** P1
  **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
