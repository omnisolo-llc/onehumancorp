issue_title: "[Research] Dynamic Inventory and Multi-Location Sync Capabilities"
issue_description: |
  # Architecture Deep Dive: Real-Time Dynamic Inventory & Multi-Location Sync

  ## Problem Statement
  Small business owners like Priya (Boutique Operator) and Jun (Location Manager) face significant challenges managing inventory across multiple sales channels (online store, Instagram, physical location) and potentially multiple physical sites. Traditional platforms often struggle with real-time sync, leading to overselling or manual reconciliation work. OHC needs a robust, multi-tenant inventory architecture that handles concurrent transactions, offline scenarios, and multi-location management gracefully without requiring technical expertise from the owner.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Shopify:** Excellent inventory management, but often requires third-party apps for complex multi-location routing or offline point-of-sale synchronization.
  - **Square:** Strong offline-first POS integration, but can be rigid when integrating with custom or external e-commerce fronts.
  - **OHC Opportunity:** Implement an edge-cached, CRDT-based (Conflict-free Replicated Data Type) or strong distributed locking mechanism for inventory that supports offline-first mobile apps and instant online syncing. The AI agent (Operations Assistant) should proactively suggest reordering, transferring stock between locations, or updating the online storefront based on predictive trends.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Mobile App - Offline First] -->|Sync when Online| B(Edge Gateway / GraphQL)
      C[Online Storefront] -->|GraphQL| B
      D[Instagram/Social Commerce] -->|Webhook/API| B
      B --> E{Inventory Conflict Resolution Service}
      E -->|Distributed Lock / CRDT| F[PostgreSQL - Multi-Tenant Inventory Ledge]
      F --> G[Event Bus]
      G --> H[Operations Assistant Agent]
      H -->|Proactive Alerts| I[Agent Feed - Mobile App]
      H -->|Auto-Reorder Drafts| I
  ```
  ### Mobile UX Flow (375px)
  1. **Home Feed:** Owner sees an alert: "Low stock on Blue Summer Dress at Downtown Location. 3 remaining."
  2. **Action Card:** Suggests transferring stock from Warehouse or reordering from Supplier.
  3. **One-Tap Action:** "Approve Transfer" or "Draft Reorder Email".
  4. **Offline Mode:** If offline, inventory deductions from in-store sales are cached locally and synced instantly upon reconnection, with conflict resolution handled server-side.

  ### AI Agent Integration
  - **Operations Assistant:** Monitors the event bus for inventory thresholds. Predicts demand based on past sales velocity and seasonality. Drafts supply requests or internal transfer orders.

  ## Implementation Prompt
  Implement the core backend services for the multi-location inventory ledger. This must include:
  - Database schema for `locations`, `inventory_items`, and `inventory_transactions` with strict `tenant_id` isolation (RLS).
  - A robust API service to handle stock adjustments, preventing race conditions during concurrent sales (using PostgreSQL `SELECT ... FOR UPDATE` or Redis locks).
  - Integration with the Event Bus to emit `InventoryLowEvent`.
  - A stub for the Operations Assistant to consume this event and create an actionable feed item.
  - Ensure the API handles idempotency for mobile clients that might retry requests due to poor connectivity.

  ## Priority: P1
  ## Estimated Scope: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
