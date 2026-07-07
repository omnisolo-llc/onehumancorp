issue_title: "Implement Offline-First Mobile POS & Optimistic Mutation Engine Architecture"
issue_description: |
  # Offline-First Mobile POS & Optimistic Mutation Engine

  ## Problem Statement
  Small business owners operating in challenging network conditions—such as Fatima managing her food cart in areas with patchy 4G, or Carlos repairing plumbing in a customer's basement with zero reception—face critical operational friction when their point-of-sale (POS), booking, or inventory management tools fail offline. Existing solutions either prevent actions completely without a connection or show generic error states, leading to lost sales and poor customer experiences. We need a robust, offline-first mobile architecture that allows the business to function seamlessly when disconnected, capturing orders, taking cash/offline payments, and securely syncing state back to the OHC backend once connectivity is restored.

  ## Research Report
  **Market Competitive Analysis:**
  - **Square POS:** The industry leader in offline mode. Square allows merchants to process cash and offline card payments (with explicit risk disclaimers) seamlessly when the network drops. Transactions are queued locally and synced within 24-72 hours.
  - **Shopify POS:** Offers offline capabilities primarily for cash and custom payment types. Card payments require an active connection. Inventory is cached locally, but true optimistic sync for complex catalog updates is limited.
  - **Wix/Squarespace:** Primarily online-dependent. While they offer mobile apps, robust offline-first POS and inventory management are not deeply integrated at the core edge layer, requiring a solid connection for most management tasks.

  **Our Opportunity:**
  OneHumanCorp can differentiate by treating offline resilience not as a bolt-on feature, but as a core architectural primitive. By employing an Optimistic Mutation Engine with Conflict-Free Replicated Data Types (CRDTs) or a robust local action queue (Local-First architecture), OHC will guarantee that a user (like Maya or Fatima) can manage inventory, process local cash/tap-to-pay offline operations, and rely on the AI Operations Department to resolve state conflicts gracefully in the background without overwhelming the user with technical "sync error" jargon.

  ## Design Doc

  ### Core Architectural Concepts
  1. **Local-First Datastore (SQLite/Hive on Mobile):** The Flutter app will maintain an encrypted local replica of the tenant's critical working set (active catalog, current bookings, today's order queue).
  2. **Optimistic Mutation Engine:** User actions (e.g., fulfilling an order, marking an item sold out) update the local UI instantly. The action is encapsulated into an `OperationIntent` and persisted to a local outbox queue.
  3. **Background Sync Coordinator:** When the network transitions to online, a background isolate processes the outbox queue, submitting intents to the Go backend via idempotent gRPC endpoints.
  4. **AI-Assisted Conflict Resolution:** If an intent fails due to state drift (e.g., trying to sell the last cupcake that was sold online while the app was offline), the Operations AI Agent intercepts the conflict, determines the best fallback (e.g., issuing an automatic refund or sending a drafted apology message), and notifies the user via the Business Advisory department.

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant User (Flutter App)
      participant Local DB (Outbox)
      participant Sync Worker (Background)
      participant OHC Backend (Go)
      participant AI Ops Agent (Gemini)

      User->>Local DB: Mark "Vegan Cake" as Sold Out (Offline)
      Local DB-->>User: UI Updates Optimistically
      Local DB->>Local DB: Store OperationIntent in Outbox

      Note over Sync Worker: Network restored

      Sync Worker->>Local DB: Read Outbox Intents
      Sync Worker->>OHC Backend: Submit Intent (Idempotency Key: UUID)

      alt Success
          OHC Backend-->>Sync Worker: 200 OK (State Synced)
          Sync Worker->>Local DB: Mark Intent as Processed
      else Conflict Detected (e.g. Item modified online)
          OHC Backend->>AI Ops Agent: Trigger Conflict Resolution
          AI Ops Agent->>OHC Backend: Resolve conflict based on tenant policy
          OHC Backend-->>Sync Worker: 200 OK (Resolved state)
          Sync Worker->>Local DB: Update local cache with resolved state
          AI Ops Agent->>User: Advisory Notification (Plain text summary)
      end
  ```

  ### Mobile-First UX Flow (375px Viewport)
  1. **Network Indicator:** A subtle, premium glassmorphism pill at the top of the dashboard. When online, it's hidden. When offline, it gracefully slides down showing "Offline Mode" in a muted amber color.
  2. **Action Execution:** Fatima taps "Sold Out" on the Falafel item.
  3. **Optimistic Feedback:** The toggle slides immediately. A soft haptic vibration confirms the action. A small "Pending Sync" icon (like a spinning dashed circle) appears next to the toggle.
  4. **Resolution:** When the connection returns, the icon disappears. If there's an issue, a non-intrusive bottom sheet pops up: "The Operations Manager adjusted your stock—Falafel was already sold out online."

  ## Implementation Prompt
  **Role:** Implementer Agent
  **Goal:** Implement the Core Optimistic Mutation Engine and Outbox Sync Queue in the Flutter frontend, along with the corresponding idempotent endpoint handlers in the Go backend.

  **Acceptance Criteria:**
  1. Define the `OperationIntent` schema locally (SQLite/Hive) and on the server (PostgreSQL).
  2. Implement a unified `MutationService` in Flutter that applies UI state optimistically, saves the intent locally, and queues it for the sync worker.
  3. Build the background sync worker that monitors network connectivity and flushes the outbox sequentially, ensuring idempotency.
  4. Implement the backend gRPC handler to receive `OperationIntent` batches, apply them within a database transaction using the tenant's context, and trigger a conflict resolution workflow if the state timestamp mismatches.
  5. Provide a simple UI component (network status pill) to display offline status and pending sync count.

  **Priority:** P0 (Critical for Mobile-First strategy)
  **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
