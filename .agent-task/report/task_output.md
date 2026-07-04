issue_title: "[Research] AI-Driven Offline-First Order Sync & Mobile Pre-caching"
issue_description: |
  ## Title
  AI-Driven Offline-First Order Sync & Mobile Pre-caching for Low-Connectivity Environments

  ## Problem Statement
  Food cart operators like Fatima often work in locations with spotty, slow, or non-existent mobile data connections. Customers might place pre-orders online, but Fatima's low-end Android phone fails to load the active order queue or synchronize inventory in real-time due to poor connectivity. This results in missed orders, angry customers, and inventory discrepancies. Current market solutions simply display spinning loaders or network error messages, paralyzing the operator's ability to fulfill work.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Shopify POS:** Relies heavily on a constant network connection for its cloud-based order synchronization. Offline mode exists but is primarily focused on queuing new transactions to sync later, not on serving a pre-cached queue of incoming online orders.
  - **Square POS:** Offers a robust offline payment mode, but incoming online order syncing is delayed until the connection is restored.
  - **Wix & GoDaddy:** Cloud-dependent dashboards that fail completely on slow 3G networks or intermittent connections.
  - **OHC Opportunity:** Implement an "Offline-First Order Synchronization" architecture. Proactively push the day's expected workload to a local device cache (SQLite/IndexedDB). A lightweight background sync mechanism with exponential backoff and payload compression ensures that even a fleeting connection is enough to sync the delta.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Central Ledger / OHC Backend] <-->|Delta Sync| B(Offline-First Sync Engine)
      B <--> C[Local Device Cache SQLite/IndexedDB]
      C <--> D[Mobile App UI 375px]
      E[Operations Agent] -->|Predictive Pre-caching| A
      F[Customer Success Agent] -->|SMS Fallback| G[Customer / Owner]
      B -->|Network Flaky| H{Connection Monitor}
      H -->|Online| A
      H -->|Offline| C
  ```

  ### Mobile UX Flow (375px First)
  1. **Order Queue Screen:** Displays the current list of orders.
  2. **Connection Status Indicator:** A subtle UI element that doesn't block the screen (e.g., gray cloud for offline).
  3. **Offline Interaction:** When offline, the owner can tap "Complete Order," mark items as "Sold Out," or add a walk-up order. Actions are instantly reflected (Optimistic UI).
  4. **Sync Resolution:** Once connectivity is restored, a small toast notification indicates "Syncing changes..." and disappears upon success.

  ### AI Agent Integration Points
  - **Operations Agent:** Analyzes past sales patterns and pre-emptively caches the most relevant data locally before the connection drops.
  - **Customer Success Agent:** If an owner marks an item "Sold Out" offline, the system queues the action. Upon reconnecting, the agent immediately updates the storefront and can automatically SMS customers.

  ### Key Design Decisions
  - **Optimistic UI:** The UI must never block the user from taking action. All state mutations are applied locally first and queued for backend synchronization.
  - **Conflict Resolution:** Server wins on critical inventory numbers, but local wins on task completion.
  - **Delta Payload Sync:** The sync engine only exchanges compressed JSON patches representing exact changes, minimizing data usage on slow networks.

  ## Implementation Prompt
  **Implementer Agent Task:**
  Implement the Offline-First Order Sync engine for the mobile client. Ensure that the core Critical User Journey (CUJ) for an operator (viewing the order list, marking an order as complete, and marking an item as sold out) can be performed seamlessly with the network completely disabled. The application state should persist locally. When the network is re-enabled, the queued mutations must automatically sync with the backend. Write Playwright E2E tests that simulate an offline state, perform actions, toggle online mode, and verify the backend state updates correctly.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []