issue_title: "Implement Offline-First Operations & Mobile POS Sync Resiliency"
issue_description: |
  ## Problem Statement
  For non-technical owner/operators running their businesses in low-connectivity environments (like Fatima operating a food cart in a crowded area, or Carlos on a remote field service route), network dropouts cause critical failures. Currently, OneHumanCorp (OHC) relies on continuous connectivity for POS transactions, inventory updates, and task completions. When the connection drops, critical operations halt, leading to lost sales, double bookings, and frustrated customers. A robust, offline-first sync architecture is needed to ensure uninterrupted operations and automatic reconciliation when connectivity is restored.

  ## Research Report
  - **Market Context**: Platforms like Shopify POS and Square offer limited offline capabilities, primarily queuing credit card transactions. However, they struggle with complex operational state (e.g., inventory reservations, task completion). Modern local-first architectures (like PowerSync or ElectricSQL) combined with CRDTs (Conflict-free Replicated Data Types) enable full operational capabilities without internet access.
  - **Codebase Findings**: The current OHC docker-compose stack provisions `powersync` for some database syncing, but the mobile POS and agentic workflows are strongly coupled to real-time `server` availability and Redis Redlock. The UI currently lacks optimistic updates for critical operations like inventory deduction or booking confirmation.
  - **Competitor Analysis**: Square allows offline card processing but warns of liability for declined cards. OHC's differentiation is leveraging AI agents to manage offline risk, e.g., an agent flagging high-risk offline transactions based on customer history, and intelligently resolving sync conflicts upon reconnection without bothering the owner.

  ## Design Doc
  ### Architecture Diagram (Mental Model / Description)
  - **Client Layer (Flutter PWA/Mobile)**: Utilizes a local SQLite database (via PowerSync client) for immediate read/writes. Implements Optimistic UI for immediate feedback.
  - **Sync Layer (PowerSync Service)**: Acts as the bridge, capturing offline mutations and continuously syncing with PostgreSQL when online.
  - **Conflict Resolution (AI Agent)**: A background worker (Operations Agent) monitors the sync queue. If double-booking or inventory over-drafts occur during offline reconciliation, the agent drafts a resolution (e.g., auto-refunding with an apology, or substituting a product) for the owner's review.
  - **PostgreSQL**: Remains the central ledger, tracking both authoritative state and sync lineage.

  ### Mobile UX Flow (375px First)
  1. Fatima takes an order in a dead zone. The network indicator on the top app bar smoothly transitions to a translucent "Offline Mode" chip (Premium Token library styling).
  2. She taps the POS checkout button (≥44x44px target). The app immediately updates the UI optimistically—deducting inventory locally and queuing the payment intent.
  3. A local success toast appears: "Order saved locally. Will sync when online."
  4. Upon network restoration, the sync indicator spins briefly. If a conflict occurs (e.g., another staff member sold the last item online), the Work Triage feed highlights an action item: "Sync Conflict: Order #123 needs attention."

  ### AI Agent Integration Points
  - **Operations Assistant**: Automatically resolves non-critical sync conflicts and escalates critical ones (like double-charged offline transactions) to the owner's triage feed with drafted solutions.

  ### Key Design Decisions
  - Adopt local-first SQLite via PowerSync rather than complex Redux/state-based queues, ensuring the entire app state is queryable locally.
  - Fall back from Redis distributed locks to optimistic local locks during offline mode, accepting eventual consistency.

  ## Implementation Prompt
  **Goal**: Implement local-first offline syncing for POS and order creation using PowerSync and Optimistic UI.
  **CUJ**: A user (Fatima) logs in, loses internet connection, successfully adds an item to an order, and completes the local checkout flow. Once the connection is restored, the order syncs to the backend database, and the Operations Agent verifies inventory levels.
  **Acceptance Criteria**:
  - The Flutter UI must correctly handle offline states gracefully with an "Offline" indicator.
  - The `create_order` action must write to the local PowerSync SQLite db and update the UI immediately, with zero loading spinners waiting for network.
  - Once online, the backend must receive the order without data loss.
  - MUST include E2E Playwright tests verifying the offline-to-online transition and order creation behavior.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
