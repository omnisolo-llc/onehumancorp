issue_title: "Architectural Design: Offline-First Mobile Point-of-Sale (mPOS) & CRDT State Reconciliation Engine"
issue_description: |
  ### Problem Statement
  Small business owners and operators like **Fatima (Food Cart Operator)** and **Carlos (Field Service Owner)** frequently operate in environments with poor or intermittent internet connectivity (e.g., street corners, basements, or rural job sites). The current OHC mobile architecture assumes a relatively stable connection to process bookings, orders, and state mutations. When connectivity drops, critical operations—such as accepting a pre-order, marking a service as complete, or taking a local payment—can fail or enter an inconsistent state, leading to lost revenue, delayed operations, and diminished trust in the platform. A true work assistant must function reliably offline and resolve state gracefully when reconnected, without requiring technical intervention from the owner.

  ### Research Report
  - **Competitive Analysis:** Leading operator platforms like Square and Shopify POS have deeply ingrained offline modes. Square allows offline swiping (with risk limits) and Shopify queues cart/order states locally until reconnection. However, they are point solutions (payments/ecom) rather than a unified owner assistant.
  - **Codebase Findings:** The current repository includes a robust backend (`src/server/db/migrations`) with some offline-related tables (e.g., `021_pos_offline_sync.sql`, `131_c_pos_offline_transactions_sync_status.sql`, `134_b_crdt_deltas.sql`), indicating the foundation for offline sync exists. However, there is a gap in a comprehensive, edge-to-cloud CRDT (Conflict-free Replicated Data Type) reconciliation architecture that seamlessly merges disconnected actions across the Flutter mobile client, backend database, and AI agent coordination.
  - **The Gap:** The mobile client (Flutter) needs a structured local-first data layer (e.g., using a local SQLite instance) that records intents and mutations as CRDT operations, paired with a robust background synchronization engine on the Go/Rust backend that handles multi-tenant conflict resolution without blocking the user.

  ### Design Doc
  #### 1. Architecture Diagram
  ```mermaid
  graph TD
      A[Flutter Mobile Client - 375px] --> B(Local SQLite Store)
      B --> C{Connectivity Monitor}
      C -- Offline --> B
      C -- Online --> D[Sync Queue Manager]
      D -->|gRPC / REST Sync API| E[OHC API Gateway]
      E --> F[CRDT Reconciliation Engine]
      F --> G[(PostgreSQL - Multi-tenant)]
      F --> H[AI Operations Assistant]
      H -->|Audit/Resolve| F
  ```

  #### 2. Mobile UX Flow (375px First)
  - **State Indicator:** A subtle, translucent pill at the top of the screen (using OHC Premium Tokens) indicates "Working Offline" when the connection drops.
  - **Action Permissibility:** Core actions (e.g., "Complete Job", "Take Cash Payment", "Add Note") remain fully interactive. The buttons visually register clicks and transition to a truthful "Saved to device" state.
  - **Queue Visibility:** An owner-friendly "Pending Sync" list is accessible via the dashboard, showing actions waiting to be pushed to the server. No developer jargon (like "CRDT" or "Queue") is visible—only clear statements like "3 orders waiting for signal".
  - **Reconnection:** Upon reconnection, the pill transitions to "Syncing..." and then disappears. A daily plain-language summary from the AI Operations Assistant highlights any resolved conflicts (e.g., "Two staff members marked order #124 complete; I logged the first timestamp").

  #### 3. AI Agent Integration Points
  - **Operations Assistant:** When a sync event detects a merge conflict that cannot be deterministically resolved (e.g., an inventory item oversold due to offline sales), the Operations Assistant intercepts the conflict and drafts an actionable summary for the owner.
  - **Customer Assistant:** Automatically queues follow-up messages (e.g., receipt delivery) while offline, triggering dispatch only when the backend successfully persists the transaction.

  #### 4. Key Design Decisions
  - **Local-First Writes:** The mobile client will treat the local database as the source of truth for read/write operations, making UI state instantly responsive.
  - **Vector Clocks / CRDTs:** The backend will implement a CRDT-based resolver for specific tables (like inventory counts or order statuses) to ensure eventual consistency across multiple offline devices (e.g., Jun managing multiple staff devices in one location).
  - **Idempotency Keys:** Every offline action must generate a unique UUIDv7 on the client, ensuring that duplicate sync requests (due to flaky reconnections) are safely ignored by the backend.

  ### Implementation Prompt
  **Goal:** Implement the end-to-end Offline-First State Engine.
  **CUJ:** Fatima is at her food cart, loses cellular data, marks three pre-orders as "Picked Up", and creates two new cash sales. Her device regains connection an hour later. The changes seamlessly sync, inventory is updated, and the backend triggers receipts.
  **Acceptance Criteria:**
  1. The Flutter mobile shell implements a local persistence layer for offline mutations, generating unique client-side idempotency keys.
  2. The UI provides truthful visual feedback (glassmorphism pill) for offline state and pending syncs, passing the "grandmother test."
  3. The backend API handles batch sync payloads, applying CRDT conflict resolution rules to ensure data integrity without throwing blocking 500 errors to the client.
  4. The AI Operations Assistant produces a unified natural-language summary if complex conflicts (like overselling) occur during the sync.
  5. 100% E2E Playwright coverage simulating an offline-to-online transition scenario.

  ### Priority and Scope
  - **Priority:** P1 (Critical for operational continuity of physical businesses)
  - **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []