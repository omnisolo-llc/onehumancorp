issue_title: "Implement Offline-First Mobile Sync Architecture for Field Workers"
issue_description: |
  # Implement Offline-First Mobile Sync Architecture for Field Workers

  ## Problem Statement
  Field operators like Carlos (Handyman) and Fatima (Food Cart Operator) frequently operate in areas with poor or zero cellular connectivity (basements, remote locations, crowded events). Currently, our mobile-first application struggles when the network drops: read operations fail, writes are lost, and the owner is blocked from doing their work. For Carlos to trust OHC as his primary assistant, he must be able to view service routes, accept job completion signatures, and log payments even while completely offline, knowing the system will seamlessly synchronize when he regains signal.

  ## Research Report
  Industry leaders in field-service and modern mobile POS systems (e.g., Square, Jobber, ServiceTitan) employ an "offline-first" architecture. Their applications treat local storage as the primary data source and background sync to the cloud.
  - **Square's Offline Mode**: Allows taking swiped payments and recording cash transactions offline. Background queues sync up when the network returns.
  - **Linear / Notion**: Utilize a local-first architecture where the UI responds instantly based on local state.
  - **OHC Gap**: OHC currently treats the cloud as the immediate source of truth. When the connection drops, our API calls fail, causing spinners or error states. We lack a robust local mutation queue and background sync engine.

  ## Design Doc
  ### Mobile UX Flow (375px)
  1. **Offline Indicator**: A subtle, non-intrusive indicator (e.g., a "Cloud with slash" icon in the header) lets Carlos know he is operating offline.
  2. **Read Availability**: All critical daily tasks (today's bookings, customer notes, map routes) are aggressively pre-cached to device storage on app launch or foreground.
  3. **Write Availability**: Carlos taps "Complete Job" and logs a $150 cash payment. The UI immediately updates to "Completed" and the payment shows as "Pending Sync".
  4. **Background Sync**: Once signal is restored, a background service pushes the queued mutations to the OHC backend. The UI updates to "Synced" seamlessly.
  5. **Conflict Resolution**: If Carlos updated a booking offline that Maya (dispatcher) changed online, the latest timestamp or a designated "field worker wins" rule resolves it, with an AI Agent feed notification explaining the merge.

  ### Architecture Integration
  - **Local Database**: Use robust local storage to persist the UI state and cached entities.
  - **Mutation Queue**: All writes (POST/PUT/DELETE) in offline mode are serialized into a local queue table (`pending_mutations`).
  - **Idempotency**: Every queued mutation generates a unique `idempotency_key` (UUID v4) at the time of action to prevent duplicate writes during retries.
  - **Sync Manager**: A singleton service listens to network state changes. On network restore, it dequeues mutations and sends them via API to the backend.
  - **AI Agent Integration**: The "Operations Assistant" monitors sync conflicts. If a conflict requires owner attention, the AI assistant drafts a simple feed item: "I merged an offline update from Carlos for Job #104. Please review the new time."

  #### ER Diagram
  ```mermaid
  erDiagram
      LOCAL_STORAGE {
          string id
          string type
          json data
          datetime last_synced
      }
      MUTATION_QUEUE {
          string idempotency_key PK
          string method
          string endpoint
          json payload
          datetime created_at
          int retry_count
      }
      LOCAL_STORAGE ||--o{ MUTATION_QUEUE : "has pending updates"
  ```

  #### Sequence Diagram
  ```mermaid
  sequenceDiagram
      actor FieldWorker
      participant UI
      participant SyncManager
      participant MutationQueue
      participant Backend

      FieldWorker->>UI: Action (Offline)
      UI->>MutationQueue: Enqueue Action (with Idempotency Key)
      UI-->>FieldWorker: Optimistic Success Response

      note over SyncManager: Detects Network Restore
      SyncManager->>MutationQueue: Dequeue Pending Actions
      loop Over Actions
          SyncManager->>Backend: API Request (Action, Idempotency Key)
          Backend-->>SyncManager: 200 OK / Conflict
      end
      SyncManager->>UI: Update Sync Status
  ```

  ## Implementation Prompt
  **For the Implementer:**
  Implement the foundation of the Offline-First Sync Architecture for the OHC mobile app interface.
  1. Introduce a local storage wrapper to cache "Today's Work" (Tasks/Bookings).
  2. Implement the `MutationQueue` service that intercepts failed or offline network writes and stores them locally with an idempotency key.
  3. Create a Network Observer that triggers the `MutationQueue` to flush its contents to the backend when connectivity is restored.
  4. Ensure the UI optimistically reflects the offline changes (e.g., updating a task to "Done") without waiting for network confirmation.
  5. **Acceptance Criteria**: Disconnect the network, mark a task as completed, observe the UI update instantly, reconnect the network, and verify the backend receives the update exactly once. Ensure the implementation is fully functional on a 375px viewport with a clear "Offline" visual indicator.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
