issue_title: "Implement Universal OHC Offline-Tolerant Mobile Sync Protocol"
issue_description: |
  # Problem Statement
  During our field use and live service gap audit of the OHC UI on simulated mobile scenarios (375px viewport), a critical gap was observed: business operators like Fatima (food cart, slow mobile data) and Carlos (handyman in basements) experience silent transaction failures when network connectivity drops. Currently, the UI either hangs on "Loading..." or drops the locally executed action entirely when the network request fails, violating the OHC promise of reliable, non-technical daily operations on mobile.

  # Research Report
  Our competitive analysis reveals that robust mobile-first tools like Square POS and modern field service apps implement local-first or offline-first sync architectures.
  - **The Gap:** OHC's current architecture expects synchronous connectivity to the gRPC/REST backend for critical operations (e.g., fulfilling an order, drafting a quote, responding to a customer).
  - **The Need:** A localized queue mechanism integrated with the OHC Assistant UI that allows the owner to "Approve" an agent's proposal, cache that approval locally if offline, and seamlessly sync it back to the AI Job Queue (PostgreSQL `SKIP LOCKED` pattern) when connectivity is restored, all without exposing the complexity to the user.

  # Design Doc
  ## Architecture Sequence Diagram
  ```mermaid
  sequenceDiagram
      participant App as Mobile App (Flutter PWA)
      participant LocalStore as IndexedDB/Local Storage
      participant SyncWorker as Background Sync Service
      participant API as OHC API Layer
      participant Queue as AI Job Queue (Postgres)

      App->>LocalStore: User Approves Action (Save locally)
      App-->>App: Optimistic UI Update (Mark as Done locally)
      SyncWorker->>LocalStore: Check for pending actions
      alt Online
          SyncWorker->>API: Sync Action payload
          API->>Queue: Enqueue Job
          API-->>SyncWorker: 200 OK
          SyncWorker->>LocalStore: Mark Synced
      else Offline
          SyncWorker-->>SyncWorker: Retry with exponential backoff
      end
  ```

  ## Entity-Relationship (ER) Diagram
  ```mermaid
  erDiagram
      Tenant {
          string id PK
          string name
      }
      OfflineActionQueue {
          string id PK
          string tenant_id FK
          string idempotency_key
          string action_type
          jsonb payload
          string status
          timestamp created_at
      }
      AIJobQueue {
          string job_id PK
          string tenant_id FK
          string action_type
          jsonb payload
          string status
      }

      Tenant ||--o{ OfflineActionQueue : queues
      Tenant ||--o{ AIJobQueue : executes
      OfflineActionQueue ||--o| AIJobQueue : transforms_to
  ```

  ## Mobile UX Flow (375px First)
  1. User views the "Unified Agent Feed" and sees a card: "Approve quote for Maya's Cake."
  2. User taps "Approve" (large touch target > 44px) while offline.
  3. The card immediately transitions to a "Pending Sync" translucent glass state with a small, unobtrusive cloud-sync icon, reassuring the user the action is saved.
  4. Once connectivity is restored, the card updates to "Done" automatically.

  ## AI Agent Integration Points
  - Operations Agents must support idempotent operations because the background sync service might retry sending the same payload.
  - The local store must include the `tenant_id` and `idempotency_key` in the cached payload to prevent duplicate executions upon reconnection.

  # Implementation Prompt
  **User-Facing Outcome:** Business owners using OHC can confidently approve tasks, send messages, and update their schedule even in areas with poor or no cellular reception. The app smoothly caches their actions and synchronizes them without throwing scary technical errors.

  **Critical User Journey (CUJ):**
  1. User opens the OHC PWA on a 375px mobile viewport.
  2. User disconnects from the network.
  3. User approves a pending task in the Agent Feed.
  4. The UI updates optimistically to show the task is handled (with a minor offline indicator).
  5. User reconnects to the network.
  6. The app syncs the action to the backend without user intervention and updates the UI to clear the offline indicator.

  **Acceptance Criteria:**
  - Implement a local storage queue mechanism on the frontend to intercept offline mutations.
  - Provide an optimistic UI state utilizing OHC Premium Tokens (translucency).
  - Ensure backend API endpoints accept idempotent requests using `idempotency_key`.
  - Add full E2E Playwright test simulating offline and online transitions during a core operation.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
