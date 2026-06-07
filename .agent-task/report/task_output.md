issue_title: "Implement Multi-Tenant High-Performance Inventory Sync Capability"
issue_description: |
  ## Mission Queue Protocol

  **Title**: Multi-Tenant High-Performance Inventory Sync Capability

  **Problem Statement**:
  Currently, Priya (our boutique owner) cannot reliably sync her in-store inventory with her online storefront in real-time. If she sells a dress in-store, the online storefront might still show it as available, leading to double-selling and customer disappointment. The existing system lacks a high-performance, strictly isolated multi-tenant background synchronization mechanism to keep physical and digital inventory in perfect lockstep across devices, especially under weak network conditions.

  **Research Report**:
  - **Competitor Analysis**: Shopify handles this via Shopify POS and centralized inventory tracking, but it requires technical setup. Wix relies on polling which is slow.
  - **Market Gap**: A fully autonomous, edge-cached, real-time inventory sync that works offline-first on a 375px mobile device.
  - **Findings**: OHC needs a robust `InventoryLedger` that acts as the single source of truth, updated via an async job queue (PostgreSQL `SKIP LOCKED`) with Redis for real-time edge caching.

  **Design Doc**:
  - **Architecture**:
    - **Data Model**: `InventoryLedger` table (tenant_id, product_id, variant_id, quantity, version).
    - **Queue**: `InventorySyncJob` workers fetching from PostgreSQL queue.
    - **Cache**: Redis keys `ohc:inventory:{tenant_id}:{product_id}`.
  - **Mobile UX Flow**:
    - Priya opens the app (375px). She sees a unified "Inventory" tab.
    - She scans a barcode or taps a product. The UI instantly updates (optimistic update).
    - In the background, the queue syncs the update to the central ledger.
    - If offline, the update is queued locally and synced when online.
  - **AI Integration**: The "Operations" agent monitors the `InventoryLedger`. If an item drops below a threshold, it triggers an alert or auto-reorders.

  **Implementation Prompt**:
  - **User-Facing Outcome**: Priya can update inventory on her mobile device, and it reflects online instantly. She never double-sells an item.
  - **CUJ**: Priya logs in -> navigates to Inventory -> updates quantity of "Red Dress M" -> UI updates instantly -> online store shows correct quantity.
  - **Acceptance Criteria**:
    - Build the `InventoryLedger` tracking mechanism.
    - Implement real-time sync with offline queuing.
    - Ensure strict multi-tenant isolation.
    - 100% unit test coverage.
    - Playwright E2E test verifying the flow on a 375px viewport.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
