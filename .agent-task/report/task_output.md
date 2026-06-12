issue_title: "Feature Design: Offline-Tolerant Mobile-First POS Sync & Eventual Consistency"
issue_description: |
  ## Problem Statement
  Small business operators in the real world—like Fatima (Food Cart Operator)—frequently operate in environments with spotty or slow cellular data. Currently, the OHC mobile application relies on strong network connectivity to capture orders, update inventory, and fetch work tasks. If the network drops, the operator is paralyzed, unable to check out customers or see their daily menu queue. They need an assistant that functions reliably on low-end devices without a constant internet connection, queuing actions and syncing seamlessly once connectivity returns.

  ## Research Report
  ### Findings
  - **Competitor Analysis:** Square and Shopify POS both offer offline modes. Square allows taking offline card payments (with risk assumed by the merchant) and queues cash transactions.
  - **OHC Gap:** OHC's current web and mobile architecture assumes consistent API reachability for gRPC/REST calls.
  - **Persona Fit:** Fatima (Food Cart) operates in crowded areas where network congestion is high. Carlos (Field Service) visits remote homes where signal drops. Both require offline read capabilities (menu, schedule) and offline write capabilities (completing an order, marking a task done).

  ## Design Doc
  ### Architecture Design
  - **Local-First Database (Flutter/PWA):** Utilize a local SQLite or IndexedDB instance (via a cross-platform Flutter package like Drift or Isar) to act as the primary read/write target for the UI.
  - **Sync Engine:** Implement a background sync manager that monitors `connectivity_plus` state. When offline, all state-mutating actions (e.g., `CompleteOrder`, `UpdateInventory`) are appended to an `Outbox` table.
  - **Conflict Resolution (Eventual Consistency):** When network returns, the sync engine drains the Outbox via REST/gRPC. The backend API handles conflict resolution (e.g., using Last-Write-Wins based on vector clocks or timestamps) and ensures idempotency for retry logic.
  - **AI Agent Integration (Operations Assistant):** If a local offline action conflicts with a backend action (e.g., inventory oversell), the Operations Assistant generates an "Action Card" for the owner to review the anomaly upon sync.

  ### Mobile UX Flow (375px)
  1. **Offline Indicator:** A subtle translucent amber banner at the top of the Home Feed: "Working Offline - Syncing Paused".
  2. **Interactivity:** User taps "Complete Order". The button shows a brief loading spinner, then a success checkmark. The data is saved locally and added to the Outbox.
  3. **Reconnection:** Once back online, the amber banner disappears, replaced by a momentary green "Sync Complete" toast.
  4. **Conflict Handling:** If a conflict occurred, an Action Card appears in the Feed: "Order #123 conflicted with an online purchase. Please review."

  ## Implementation Prompt
  Implement the Offline-Tolerant Sync Engine for the Flutter mobile client.
  - Create the local `Outbox` entity using the existing local database strategy.
  - Implement a background service that listens to network connectivity changes and processes the `Outbox` queue.
  - Ensure the UI components gracefully handle the offline state (e.g., showing the offline banner, processing local writes immediately for optimistic UI).
  - Add backend idempotency checks for the synced endpoints.
  - Do NOT implement specific database migrations for the backend, but rather use existing API endpoints with idempotency keys. Let the backend developer handle specific backend schema changes if needed.

  **Acceptance Criteria:**
  - On a 375px viewport simulator with network disconnected, the user can mark an order as complete.
  - The UI updates instantly (optimistic update) and shows an offline indicator.
  - When network is restored, the `Outbox` is flushed and the backend reflects the completed order.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
