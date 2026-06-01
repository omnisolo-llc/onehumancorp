issue_title: "[Architecture] Distributed Sync Daemon for Universal Offline Capabilities"
issue_description: |
  ## Problem Statement
  Small business owners like Fatima (food cart) or Carlos (handyman) frequently operate in environments with spotty or non-existent cellular coverage (e.g., inside a client's basement, at a crowded farmer's market). A primary limitation of the current architecture is that the OHC mobile client assumes a reliable connection to the cloud for most actions, leading to failed reads, stuck operations, and silent transaction drops when offline. This directly violates the "mobile-first, offline-capable" core mandate. We need a robust, unified offline synchronization engine (a "Sync Daemon") that caches necessary read-state locally and reliably queues and resolves offline mutations, ensuring zero data loss and a seamless offline user experience.

  ## Research Report
  - **Context & Market Analysis:** Offline support is a major differentiator in the SMB platform space.
    - **Shopify POS:** Offers offline capabilities primarily for processing cash payments, but card payments and inventory syncs often fail or get stuck.
    - **Square:** Known for its robust offline mode, securely caching card details and queuing transactions, which is a major reason for its adoption in food trucks and events.
    - **OHC Gap:** OHC lacks a unified, cross-domain offline synchronization protocol. While specific domains (like Tap-to-Pay POS or printing) have offline designs, there is no generic, transparent layer that handles offline queuing, conflict resolution, and optimistic UI updates for the entire mobile app.

  - **Key Learnings:**
    1. **Optimistic UI is Essential:** Users must feel like the app is working normally even when offline.
    2. **Conflict Resolution:** When syncing back to the cloud, the system must handle conflicts gracefully (e.g., if inventory was updated from another device while offline).
    3. **Background Sync:** The sync process must run reliably in the background when connectivity is restored, without requiring the user to keep the app open.

  ## Design Doc
  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  graph TD;
      AppUI[OHC Mobile App UI] --> SyncDaemon[Local Sync Daemon];
      SyncDaemon --> LocalStore[Local SQLite Database];
      SyncDaemon --> MutationQueue[Local Mutation Queue];

      MutationQueue -->|Network Restored| SyncGateway[Cloud Sync Gateway];
      LocalStore <--|Background Sync| SyncGateway;

      SyncGateway --> CoreAPI[OHC Core API];
      CoreAPI --> Database[PostgreSQL Database];
      CoreAPI --> AgentQueue[AI Job Queue];
  ```

  ### Mobile UX Flow (375px First)
  1. **Offline Indicator:** A subtle, non-intrusive indicator (e.g., a cloud with a slash) appears in the header, letting the user know they are offline but can continue working.
  2. **Optimistic Actions:** Fatima accepts an order. The UI instantly updates to show the order as "Queued" or "Pending Sync", with a clear success state. No loading spinners waiting for a timeout.
  3. **Background Sync Notification:** When connection is restored, a small, transient toast notification appears: "Syncing 3 pending orders..." followed by "All caught up!".
  4. **Conflict Resolution UI:** In the rare case of a conflict (e.g., "Item sold out online while you were offline"), a clear, actionable card appears on the dashboard explaining the issue in plain language and offering a simple choice to resolve it.

  ### Key Design Decisions
  - **Local-First Architecture:** The mobile app should primarily read from and write to the Local SQLite Database. The Sync Daemon handles the complexity of syncing this local state with the cloud.
  - **Mutation Queue:** All state changes (mutations) made offline are appended to a persistent, ordered Local Mutation Queue.
  - **Idempotency & CRDTs (Future):** Sync operations must be idempotent. Where possible, data models should be designed to minimize conflicts (e.g., using operation-based tracking like "add 1" instead of "set to 5").
  - **Zero Trust:** Even offline, the Sync Daemon respects tenant boundaries and only stores data relevant to the authenticated user.

  ### AI Agent Integration Points
  - **Operations Agent:** Monitors sync logs. If an offline queue is stuck or contains errors, the Operations Agent can attempt auto-resolution or notify the user via the plain-language daily briefing.

  ## Implementation Prompt
  **Task:** Implement the foundational Local Sync Daemon and Mutation Queue for the OHC mobile client.
  **User Story:** As an SMB owner operating in a dead zone, I want to continue using the app to accept orders and update my business, trusting that all my actions will automatically and reliably sync to the cloud once my connection is restored.
  **Acceptance Criteria:**
  - Create a generic Sync Daemon interface for the mobile client capable of intercepting API requests and caching them locally when offline.
  - Implement a persistent Local Mutation Queue (using SQLite) that reliably stores offline actions.
  - Implement a background process that monitors network connectivity and automatically replays the Mutation Queue sequentially to the cloud `SyncGateway` when online.
  - Ensure the UI receives real-time updates regarding sync status (offline, syncing, synced) to provide accurate optimistic feedback.
  - Establish a basic conflict resolution strategy for critical entities (e.g., last-write-wins or specific merging logic for inventory).

  ## Priority: P0
  ## Estimated Scope: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
