issue_title: "[Architecture] Distributed Offline-First AI Sync Mesh for Field Service Operations"
issue_description: |
  # Research Report: Distributed Offline-First AI Sync Mesh

  ## Problem Statement
  Field service owners like Carlos (Handyman) and Fatima (Food Cart) often operate in environments with poor, flaky, or non-existent mobile data connections (e.g., in a customer's basement or in crowded market squares). Traditional web applications fail immediately without a network, blocking them from taking payments, updating job statuses, or adding notes to customer records. They need a system that feels instantaneous and reliable regardless of network state, while still allowing AI agents in the cloud to act on their data once connectivity is restored.

  ## Research & Competitive Analysis
  - **The "Spinning Wheel of Death"**: Standard PWA and hybrid apps (like early Shopify mobile POS) rely on optimistic UI updates combined with immediate REST/GraphQL calls. If the call fails, the UI errors out.
  - **Local-First Architectures (Linear, Notion, Superhuman)**: These platforms use an offline-first data model. Reads and writes happen against a local embedded database (SQLite/IndexedDB). Synchronization happens asynchronously in the background via CRDTs or event sourcing.
  - **PowerSync & WatermelonDB**: Industry-leading offline sync engines that manage background delta synchronization over WebSockets, allowing mobile clients to hold a synchronized subset of the PostgreSQL database.
  - **OHC Gap**: OHC currently assumes constant connectivity for AI agent operations. If Carlos finishes a job offline, the AI Operations Agent cannot draft his follow-up email or update his revenue metrics until he is online. We need an architecture that seamlessly buffers his offline actions and syncs them to the cloud to trigger agentic workflows.

  ## System Design & Architecture
  This architecture bridges the gap between a local-first mobile client and the cloud-native AI Agent mesh.

  ### 1. The Mobile Offline Mesh (Client-Side)
  - **Embedded Store**: Use `sqflite` (Flutter) or `IndexedDB` (Web) to maintain a local, encrypted SQLite replica of the tenant's critical data (Customers, Jobs, Catalog).
  - **Mutation Queue**: All writes (e.g., `UpdateJobStatus`, `DraftInvoice`) are written locally to a `mutation_queue` table.
  - **Optimistic AI Drafts**: The client can use a lightweight on-device model or rule engine to provide immediate feedback (e.g., auto-filling standard service notes), pending cloud agent review.

  ### Architecture Diagram
  ```mermaid
  erDiagram
      TenantContext ||--o{ MutationQueue : queues
      TenantContext ||--o{ Customers : caches
      TenantContext ||--o{ Jobs : caches
      TenantContext ||--o{ Catalog : caches

      MutationQueue {
          uuid id PK
          uuid tenant_id
          string action_type
          json payload
          string status "pending|synced|failed"
          timestamp created_at
      }

      CloudEventBus ||--o{ SyncEvent : triggers
      SyncEvent {
          uuid id PK
          uuid tenant_id
          uuid batch_id
          string action_type
          json payload
          timestamp synced_at
      }

      OperationsWorker ||--o{ SyncEvent : consumes
  ```

  ### Sequence Diagram
  ```mermaid
  sequenceDiagram
      actor Carlos
      participant MobileClient as Mobile Client (SQLite)
      participant Network as Network (Offline/Online)
      participant PowerSync as PowerSync / Backend API
      participant EventBus as OHC Event Bus
      participant OpsAgent as AI Operations Agent

      Carlos->>MobileClient: Completes Job & Logs Notes
      Note over MobileClient: Device is Offline
      MobileClient->>MobileClient: Save to Local SQLite (Jobs + MutationQueue)
      MobileClient-->>Carlos: UI Update: "Saved Offline (Green Check)"

      Carlos->>Network: Drives to Coverage (Online)
      MobileClient->>PowerSync: Connect WebSocket & Push Mutation Batch
      PowerSync->>PowerSync: Resolve Conflicts (LWW) & Commit to Postgres
      PowerSync->>EventBus: Emit SyncEvent (JobCompleted)
      PowerSync-->>MobileClient: Ack Mutation Sync

      EventBus->>OpsAgent: Consume SyncEvent
      OpsAgent->>OpsAgent: Analyze Job Notes
      OpsAgent->>EventBus: Draft Invoice & Action Card
      EventBus-->>MobileClient: Push Notification: "Invoice Drafted"
  ```

  ### 2. The Sync Engine (Backend & Transport)
  - **PowerSync Integration**: Leverage the existing `journeyapps/powersync-service` in our docker-compose stack. PowerSync reads from our logical replication slot in PostgreSQL and sends only the delta (changes) to the mobile client over websockets.
  - **CRDTs / LWW**: Use Last-Write-Wins (LWW) conflict resolution for simple fields, and application-level resolution for critical state (e.g., double-booking).

  ### 3. Agent Trigger Orchestration
  - **The Sync Webhook**: When PowerSync successfully upstream-syncs a batch of mutations (e.g., Carlos's completed job), it triggers a unified `SyncEvent` to the OHC Event Bus.
  - **Agent Activation**: The Operations Agent listens for `SyncEvent:JobCompleted`, reads the synced notes, and autonomously drafts the follow-up invoice or review request, pushing an Action Card to the owner's feed.

  ### AI Integration Points
  - The AI Operations Agent never blocks the user. It acts asynchronously when data syncs.
  - The UI must clearly indicate "Syncing" vs "Agent Reviewing" states.

  ### Mobile UX Flow (375px First)
  1. Carlos views his daily schedule (cached offline).
  2. He taps a job, marks it "Complete", and types "Replaced the valve." (Saved instantly to local SQLite).
  3. UI shows a green checkmark with a small "Saved offline" indicator.
  4. Carlos drives to cellular coverage. The app syncs invisibly.
  5. The AI Operations Agent sees the sync, drafts an invoice, and pushes a notification: "Invoice drafted for 123 Main St. Tap to review & send."

  ## Implementation Prompt (For Implementer Agent)
  Implement the backend infrastructure and client-side adapters for the Offline-First Sync Mesh.
  1. Define the data structures for the `MutationQueue` and `SyncEvent` stream.
  2. Implement the backend webhook receiver that processes incoming mutation batches from the mobile client and emits strongly-typed events to the internal job queue.
  3. Create an integration point where the `OperationsWorker` (AI agent) can subscribe to these sync events to trigger follow-up actions (like drafting an invoice).
  4. Write E2E tests verifying that a simulated mobile client can go offline, generate mutations, come back online, sync, and trigger the AI agent.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
