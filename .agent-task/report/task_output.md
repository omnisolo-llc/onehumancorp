issue_title: "[architecture] Unified Offline-First Sync Architecture"
issue_description: |
  # Unified Offline-First Sync Architecture

  ## Problem Statement
  Small business owners often operate in environments with poor or intermittent internet connectivity (e.g., Fatima's food cart, Carlos in a client's basement, Maya doing a pop-up market). Current web and mobile architectures often assume constant connectivity, leading to data loss, stalled operations, and frustration when offline. They need a system that allows them to continue taking orders, booking appointments, and managing inventory seamlessly, synchronizing automatically when connectivity is restored.

  ## Research Report
  - **Market Context**: Most platforms (Shopify, Wix) degrade gracefully but lose core transactional capabilities (like accepting a payment or recording an order) when offline. Specialized POS systems (Square) handle offline payments but often lack deep integration with the full online suite.
  - **User Needs**:
    - **Fatima (Food Cart)**: Must be able to view pre-orders and mark them complete even if her 4G connection drops.
    - **Carlos (Handyman)**: Needs to draft quotes and log completed work while deep inside a building with no signal.
    - **Priya (Boutique)**: Needs her POS to continue processing sales (queuing them) if the store's Wi-Fi goes down.
  - **Technical Gap**: OHC currently lacks a unified, multi-tenant offline-first sync engine that guarantees eventual consistency without burdening the user with manual sync actions or conflict resolution.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
    subgraph Mobile Client (375px First)
      UI[User Interface] --> LocalDB[(Local SQLite/Riverpod/Zustand)]
      LocalDB --> SyncManager[Sync Engine Manager]
      SyncManager --> ActionQueue[(Action/Mutation Queue)]
    end

    subgraph OHC Backend
      Gateway[API Gateway] --> ConflictResolver[Conflict Resolution Engine]
      ConflictResolver --> DB[(PostgreSQL)]
      ConflictResolver --> AI[AI Advisory Engine - Flags Anomalies]
    end

    ActionQueue -- Network Available --> Gateway
    Gateway -- Sync Delta --> LocalDB
  ```

  ### Key Design Decisions
  1.  **Local-First Data Model**: The mobile app always reads from and writes to a local database (e.g., SQLite via drift in Flutter or IndexedDB in PWA).
  2.  **Optimistic UI Updates**: The UI instantly reflects local changes (e.g., "Order marked complete"), providing immediate feedback.
  3.  **Mutation Queue & Sync Protocol**: All offline actions are queued locally. When connectivity returns, the Sync Engine replays mutations to the backend. The backend uses logical timestamps (or CRDTs if appropriate) to resolve conflicts, favoring the most recent user intent.
  4.  **Tenant Isolation**: All sync payloads are strictly scoped by `tenant_id`.

  ### Mobile UX Flow (375px)
  - **Status Indicator**: A subtle, non-intrusive indicator (e.g., a small cloud icon with a line through it) appears when offline, using translucent glassmorphism.
  - **Seamless Interaction**: The user interacts with the app normally. Buttons like "Complete Order" or "Save Quote" work instantly.
  - **Sync Feedback**: When back online, the indicator changes to a spinning sync icon briefly, then disappears. If a conflict occurs that requires manual intervention (rare), an AI Assistant notification appears: "I noticed a conflict with inventory while you were offline. I've adjusted the stock, click here to review."

  ## Implementation Prompt
  **Task**: Implement the Unified Offline-First Sync Architecture.
  **CUJ**: A user (like Carlos) opens the app, loses network connectivity, drafts and saves a new quote for a client, and marks a previous job as "completed". The app must instantly reflect these changes locally. Once network connectivity is restored, the app must automatically sync these changes to the backend without manual intervention, and the backend must update the central database.
  **Acceptance Criteria**:
  - Local database layer implemented to store core entities (e.g., Quotes, Orders).
  - Action queue implemented to store pending mutations.
  - Sync engine implemented to automatically push queued mutations when online.
  - Backend conflict resolution mechanism handles incoming sync payloads securely (tenant isolated).
  - 100% Unit Test coverage for the sync logic.
  - Playwright/E2E test verifying the offline-then-online CUJ flow.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
