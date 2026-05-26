issue_title: "[Architecture] Autonomous Offline-First Multi-Location Inventory Synchronization Engine"
issue_description: |
  ## Title
  [Architecture] Autonomous Offline-First Multi-Location Inventory Synchronization Engine

  ## Problem Statement
  Small business owners like Priya (Boutique owner) or Fatima (Food Cart operator) often struggle with managing inventory across multiple locations (e.g., physical storefront, online store, pop-up markets). When internet connectivity drops (like at a crowded festival or a rural location), they cannot reliably check stock levels or update inventory after a sale. Current platforms (Shopify POS, Square) heavily rely on continuous internet connection for inventory management, causing friction, overselling, and lost revenue when offline. Non-technical users need an invisible system that allows them to sell offline seamlessly, automatically reconciling inventory across all locations when connectivity is restored, without requiring manual intervention.

  ## Research Report
  *   **Current Architecture Limits:** OHC's current inventory management assumes continuous connectivity for stock checks and deductions, risking overselling or failed checkouts during network drops.
  *   **Competitor Analysis:**
      *   *Shopify POS / Square:* While they offer basic offline payment processing, full multi-location inventory synchronization and real-time conflict resolution when offline are complex or heavily reliant on the cloud.
      *   *Wix:* Weak offline capabilities for complex inventory scenarios.
  *   **Discovery:** OHC requires an "Autonomous Offline-First Multi-Location Inventory Sync Engine." This engine must utilize a local, encrypted event queue on the mobile device to track inventory changes (deductions, additions). When the device reconnects, the "Operations Agent" must intelligently reconcile these local events with the cloud state, using conflict-free replicated data types (CRDTs) or a robust event-sourcing model to prevent overselling, even if simultaneous online sales occurred.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      MOBILE-APP ||--o{ LOCAL-EVENT-QUEUE : "Records Offline Inventory Actions"
      MOBILE-APP ||--o{ LOCAL-SQLITE-CACHE : "Reads Local Stock"
      LOCAL-EVENT-QUEUE }|--|| SYNC-ENGINE : "Batches Events on Reconnect"
      SYNC-ENGINE ||--o{ CLOUD-INVENTORY-LEDGER : "Reconciles State"
      CLOUD-INVENTORY-LEDGER ||--o{ OPERATIONS-AGENT : "Resolves Conflicts"
      OPERATIONS-AGENT ||--o{ NOTIFICATION-SERVICE : "Alerts Owner (if needed)"
  ```

  ### UI Wireframes & Mobile UX Flow (375px)
  *   **Inventory View (OHC Mobile App - 375px):**
      *   **Action:** Priya is at a pop-up shop, offline. She sells a dress and updates the inventory.
      *   **Offline Mode:** A small, reassuring Translucent Glass indicator: "Offline. Inventory changes saved locally."
      *   **Stock Update:** Priya taps a product to reduce stock. The UI updates instantly without lag.
      *   **Background Sync:** Upon reconnecting, the app silently syncs. If a conflict occurs (e.g., the last dress was also sold online), the Operations Agent triggers a simple 1-tap resolution prompt: "Online order sold out [Item X]. Would you like to refund the online order or cancel the offline sale?"

  ### Key Design Decisions
  *   **Local-First Architecture:** The mobile app must treat the local SQLite database as the source of truth for immediate reads/writes to ensure zero latency, syncing asynchronously with the cloud.
  *   **Event Sourcing for Reconcilliation:** Instead of overwriting final stock counts, the system must record *events* (e.g., `StockReducedBy: 1`). The Operations Agent replays these events to calculate the true final state.
  *   **Zero Trust & Multi-Tenancy:** The sync engine must strictly authenticate via SPIFFE/SPIRE, ensuring inventory data never bleeds across `tenant_id` boundaries during batch synchronization.

  ### AI Agent Integration Points
  *   **Operations Agent:** Autonomously handles the background merging of offline and online inventory events. If a negative stock situation arises due to simultaneous offline/online sales, it drafts a resolution plan for the owner's 1-tap approval.

  ## Implementation Prompt
  Implement the Autonomous Offline-First Multi-Location Inventory Synchronization Engine.
  *   **Acceptance Criteria 1 (Local Queuing):** The mobile application must accurately record inventory changes (additions/deductions) to an encrypted local queue when network connectivity is lost.
  *   **Acceptance Criteria 2 (Seamless Sync):** Upon network restoration, the system must automatically and securely (via SPIFFE/SPIRE) sync the local event queue to the central `CLOUD-INVENTORY-LEDGER` without manual user intervention.
  *   **Acceptance Criteria 3 (Conflict Resolution):** The `OPERATIONS-AGENT` must accurately process concurrent offline and online inventory events. In the event of overselling, it must generate a plain-language, 1-tap resolution prompt for the business owner via the mobile dashboard.
  *   **Acceptance Criteria 4 (Multi-Tenant Isolation):** All database interactions, both local and cloud, must enforce strict multi-tenant isolation using the `tenant_id`.

  ## Priority
  P0 (Critical) - Solving offline friction is a massive differentiator for OHC's target personas.

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
