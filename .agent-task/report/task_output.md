issue_title: "Implement Offline-Tolerant Agentic POS & Order Sync Architecture"
issue_description: |
  # Research Report: Offline-Tolerant Mobile-First Agentic Point of Sale (POS) & Order Sync Architecture

  ## Problem Statement
  For non-technical owners like Fatima (Food Cart Operator) or Carlos (Field Service Owner), constant high-speed internet is an unreliable luxury. They operate in areas with spotty 4G coverage or degraded networks. Currently, cloud-reliant SaaS platforms force them to experience slow loading times, missed pre-orders, and stalled payment processing when offline. Furthermore, existing POS and order management systems require manual intervention for simple tasks (like toggling sold-out items or sending pickup notifications), increasing operational friction.

  We need a capability that guarantees uninterrupted order intake, local POS operations, and background agentic workflows (like language translation for Fatima's diverse customer base or auto-toggling sold-out statuses based on local inventory counting) that seamlessly sync when connectivity is restored, all functioning within a 375px mobile viewport.

  ## Research Report & Competitive Analysis
  - **Shopify POS / Square**: Provide robust offline payments and limited offline cart building, but they fail to incorporate AI agents to automate inventory and customer interactions while offline. Their setups remain complex for micro-SMBs.
  - **Toast / Clover**: Focused on food service, but heavily reliant on localized server hardware or strong connectivity for advanced features. They lack autonomous multi-lingual agents for pre-order triage.
  - **OHC Opportunity**: Introduce a "Local-First Agentic Sync" architecture. By leveraging local databases (e.g., SQLite via PowerSync/ElectricSQL) on the Flutter frontend, we can ensure instant UI responsiveness (0ms latency for CRUD). Background sync mechanisms will reconcile state with the cloud. Agentic tasks (like translating a customer's pre-order or updating inventory) are queued locally and executed by the backend asynchronously when synced, or by localized smaller models if device capabilities permit.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  architecture-beta
      group Client(Mobile App - Flutter)
      service UI(375px Mobile UI) in Client
      service LocalDB(Local SQLite / PowerSync) in Client
      service SyncEngine(Sync Engine) in Client

      group Cloud(OHC Backend - Go/Bazel)
      service API(gRPC / REST Gateway) in Cloud
      service Postgres(PostgreSQL Multi-Tenant) in Cloud
      service AgentQueue(AI Job Queue - SKIP LOCKED) in Cloud
      service Agents(LLM Agent Workers) in Cloud

      UI -- LocalDB
      LocalDB -- SyncEngine
      SyncEngine -- API : Bi-directional Sync
      API -- Postgres
      Postgres -- AgentQueue : Trigger Async Task
      AgentQueue -- Agents : Process
  ```

  ### Mobile UX Flow (375px First)
  1. **Order Intake (Offline Mode)**: The home screen displays an "Order List" (clean Ubiquiti-style cards). If offline, an amber "Syncing Paused" translucent badge appears at the top.
  2. **Fast Toggles**: Fatima taps a menu item to instantly toggle it "Sold Out". The local DB updates immediately (0ms UI latency).
  3. **Auto-Recovery**: When connectivity resumes, the Sync Engine pushes the "Sold Out" state. The Cloud AgentQueue detects the state change and automatically triggers the Customer Agent to send SMS updates to any pending pre-orders that included the sold-out item.
  4. **Multi-lingual Support**: Orders received in different languages are instantly translated by the backend Agent before being synced down to Fatima's device in her preferred language.

  ### AI Agent Integration Points
  - **Operations Agent**: Monitors the sync queue. If an inventory conflict arises (e.g., sold out locally but ordered online simultaneously), the agent intelligently resolves it by offering the online customer an alternative or a refund, drafting the SMS automatically.
  - **Customer Assistant**: Listens to the local POS events and queues follow-up review requests to be sent once connectivity is restored.

  ### Key Design Decisions
  - **Local-First Persistence**: UI reads/writes exclusively to a local SQLite store to guarantee offline tolerance and instant touch responses.
  - **Conflict-Free Replicated Data Types (CRDTs) or Timestamp/Version Vectors**: Used for robust sync resolution.
  - **Zero Trust Multi-Tenancy**: Sync engines must strictly authenticate via SPIFFE/SPIRE, and all synced data is partitioned by `tenant_id` at the edge and cloud.

  ## Implementation Prompt
  **User-Facing Outcome**: The user (Fatima) can open the OHC app, view orders, and toggle inventory availability instantly, even when completely offline. When reconnected, the system invisibly syncs her changes and triggers the Operations Agent to handle any conflicts or customer notifications.

  **CUJ for E2E Testing**:
  1. Log in as a food cart owner on a mobile viewport (375px).
  2. Simulate offline mode (disconnect network).
  3. Toggle an inventory item to "Sold Out".
  4. Create a manual POS order for an available item.
  5. Verify the UI updates instantly and displays an "Offline" indicator.
  6. Simulate online mode (restore network).
  7. Verify the sync completes successfully and the backend reflects the updated inventory and new order.

  **Acceptance Criteria**:
  - UI state relies on local persistence; no loading spinners for basic CRUD operations.
  - Mobile layout adheres strictly to 375px constraints.
  - AI job queue is successfully triggered by the sync event (verified via backend telemetry/logs).

  ## Priority & Scope
  - **Priority**: P1
  - **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
