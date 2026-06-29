issue_title: "Implement Offline-First Mobile POS Sync & Eventual Consistency Engine"
issue_description: |
  # Research Report: Offline-First Mobile POS Sync & Eventual Consistency Engine

  ## Problem Statement
  Currently, the OHC mobile application requires a continuous and stable internet connection to process Point-of-Sale (POS) transactions. This is a critical point of failure for our core owner/operator personas, specifically **Fatima (food cart operator)** and **Carlos (field service owner)**. Fatima operates in areas with slow mobile data, and Carlos frequently works in rural locations with intermittent or no connectivity. When network connections drop or flap, they are unable to create orders, update inventory, or accept deferred payments, severely impacting their daily operations and causing them to lose trust in the OHC platform.

  The system requires an offline-tolerant write path and an eventual consistency engine that allows operators to confidently continue their work without connectivity, knowing the platform will invisibly and reliably synchronize the data when the connection is restored.

  ## Research Report & Competitor Analysis
  - **Square & Clover**: Lead the market in offline mode capabilities. Square allows processing of magnetic stripe and chip card payments offline, storing the encrypted data locally and syncing automatically when reconnected (usually within 24 hours).
  - **Shopify POS**: Offers an offline mode for cash transactions and custom payment types, but offline credit card processing is more restricted compared to Square.
  - **Modern PWA/Mobile Patterns (Linear, Notion)**: Use robust local-first architectures (e.g., RxDB, WatermelonDB, or custom IndexedDB/SQLite sync engines) combined with optimistic UI updates. This allows the user to feel immediate responses to actions (like creating a task or order) regardless of network status.
  - **The OHC Opportunity**: While Square provides the payment rail, OHC must provide the *agentic workflow* rail. When an offline transaction is recorded, OHC needs to not just sync the ledger, but trigger the AI Operations and Finance agents to retroactively process the context, update inventory, and draft any missed communications once back online.

  ## Design Doc (Architecture)

  ### System Architecture
  - **Local Persistence Layer (Flutter/Mobile)**: Implement a robust local database (e.g., Isar or sqflite) on the mobile client. All POS order creations, inventory deductions, and payment intent recordings are written here first.
  - **Sync Queue Mechanism**: An offline-tolerant background queue that monitors network state. When online, it processes the local queue sequentially to the `server/api/sync` endpoint.
  - **Backend Eventual Consistency Engine (Go/PostgreSQL)**: The API layer receives the batched sync payload. It processes the operations against the Central Ledger using the recorded timestamps.
  - **Conflict Resolution (Operations Agent)**: If an offline POS inventory deduction conflicts with an online sale that occurred during the disconnected period (oversell), the Operations Agent is triggered. It does not block the sync; instead, it accepts the ledger state and generates a "Sync Anomaly Report" for the owner, suggesting next steps (e.g., draft an apology/refund or generate a backorder).

  ### AI Agent Integration Points
  - **Operations Agent ("The Manager")**: Monitors the sync endpoint for delayed payloads. Evaluates sync conflicts (like inventory oversells) and prepares resolution proposals for the owner's feed.
  - **Customer Success Agent ("The Ambassador")**: If an offline sale requires a receipt or follow-up email, the agent queues the communication and fires it immediately upon successful backend sync.

  ### Mobile UX Flow (375px First)
  1. The user (Fatima) opens the OHC app; a subtle status pill at the top indicates "Network Offline - Working Locally".
  2. She adds items to the cart and taps "Checkout". The UI responds instantaneously, showing a successful order screen.
  3. A small queue indicator (e.g., "1 pending sync") appears.
  4. When the connection returns, the indicator spins briefly, then disappears. The Operations Agent drops a notification if any sync anomalies require attention.

  ## Implementation Prompt
  **Target Outcome**: Implement the local-first sync architecture for the Mobile POS flow, allowing users to create orders and update local inventory while completely disconnected from the network.

  **Critical User Journey (CUJ)**:
  1. Login as an operator (e.g., using Fatima's profile).
  2. Disconnect the device/browser from the internet (simulate offline mode).
  3. Create a new POS order and complete the local checkout flow.
  4. Verify the UI reflects a successful order creation and updates the local inventory count instantly, showing an offline pending status.
  5. Reconnect to the internet.
  6. Verify the app automatically syncs the pending order to the backend.
  7. Verify the backend successfully persists the order to PostgreSQL and triggers the AI agent pipeline for subsequent processing.

  **Acceptance Criteria**:
  - The POS creation flow must succeed while offline, persisting to a local store.
  - The UI must optimistically update and clearly indicate offline/pending status.
  - The background sync engine must automatically push queued actions to the Go backend upon reconnection.
  - The backend must handle retroactive timestamps and trigger the appropriate AI agents without failing the sync.
  - E2E Playwright tests must simulate the offline/online network states and verify the complete synchronization lifecycle.
  - Unit tests for the local store, sync queue, and Go backend sync handler must achieve 100% coverage.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
