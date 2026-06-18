issue_title: "Implement High-Performance Offline-Capable Terminal SDK Architecture"
issue_description: |
  **Title**: Implement High-Performance Offline-Capable Terminal SDK Architecture

  **Problem Statement**:
  Currently, physical business personas like Priya (boutique owner) and Fatima (food cart operator) struggle to process in-person payments reliably when their mobile networks fluctuate or during high foot-traffic events (e.g., weekend markets or conventions). The existing payment stack lacks a dedicated, offline-tolerant, caching-enabled Terminal SDK architecture. Owners are forced to wait for network roundtrips, resulting in stalled checkouts, lost revenue, and poor customer experiences at the point of sale.

  **Research Report**:
  - **Market Context**: Platforms like Square, Shopify POS, and Stripe Terminal have long provided localized caching and eventual-consistency ledgers to ensure immediate transaction feedback and subsequent async settlement.
  - **Codebase Insights**: Reviewing our current payment systems, there is an over-reliance on synchronous remote calls for all payment flows. Multi-tenancy is strong on the server (PostgreSQL + RLS), but weak in local state persistence on mobile devices.
  - **Impact**: Without robust offline support and a localized point-of-sale layer, OHC falls short for in-person operators.

  **Design Doc**:
  - **Architecture Diagram**:
    ```mermaid
    graph TD;
        StorefrontClient[Flutter App] --> POSCache[POS Local Ledger Cache];
        POSCache --> EventQueue[Event Sync Queue];
        EventQueue -.-> NetworkBoundary[Network Boundary];
        NetworkBoundary --> APIGateway[API Gateway];
        APIGateway --> Postgres[(PostgreSQL)];
    ```
  - **Mobile UX Flow (375px First)**:
    1. Cashier enters amount or selects items in a quick-tap grid.
    2. App immediately processes payment (if offline via cached config/cash, or signals intent for card terminal).
    3. Success screen displays instantly with a visual indicator of "Syncing..." or "Synced" based on network state.
  - **AI Agent Integration**:
    *The Operations Assistant* can proactively alert the owner if terminal devices have low battery, or if there's a sync backlog of off-line transactions that need attention. *The Finance Assistant* reconciles offline batches against bank payouts at the end of the day.
  - **Key Design Decisions**:
    - *Local-First Persistence*: Transactions are written to local storage first. The sync queue guarantees at-least-once delivery to the backend.
    - *Idempotency Guarantee*: The mobile client generates UUIDv4 idempotency keys before logging any transaction to prevent double charges upon sync.
    - *Zero Trust & Multi-Tenancy*: Local databases are strictly segmented by `tenant_id` to prevent data leakage in shared-device scenarios.

  **Implementation Prompt**:
  Implement the offline-capable Terminal SDK architecture for the Flutter application and the corresponding backend sync endpoints.
  - Update the mobile frontend to queue transactions locally when offline and synchronize them when connectivity is restored.
  - Implement a visual indicator on the mobile POS screen (375px viewport) showing sync status.
  - Expose backend endpoints to accept batched, out-of-order transaction syncs with strict idempotency validation.
  - The feature must be tested extensively using the browser/Playwright flow to ensure zero-data-loss behavior when simulating network drops.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
