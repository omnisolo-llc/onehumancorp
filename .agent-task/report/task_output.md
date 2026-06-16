issue_title: "Implement AI-Native Tap-to-Pay & Offline Sync Terminal Integration"
issue_description: |
  ## Research Report: Centralized Inventory & Distributed POS Architecture with AI Agent Orchestration

  **Persona Focus**: Priya (boutique owner) and Carlos (handyman) require seamless payment processing and inventory tracking across both online (web/mobile) and in-store/field operations (tap-to-pay or card reader) using mobile devices.

  **The Gap / Problem Statement**:
  Currently, OHC lacks a robust, real-time tap-to-pay POS client integration combined with an offline-tolerant sync protocol. Without this, multi-channel merchants (online + offline) experience severe friction:
  - Disconnected checkout experiences requiring third-party, non-integrated hardware.
  - Risk of double-booking or selling out-of-stock items because online and offline inventory are not synchronized atomically.
  - No AI coordination to reconcile financial splits, generate unified daily reports, or alert on stock depletion in real time.
  Users like Priya currently have to use a separate POS system (like Square) that doesn't talk to her online storefront, making her "Work Assistant" blind to offline sales.

  **Research Report**:
  - **Competitor Analysis**: Shopify and Square dominate POS, but their solutions lack deep, proactive AI agent integration out of the box. They act as passive ledgers. OHC can differentiate by deeply embedding POS events into the Agent Feed (triggering Operations and Finance Agents automatically upon sale).
  - **Data Resilience**: We must employ a distributed locking mechanism (Redis Redlock) for checkout inventory reservation and an offline-first mobile architecture (Eventual Consistency queue) to allow offline field sales (e.g., Carlos working in an area with poor cell service) to sync upon reconnection.

  **Design Doc**:
  - **Architecture Diagram (Mental Model)**:
    - **Frontend (Flutter PWA)**: Mobile-first POS view with Tap-to-Pay SDK integration (e.g., Stripe Terminal SDK wrappers). 375px optimized layout.
    - **Offline Queue**: Local device queue (IndexedDB/SQLite via PowerSync or custom sync layer) storing transactions when offline.
    - **Backend (Rust)**:
      - `terminal_api.rs`: Existing API must be extended to support offline sync conflict resolution and robust token exchange.
      - **Sync Worker**: A worker process that ingests the offline queue and applies transactions to the PostgreSQL Central Ledger.
    - **Data Model**: Enhanced `Invoice` and `PaymentIntent` structures to support `offline_id` and `sync_status`.
    - **AI Agent Integration**:
      - **Finance Agent**: Automatically ingests offline POS transactions to update the daily P&L summary.
      - **Operations Agent**: Tracks real-time inventory deduction and initiates low-stock protocols if an in-store sale depletes web inventory.

  **Mobile UX Flow (375px)**:
  1. Open OHC App -> POS Tab.
  2. Add items to cart (large touch targets, responsive to quick taps).
  3. Hit "Charge". App attempts Tap-to-Pay.
  4. If offline, the app visibly indicates "Saved Offline" and adjusts local inventory count optimistically.
  5. Upon network restoration, a background sync occurs, updating the owner's Agent Feed with a success notification.

  **Implementation Prompt**:
  *For the Implementer Agent*:
  Implement the "Tap-to-Pay & Offline Sync Terminal" feature. Your implementation should:
  1. Create a robust mobile-first (375px) POS cart and checkout UI in the frontend (Flutter/Next.js).
  2. Implement an offline-first transaction queue that optimistically updates local state and syncs to the backend via `sync_gateway.rs` or a new endpoint when online.
  3. Ensure backend idempotency for POS transactions to handle duplicate sync attempts gracefully.
  4. Integrate the new offline/online POS transaction events into the AI Agent Feed so the Finance and Operations agents can react.
  5. The UI must contain ZERO mock data and rely on the actual backend connection (or truthful offline state).
  6. E2E Playwright tests must be added to verify the offline queuing and online synchronization flows.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
