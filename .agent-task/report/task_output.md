issue_title: "[KDS] Build Real-Time Multilingual Kitchen Display System for Food Operators"
issue_description: |
  # Research Report: Real-Time Multilingual KDS & Pre-Order Engine

  ## Problem Statement
  Fatima (Food Cart Operator, 50) operates a busy halal food cart and takes pre-orders. She struggles with existing platforms because they rely heavily on complex English interfaces, require a constant high-speed internet connection, and fail to provide immediate, loud mobile notifications in noisy environments. When the lunch rush hits, cellular networks get congested, and she cannot afford to miss an order or spend time navigating clunky menus to mark an item as "sold out." Existing systems don't seamlessly bridge the gap between a customer's online pre-order and Fatima's low-end Android device operating as a Kitchen Display System (KDS) in a chaotic, multi-lingual environment.

  ## Research Report
  *   **Competitor Analysis**:
      *   **Square KDS**: Robust but requires dedicated iPad hardware and constant internet. The UI is rigid and English-centric.
      *   **Shopify POS**: Not optimized for quick-service food pre-orders. Lacks native multi-language toggle for staff-facing UI vs customer-facing UI.
      *   **Wix Restaurants**: Heavy web-based interface that performs poorly on low-end Android devices and offline environments.
  *   **The OHC Differentiator**: OHC must provide a zero-hardware KDS that turns any low-end smartphone into a real-time, multilingual pre-order receiver with offline resilience and native-feeling performance.

  ## Design Doc

  ### High-Level Architecture
  ```mermaid
  graph TD;
      Customer[Customer on Storefront] -->|Places Pre-Order| Gateway[Zero-Trust Edge Gateway];
      Gateway --> KAIROS[KAIROS Orchestration Hub];
      KAIROS --> Inventory[(Global Inventory / Ledger)];
      KAIROS --> EventMesh[Hybrid Event Mesh];
      EventMesh -->|Real-Time Push| SyncDaemon[Local Sync Daemon];
      SyncDaemon --> LocalDB[(SQLite Local DB)];
      LocalDB --> KDS_UI[OHC App: KDS View];
      EventMesh --> OperationsAgent[AI Operations Agent];
      OperationsAgent -->|Low Stock Alert| MarketingAgent[AI Marketing Agent];
  ```

  ### Mobile UX Flow (375px)
  1.  **Incoming Order Alert**: Loud, persistent notification overriding silent mode if configured. High-contrast flash on screen.
  2.  **Order Card View**: Large typography, simplified layout. Shows item, modifiers, and pickup time.
  3.  **One-Tap Actions**: "Mark Preparing" / "Mark Ready" using 44x44px minimal touch targets.
  4.  **Quick Status Toggle**: Single tap to mark an item "Sold Out" instantly updating the global inventory via the Local Sync Daemon.
  5.  **Language Toggle**: Immediate UI translation (e.g., English to Arabic) without requiring a full page reload or complex settings navigation.

  ## Implementation Prompt
  **Goal:** Build the frontend UI and data synchronization logic for a Real-Time Multilingual KDS tailored for food cart operators like Fatima.
  **Requirements:**
  1.  Create a mobile-first KDS screen ensuring all interactive elements are easily accessible on a 375px wide screen with 44x44px touch targets.
  2.  Implement a local SQLite-backed data layer to handle incoming orders optimistically, syncing with the central PostgreSQL ledger via the Event Mesh.
  3.  Include an instant language toggle that seamlessly translates the critical UI elements on the KDS screen without a full reload.
  4.  Implement a "sold out" quick-action button that updates the local data store and queues a synchronization event for the backend.
  5.  Ensure zero mock data is present in the UI and all updates flow through the designated CRDT and Outbox Queue mechanisms.

  **Acceptance Criteria:**
  - KDS screen displays orders in real-time, functioning accurately on a 375px viewport.
  - "Sold out" and order status changes work seamlessly without network latency, utilizing the local sync daemon.
  - Language toggle switches the interface immediately.
  - All E2E tests pass, verifying the offline-to-online sync behavior.

  ## Priority: P1
  ## Estimated Scope: Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
