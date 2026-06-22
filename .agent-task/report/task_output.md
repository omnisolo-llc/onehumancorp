issue_title: "Research: Automated Offline Sync and Resolution for Unstable Networks"
issue_description: |
  # Research Report: Automated Offline Sync and Resolution for Unstable Networks

  ## Problem Statement
  Small business owners like Carlos the handyman or Fatima the food cart owner often operate in environments with unstable internet connections (e.g., basements, remote areas, or crowded events). Current platforms often fail silently, lose data, or block the user completely when offline. The owner needs an assistant that transparently records actions while offline and automatically synchronizes them once connectivity is restored, without requiring manual intervention or causing data conflicts. The lack of offline support causes missed orders, lost service updates, and frustration.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Traditional SaaS (e.g., Shopify POS, Square):** Often have basic "offline mode" for taking payments or recording orders, but they are typically rigid, requiring manual syncing or only working for specific features like POS. They do not offer a comprehensive, platform-wide offline-first architecture.
  - **Modern App Architecture (e.g., Linear):** Linear uses an offline-first architecture where every action is a local mutation that syncs optimistically to the server. This provides a fast, seamless experience regardless of network status.
  - **OHC Opportunity:** Implement an "Offline First & Sync" capability across the mobile-first platform. This involves using local first databases (like SQLite or IndexedDB) with an optimistic update strategy, combined with an AI conflict resolution agent to handle any sync issues intelligently when reconnecting. This ensures Carlos can update a job status in a basement, and it automatically syncs to the central database when he drives away.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Mobile App 375px] --> B{Local SQLite/IndexedDB}
      B --> C[Optimistic UI Updates]
      A -.->|Unstable Network| D(Sync Queue Manager)
      D -.->|When Online| E[Backend API Gateway]
      E --> F[Conflict Resolution Agent]
      F --> G[Unified DB PostgreSQL]
      G -.-> E
      E -.-> D
      D -.-> B
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **Global Header (Mobile):** Add an elegant, subtle indicator for connection status. When offline, a small, non-intrusive "Working Offline" pill appears. When syncing, it changes to "Syncing..." and then disappears upon success.
  - **Interaction:** The user interacts with the app normally. Tapping "Complete Job" updates the UI instantly, regardless of connection.
  - **Action:** If a conflict occurs during sync (e.g., another device updated the same record), the Conflict Resolution Agent attempts auto-resolution. If it cannot, a card appears in the Agent Feed: "Sync Conflict for Job #123. Review needed."
  - **Visual Design:** Translucent glass materials. Status pills use semantic colors (e.g., warning yellow for offline, success green for synced momentarily).

  ### AI Agent Integration Points
  - **Conflict Resolution Agent (The Conciliator):** Triggered when the backend detects a version mismatch or conflicting update during a sync operation from the Sync Queue Manager. It uses heuristics and context (e.g., timestamp, user role, entity type) to automatically resolve conflicts when safe, and only escalates to the owner via the Agent Feed when necessary.

  ### Key Design Decisions
  - **Offline First:** All reads and writes go to the local database first. The UI is always driven by the local state, ensuring zero latency and full offline capability.
  - **Optimistic Updates:** Actions appear complete instantly to the user.
  - **Agentic Resolution:** Offload the burden of resolving complex sync conflicts from the user to an AI agent whenever possible.

  ## Implementation Prompt
  **User-Facing Outcome:** As an operator, I can continue managing my business, updating job statuses, and drafting responses even when I have no cell service. When I get back to a good connection, everything updates automatically in the background.
  **CUJ & Acceptance Criteria:**
  1. A user logs into the app and the initial state is synced to the local database.
  2. The network connection is simulated as offline.
  3. The user performs a critical action (e.g., marking a task as complete). The UI updates immediately, and the action is queued locally.
  4. The network connection is restored.
  5. The Sync Queue Manager automatically pushes the pending action to the backend.
  6. The backend processes the action and confirms the sync.
  7. Provide Playwright E2E tests: A user performs an action while the browser is set to "offline mode" via Playwright's network emulation, verifies the UI updates optimistically, then sets the browser to "online mode" and verifies the action is synced to the backend and the offline indicator clears.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
