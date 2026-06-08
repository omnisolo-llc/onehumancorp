issue_title: "[Architecture] Offline-First Multi-Location Inventory & Supply Sync"
issue_description: |
  ## Problem Statement
  Small business operators with physical goods (like Priya with her boutique or Fatima with her food cart) need reliable inventory and menu availability toggles. Network connectivity in retail locations, food carts, or basements is often flaky or non-existent. Current basic inventory systems force the owner to wait for a spinning loader to update a stock count. If the network drops, the update fails, and the owner loses track of stock, leading to overselling or lost sales. Furthermore, managers of multi-location operations (like Jun) need to ensure stock levels are eventually consistent across locations and the central database, without interrupting the local checkout or restock workflow.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Shopify POS:** Offers offline capabilities for checkout, but inventory syncing can be delayed. It requires a robust connection for real-time stock updates across channels.
  - **Square:** Good offline mode for payments, but inventory management often requires a connection to reflect changes online or at other locations accurately.
  - **Odoo/ERPNext:** Powerful multi-location inventory but far too complex for a solopreneur or small location manager to configure and use quickly on a mobile device.
  - **OHC Opportunity:** Implement an "Optimistic Offline-First Inventory Engine" combined with an AI Operations Assistant. The owner updates stock on their 375px mobile screen. The app immediately reflects the change locally (optimistic UI) and queues the mutation. When the network returns, the sync daemon resolves the queue. If there's a conflict (e.g., the same item was sold online simultaneously), the AI Operations Assistant drafts an "Inventory Conflict Resolution" card for the owner's feed, rather than just showing a generic error.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Mobile App - Flutter] -->|Optimistic Update| B(Local SQLite Cache)
      A -->|Queue Mutation| C(Local Event Queue)
      C -.->|Network Restored| D[API Gateway]
      D --> E{Conflict Resolution Engine}
      E -->|No Conflict| F[PostgreSQL - Unified Ledger]
      E -->|Conflict Detected| G[Operations Assistant AI]
      G -->|Draft Resolution| H[Owner Feed]
      H -.->|User Approves| F
      B -.->|Sync| A
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **Inventory Screen (Mobile):** A clean, un-cluttered list of items with their current stock levels. Large, touch-friendly "+" and "-" buttons for quick adjustments.
  - **Offline State:** A subtle indicator (e.g., a small slashed-cloud icon in the header) shows offline status. The user taps "+" to add stock. The number updates instantly. A small badge indicates "Syncing..." or "Pending".
  - **Conflict Resolution (Home Feed):** If a conflict occurs upon reconnect (e.g., stock went negative due to concurrent online sales), a card appears in the Work Triage feed: "Inventory Mismatch: 2 Chocolate Cakes sold online while you were offline. Current stock is -1. Do you want to restock or cancel the online order?" with clear 1-tap action buttons.
  - **Visual Design:** Use the OHC Premium Token library with translucent glass styling for the resolution cards. Keep the inventory list highly legible with strong typography.

  ### AI Agent Integration Points
  - **Operations Assistant:** Monitors the sync queue. If a constraint violation occurs (e.g., inventory drops below zero) during offline reconciliation, the agent intercepts the error, queries the relevant context (what sold where), and drafts a human-readable decision prompt for the owner, avoiding technical error messages like "Constraint Violation on table inventory_items".

  ### Key Design Decisions
  - **Local-First Architecture:** The source of truth for the UI is always the local SQLite database. The network is just a synchronization channel.
  - **Eventual Consistency with AI Mediation:** We accept eventual consistency. Conflicts are treated as business operations problems to be solved by the Operations Assistant, not just technical errors to be dumped in a log.
  - **Tenant Isolation:** All queued events and local data must be strictly scoped to the `tenant_id` and potentially `location_id`.

  ## Implementation Prompt
  **User-Facing Outcome:** As a business operator, I can update my inventory counts instantly on my phone, even if I'm in a dead zone. The app never makes me wait for a network request. If an issue arises when I reconnect, my assistant explains it clearly and gives me options to fix it.
  **CUJ & Acceptance Criteria:**
  1.  Implement a local SQLite-backed event queue and inventory cache in the Flutter frontend or a corresponding local sidecar.
  2.  Create API endpoints (gRPC/REST) in the Rust backend to accept batch inventory mutation events.
  3.  Implement a Conflict Resolution Engine in the backend that detects concurrent modifications.
  4.  Integrate the Operations Assistant to handle detected conflicts by generating a notification/task for the owner.
  5.  Provide Playwright E2E tests: A user logs in, toggles offline mode (simulated network drop), updates inventory, toggles online mode, and verifies the data syncs correctly. A second test should simulate a conflict and verify the generated AI resolution task appears in the feed.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
