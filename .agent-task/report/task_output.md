issue_title: "[research] Implement Centralized Inventory & Distributed POS Architecture"
issue_description: |
  # Research Report: Centralized Inventory & Distributed POS Architecture

  ## Problem Statement
  Small business owners like Priya (boutique operator) who sell both online and in-store face significant challenges maintaining inventory sync. Existing systems (Shopify + third party apps, Square) either lack seamless integration between online and offline or require expensive add-ons. Without a strongly consistent inventory locking mechanism, merchants experience double-booking, overselling, and inventory drift between online storefronts and mobile POS systems (tap-to-pay).

  ## Research Report
  - **Market Context**: Platforms like Shopify dominate e-commerce but their basic plans struggle with real-time offline sync without complex, paid add-ons. Square is strong offline but less integrated for complex online storefront needs without extensive setup.
  - **The OHC Opportunity**: Providing a single, unified backend that guarantees strong consistency using a Central Ledger and Distributed Locks, while supporting an offline-capable, locally-cached mobile POS client. AI agents can coordinate behind the scenes to reconcile eventual consistency when network conditions improve.
  - **Competitor Gaps**:
    - *Shopify*: Often requires third-party apps for robust real-time hybrid sync; online and offline can drift.
    - *Square*: Excellent hardware/POS, but advanced online routing and AI-agentic automation are bolted on.

  ## Design Doc
  ### Architecture Diagram (Concept)
  ```mermaid
  graph TD
      A[Mobile POS Client - Flutter] -->|Sync/Async| B[API Layer - Axum]
      C[Online Storefront - Web/PWA] -->|Sync| B
      B --> D{Distributed Locks - Redis}
      D --> E[(Central Ledger - PostgreSQL)]
      B --> F[AI Operations Agent]
      F --> E
  ```

  ### Data Model & Sync Protocol
  - **Central Ledger (PostgreSQL)**: Source of truth. Uses row-level locking or optimistic concurrency.
  - **Distributed Locks (Redis)**: Temporary reservations (e.g., `ohc:lock:{tenant_id}:inventory:{product_id}`) to prevent double-booking during checkout.
  - **Offline/Local First**: Mobile client caches catalog and queues offline sales. Eventual consistency handles reconciliation upon reconnection.

  ### Mobile UX Flow (375px)
  1. **POS View**: Cashier (Priya) sees a fast, locally cached product grid (44x44px min touch targets).
  2. **Checkout**: Tap-to-pay is initiated. The system attempts a fast lock; if offline, it queues the transaction with a clear "Pending Sync" indicator.
  3. **Reconciliation**: Upon network restore, the app silently syncs queued transactions. If a conflict occurs, the AI Operations Agent flags it in the Owner's work feed.

  ### AI Integration
  - **Operations Agent**: Monitors inventory levels, detects sync conflicts (e.g., sold offline and online simultaneously), and alerts the owner with a suggested resolution (e.g., "Draft refund for online order or substitute item").

  ## Implementation Prompt
  **Feature Name**: OHC Centralized Inventory & Distributed POS Sync

  **User Persona**: Priya (Boutique Operator)

  **The CUJ (Critical User Journey)**:
  1. Priya opens the OHC mobile app (POS mode) on her phone.
  2. She adds a physical product to the cart and processes an in-person payment (tap-to-pay).
  3. Meanwhile, a customer online adds the *same* last-in-stock item to their cart.
  4. The system must use a distributed lock to reserve the item for the online cart temporarily, or commit the offline sale immediately if it occurs first, accurately reflecting the remaining stock.
  5. If Priya is offline, the sale is queued locally. When reconnected, if the item was oversold, the AI Agent notifies her in the feed with a suggested action.

  **Acceptance Criteria**:
  - Implement the core API endpoints for POS operations (`/sync_offline`, `/reserve`, `/commit`) and inventory management.
  - Ensure multi-tenant isolation (`tenant_id`) in all database schemas and Redis lock keys.
  - The mobile-first POS UI must handle offline states gracefully with visual indicators ("Pending Sync").
  - The AI Operations Agent must be integrated to flag anomalies and suggest resolutions.
  - All E2E tests must pass, specifically verifying the concurrency lock (preventing double booking).

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
