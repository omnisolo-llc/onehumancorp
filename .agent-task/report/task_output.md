issue_title: "Implement Offline-First Mobile POS & Optimistic Mutation Engine"
issue_description: |
  # Research Report: Offline-First Mobile POS & Optimistic Mutation Engine

  ## 1. Problem Statement
  Small business owners operating in challenging network conditions—such as Fatima managing her food cart in areas with patchy 4G, or Carlos repairing plumbing in a customer's basement with zero reception—face critical operational friction when their point-of-sale (POS), booking, or inventory management tools fail offline. Existing solutions either prevent actions completely without a connection or show generic error states, leading to lost sales and poor customer experiences. The owner needs a robust, offline-first mobile architecture that allows the business to function seamlessly when disconnected, capturing orders, taking cash/offline payments, and securely syncing state back to the OHC backend once connectivity is restored.

  ## 2. Research Report
  - **Competitive Analysis**:
    - **Square POS:** The industry leader in offline mode. Square allows merchants to process cash and offline card payments seamlessly when the network drops. Transactions are queued locally and synced later.
    - **Shopify POS:** Offers offline capabilities primarily for cash. Card payments require an active connection. Inventory is cached locally, but true optimistic sync for complex catalog updates is limited.
    - **Wix/Squarespace:** Primarily online-dependent.
  - **Market Gap in OHC**: Currently, OHC requires a stable connection for POS transactions and inventory mutations. To capture the micro-SME market (food carts, field services), OHC must treat offline resilience not as a bolt-on feature, but as a core architectural primitive.
  - **Technical Context**: OHC uses PostgreSQL on the backend. We can leverage local CRDTs (Conflict-Free Replicated Data Types) or a robust local action queue (Local-First architecture) on the Flutter/PWA client to guarantee that users can manage inventory and process local cash operations offline.

  ## 3. Design Doc

  ### Architecture Overview (Mermaid)
  ```mermaid
  graph TD
    A[Mobile Client] -->|Action| B(Local SQLite/CRDT Store)
    B -->|Queue| C{Optimistic Mutation Engine}
    C -->|Online?| D(Cloud Gateway API)
    D --> E[(Cloud PostgreSQL)]
    C -.->|Offline| F[Local Action Queue]
    F -.->|Reconnects| D
    E -.->|Sync| B
    E --> G[AI Operations Agent]
    G -.->|Conflict Resolution| E
  ```

  ### Mobile UX Flow (375px)
  1. **Network Indicator**: A subtle, premium glassmorphism pill at the top of the dashboard. When online, it's hidden. When offline, it gracefully slides down showing "Offline Mode" in a muted amber color.
  2. **Active State**: The user continues to navigate the catalog, add items to the cart, and process cash payments exactly as they would online.
  3. **Visual Cues**: Pending synced items (like orders or inventory changes) show a small, unobtrusive "syncing" icon next to them.
  4. **AI-Assisted Resolution**: If a conflict occurs upon reconnection (e.g., overselling an item), the Operations AI Agent intercepts it, handles the fallback (e.g., drafting an apology to the online buyer), and notifies the owner via the Business Advisory department.

  ### AI Agent Integration Points
  - **Conflict Resolution**: The AI Agent automatically resolves state drift issues when the app reconnects, preventing the user from needing to manually reconcile complex database conflicts.

  ### Key Design Decisions
  - **Local-First Database**: Use SQLite with a CRDT-based sync mechanism or a structured mutation queue on the mobile client.
  - **Optimistic UI**: The UI must update immediately upon user action, assuming success, to provide a snappy experience regardless of network speed.

  ## 4. Implementation Prompt
  **Goal**: Implement an Offline-First Optimistic Mutation Engine for the OHC POS module.

  **Frontend (Flutter/PWA)**:
  - Implement a local data store (e.g., SQLite) to cache catalog and order state.
  - Build a mutation queue that intercepts API calls when offline and stores them locally.
  - Implement the "Offline Mode" network indicator pill in the UI.
  - Ensure the UI optimistically updates for operations like adding to cart and processing cash payments.

  **Backend**:
  - Expose a sync endpoint that accepts batches of queued mutations from the client.
  - Implement deterministic conflict resolution logic (or hook into the AI Operations Agent for complex conflicts).
  - Ensure strict tenant data isolation during the sync process.

  **Acceptance Criteria**:
  - User can disconnect from the internet, process a cash transaction, and see the order locally.
  - Upon reconnection, the transaction syncs to the backend automatically without user intervention.
  - The UI accurately reflects network status and pending sync counts.
  - E2E Playwright test simulating offline mode, action execution, and subsequent online sync passes.

  ## 5. Scope & Priority
  - **Priority**: P0 (Critical for specific personas like Fatima and Carlos).
  - **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
