issue_title: "[Architecture] Offline-First AI Operations Assistant & Edge Sync Engine"
issue_description: |
  # Mission Queue Protocol: Offline-First AI Operations Assistant & Edge Sync Engine

  ## 1. Problem Statement (Track 1)
  Business owners like **Fatima (food cart operator)** and **Carlos (field service owner)** operate in environments with highly unreliable cellular networks (e.g., crowded events, basements, remote sites). Currently, the OHC architecture relies on constant synchronous connections to the cloud PostgreSQL database and AI agent queue. When connectivity drops, workers are unable to process payments, update task statuses, or receive AI suggestions, leading to lost revenue and operational paralysis.

  ## 2. Research Report
  Our competitive analysis of Square, Toast, and Shopify POS reveals that offline resilience is a non-negotiable feature for physical-world operators. However, these systems only offer offline *payments* (storing and forwarding transactions). OHC's differentiation is the **AI Assistant**, which currently degrades completely offline. We need an architecture that pushes critical operations agent logic (e.g., triage rules, catalog constraints, basic scheduling) and optimistic data mutation to the edge (the mobile device).

  ## 3. Design Doc (Track 2 & Track 3)

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant User as Mobile App (Fatima)
      participant LocalCache as Local Edge Store (SQLite/Isolate)
      participant LocalAgent as On-Device Agent Model
      participant SyncEngine as Background Sync Daemon
      participant CloudAPI as OHC Cloud API
      participant PG as Central PostgreSQL

      User->>LocalCache: Record Order & Process Payment (Offline)
      LocalCache-->>User: Optimistic Success (UI updates instantly)
      User->>LocalAgent: Query: "Do we have enough rice left?"
      LocalAgent-->>User: AI Estimate based on local cache
      Note over SyncEngine: Network Restored
      SyncEngine->>LocalCache: Fetch pending mutations
      SyncEngine->>CloudAPI: POST /api/v1/sync (Idempotent)
      CloudAPI->>PG: Reconcile Ledger & Inventory
      CloudAPI-->>SyncEngine: ACK & State Diff
      SyncEngine->>LocalCache: Update local state with truth
  ```

  ### Mobile UX Flow
  1. **Connectivity Indicator**: The 375px top app bar subtly shifts from translucent white to a translucent amber token indicating "Offline Mode - Safe to work."
  2. **Optimistic Mutation**: Fatima taps "Checkout" for a $12 order. The button shows a fast local spinner, then instantly turns into a green "Done" state.
  3. **Local AI Assistance**: The AI Agent tab remains accessible. Fatima types "low on napkins". The on-device LocalAgent tags this as a high-priority supply task and queues it for cloud escalation.
  4. **Background Sync**: When LTE returns, a non-intrusive toast notification reads "3 offline actions synced."

  ### AI Agent Integration Points
  - **Operations Agent (Cloud)**: Receives the delayed sync payload, recalculates global inventory, and if oversold, triggers a priority alert to the owner.
  - **On-Device Agent**: A quantized, specialized LLM/rule-engine running on the mobile client (via Flutter local execution/TFLite) that provides basic operational triage and draft responses while disconnected.

  ### Key Design Decisions
  - **Event Sourcing at the Edge**: Instead of syncing database rows, the mobile client queues discrete operational *events* (e.g., `OrderPlaced`, `TaskCompleted`). This prevents complex merge conflicts in PostgreSQL.
  - **Idempotency Keys**: Every offline event receives a client-generated UUID to guarantee exactly-once processing when the sync daemon eventually reaches the OHC API.

  ## 4. Implementation Prompt
  **Role:** Implementer Agent

  **Objective:** Implement the Offline-First Edge Sync Engine for the Flutter mobile client and the Go backend API.

  **CUJ (Acceptance Criteria):**
  1. Launch the OHC app and log in as a physical-world operator (e.g., Food Cart).
  2. Simulate network disconnect (airplane mode).
  3. Create a new customer order and complete it via tap-to-pay. The UI MUST optimistically succeed and show the new order in the "Today's Work" feed.
  4. Re-enable the network.
  5. The background Sync Daemon must automatically detect connectivity, push the `OrderPlaced` event to the backend, and receive a successful ACK without user intervention.

  **Constraints:**
  - Do NOT prescribe the exact local database (SQLite/Hive/Isar) — choose the most appropriate for Flutter.
  - The backend sync endpoint must be strictly idempotent and validate multi-tenant isolation (tenant_id).
  - Provide full unit and Playwright E2E test coverage simulating the offline-to-online transition.

  ## 5. Priority & Scope
  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
