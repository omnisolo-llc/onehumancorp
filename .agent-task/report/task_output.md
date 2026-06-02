issue_title: "[architecture] Universal Multilingual KDS & Offline-First Pre-Order Engine"
issue_description: |
  # Architecture Deep Dive: Universal Multilingual KDS & Offline-First Pre-Order Engine

  ## Problem Statement
  Fatima (Food Cart Operator, 50) operates a busy halal food cart and takes pre-orders. She struggles with existing platforms because they rely heavily on complex English interfaces, require a constant high-speed internet connection, and fail to provide immediate, loud mobile notifications in noisy environments. When the lunch rush hits, cellular networks get congested, and she cannot afford to miss an order or spend time navigating clunky menus to mark an item as "sold out." Existing systems don't seamlessly bridge the gap between a customer's online pre-order and Fatima's low-end Android device operating as a Kitchen Display System (KDS) in a chaotic, multi-lingual environment.

  ## Research Report & Core Insights
  *   **Competitor Analysis**:
      *   **Square KDS**: Robust but requires dedicated iPad hardware and constant internet. The UI is rigid and English-centric.
      *   **Shopify POS**: Not optimized for quick-service food pre-orders. Lacks native multi-language toggle for staff-facing UI vs customer-facing UI.
      *   **Wix Restaurants**: Heavy web-based interface that performs poorly on low-end Android devices and offline environments.
  *   **The OHC Differentiator**: OHC must provide a zero-hardware KDS that turns any low-end smartphone into a real-time, multilingual pre-order receiver with offline resilience and native-feeling performance.
  *   **The Gap:** OneHumanCorp current architecture lacks a multi-tenant, edge-caching and offline-first queue capable of guaranteeing immediate synchronization between online storefronts and low-end mobile POS devices in low-connectivity environments.

  ## Design Doc

  ### High-Level Architecture
  ```mermaid
  graph TD;
      Customer[Customer on Storefront] -->|Places Pre-Order| EdgeNode[OHC Edge Server];
      EdgeNode --> AI_Ops[Operations Agent: Order Processing];
      AI_Ops --> CoreDB[(PostgreSQL Tenant DB)];
      AI_Ops --> KDSQueue[Redis Order Event Queue];
      KDSQueue -->|Real-Time Push / Sync| SyncDaemon[Mobile Local Sync Engine];
      SyncDaemon --> LocalDB[(SQLite Local Storage)];
      LocalDB --> KDS_UI[OHC App: Multilingual KDS View];
      KDS_UI -->|Optimistic UI Action| SyncDaemon;
      SyncDaemon -->|Background Sync| CoreDB;
  ```

  ### Data Model & Invariants
  *   **Entity Relationships:**
      *   `Tenant`: Fatima's food cart.
      *   `Order`: Core entity placed by customer, tracked by state (`Received`, `Preparing`, `Ready`, `Fulfilled`).
      *   `InventoryItem`: Menu item with `is_sold_out` boolean and `stock_count`.
  *   **Invariants:** Row-level tenant isolation is strictly enforced. Offline actions on `Order` status are queued with timestamps and synchronized using a last-write-wins (LWW) conflict resolution strategy when connectivity is restored.

  ### AI Department Coordination
  *   **Operations (The Manager):** Listens to incoming pre-orders, updates the Redis queue, and immediately pushes notifications to the mobile client.
  *   **Marketing (The Promoter):** If Fatima toggles "Sold Out" on a popular item, the Operations agent updates the database, and the Marketing agent immediately drafts an Instagram story update ("Chicken Over Rice is sold out! Grab the lamb while it lasts!") for 1-tap approval.

  ### Mobile-First UX Flow (375px First)
  1.  **Lock Screen**: Fatima receives a native push notification: "New Pre-Order: 2x Chicken Over Rice."
  2.  **KDS View (Arabic + English)**: High-contrast, large touch targets (≥ 60x60px for core actions). The screen displays a queue of active orders with clear status badges.
  3.  **Action**: Fatima taps a massive green "Preparing" button. The UI optimistically updates instantly, queueing the change locally if offline.
  4.  **Sold-Out Toggle**: A prominent, single toggle next to the item photo instantly marks it as sold out locally and initiates a sync.

  ### Performance & Security Targets
  *   **Offline-First**: App must be fully functional for reading current orders and changing their status without an active connection.
  *   **Latency**: Optimistic UI updates < 100ms.
  *   **Security**: Authentication via SPIFFE/SPIRE with rigid tenant isolation for all sync payloads.

  ## Implementation Prompt
  **Objective**: Implement the Universal Multilingual KDS & Offline-First Pre-Order Engine.

  **User Journey (CUJ) & Acceptance Criteria**:
  1.  **Order Sync**: Simulate a pre-order creation. Ensure it appears in the mobile KDS view.
  2.  **Multilingual Display**: Verify the KDS view can switch to Arabic (RTL support) while maintaining data integrity.
  3.  **Offline Action**: Disconnect the network, change an order status to "Preparing", and verify the optimistic UI updates instantly.
  4.  **Background Sync**: Reconnect the network and verify the offline status change successfully syncs to the backend database.
  5.  **Sold Out Toggle**: Toggle an inventory item to "Sold Out" and verify it propagates to the backend.

  **Constraints**:
  Focus on the UI state management, the local SQLite/storage layer for offline capability, and the background synchronization mechanism. Ensure the UI adheres strictly to the OHC Premium Glassmorphism and UniFi card layouts, prioritizing massive, clear touch targets for a non-technical user in a chaotic environment.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
