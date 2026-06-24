issue_title: "Implement Autonomous Data Synchronization Protocol for Offline-First Edge Clients"
issue_description: |
  # Research Report: Autonomous Data Synchronization Protocol for Offline-First Edge Clients

  ## Problem Statement
  Small business owners and operators using OneHumanCorp (OHC) frequently operate in environments with intermittent, flaky, or non-existent network connectivity. For example:
  - **Fatima (Food Cart):** Operates her food cart in a busy urban plaza where mobile data is congested or drops out entirely. She needs to continue taking pre-orders, updating menu availability, and reviewing her task list without interruption.
  - **Carlos (Field Service Owner):** Performs repairs in customer basements or remote areas with zero cell service. He needs to access service route details, update job statuses, and draft estimates offline, knowing they will sync seamlessly when he returns to a connected area.

  Currently, while OHC has an architecture for cloud relays and webhook tunnels, the core mobile-first shell lacks a robust, transparent, and conflict-free data synchronization protocol that allows users to perform critical writes (e.g., updating a booking status, completing a task, capturing customer notes) while completely offline. When the app goes offline, critical writes fail or hang, destroying user trust. The owner must not be burdened with manual "sync now" buttons or understanding technical connection states.

  ## The Gap
  The absence of an offline-first data synchronization engine means that our mobile client (Flutter PWA) is fragile in real-world operator environments. If a network request fails, the state is lost or out of sync. We need a system where local writes are optimistic and persisted locally, then queued and autonomously synchronized to the Go + Bazel backend (PostgreSQL) when connectivity is restored, with AI agents handling conflict resolution.

  ## Competitive Analysis
  - **Linear / Notion:** Excellent offline-first capabilities using local data stores (IndexedDB/SQLite) and sophisticated sync engines. However, these are built for knowledge workers, not necessarily field operators.
  - **Square:** Allows offline payments (with risk assumed by the merchant) and queues them for later. Simple and effective for one specific domain.
  - **Powersync / ElectricSQL:** Modern local-first database synchronization tools. These are powerful but introduce heavy architectural dependencies.
  - **OHC's Differentiation:** We will build a seamless, AI-assisted **Autonomous Data Synchronization Protocol**. It will leverage a local event queue in the Flutter PWA (persisted to local storage). When online, the queue is drained to the backend. If a conflict occurs (e.g., Carlos updates a booking offline, but the customer cancelled it online), the "Operations Agent" steps in to resolve the conflict intelligently based on context, notifying the owner only if human judgment is strictly required.

  ## Architectural Design

  ### System Overview

  ```mermaid
  graph TD
      subgraph Mobile Client "Flutter PWA (Edge Client)"
          UI[Assistant-First Shell UI]
          LocalDB[(Local SQLite / IndexedDB)]
          SyncQueue[Local Sync Queue]
          NetworkMonitor[Network Status Monitor]
      end

      subgraph Backend "Go + Bazel Backend"
          API[Sync API Endpoint]
          Ledger[(PostgreSQL - Tenant DB)]
          ConflictQueue[Conflict Resolution Queue]
      end

      subgraph AI "AI Agent Departments"
          OpsAgent[Operations Agent]
      end

      UI -->|Optimistic Write| LocalDB
      UI -->|Enqueue Mutation| SyncQueue

      NetworkMonitor -->|Detects Online| SyncQueue
      SyncQueue -->|Drains Batched Events| API

      API -->|Attempt Apply| Ledger
      API -->|Detect Conflict| ConflictQueue

      ConflictQueue --> OpsAgent
      OpsAgent -->|Resolves intelligently| Ledger
      OpsAgent -->|Notifies if needed| UI
  ```

  ### Mobile UX Flow (375px)
  1. **Offline State Indicator:** A subtle, translucent glass pill at the top of the UI indicates "Working Offline - Changes Saved" using design tokens. No intrusive modals.
  2. **Optimistic Updates:** When Carlos taps "Complete Job", the UI updates instantly. The change is stored locally.
  3. **Background Sync:** When the network returns, the pill transitions to "Syncing..." and then fades out. The sync happens transparently via a background worker.
  4. **Conflict Notification:** If a conflict requires attention, a unified "Work Triage" card appears: "Agent Notice: The booking for Smith was cancelled online while you updated it offline. I kept the cancellation. Tap to review."

  ### Key Design Decisions
  - **Event-Sourced Local Queue:** Instead of trying to diff complex local database states against the server, the mobile client records a chronological queue of mutation events (e.g., `UpdateJobStatus(jobId, "COMPLETED")`).
  - **Idempotency:** Every event in the queue has a unique UUID. The backend API (`/api/v1/sync/events`) processes events idempotently to handle flaky network retries safely.
  - **AI Conflict Resolution:** Traditional systems use "last write wins" or complex CRDTs. OHC will use a pragmatic approach: apply writes if the base version matches. If a version mismatch occurs, route the event to the Operations Agent, which uses LLM context to decide the outcome (e.g., a customer cancellation always overrides a provider "started" status).

  ## Implementation Prompt
  **Role:** Implementer Agent
  **Task:** Implement the backend foundation for the Autonomous Data Synchronization Protocol.
  **CUJ:** The mobile client (simulated via API calls) sends a batch of offline mutation events to the backend. The backend applies valid events to the database and flags conflicting events.
  **Acceptance Criteria:**
  1. Create a generic `SyncEvent` Go struct and corresponding database table (or utilize an existing event ledger) to receive batched mutations from edge clients.
  2. Implement a new REST endpoint (`POST /api/v1/sync/events`) that accepts an array of `SyncEvent` payloads.
  3. The endpoint must process events idempotently using the event UUID.
  4. Implement basic version checking: if the entity being mutated has a newer version in the DB than the event's `base_version`, route the event to a "ConflictQueue" (this can be a simple DB table or in-memory queue for this iteration) instead of applying it directly.
  5. Ensure multi-tenant isolation (RLS) is strictly enforced for all sync operations using the `organization_id` claim.
  6. Add comprehensive unit tests covering successful application, idempotent retries, and conflict detection.

  ## Priority
  `P1` (High) - Critical for mobile-first user trust in real-world environments.

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
