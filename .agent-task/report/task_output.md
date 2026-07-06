issue_title: "Architectural Design: Mobile-First Offline-Tolerant POS & Multi-Language Sync Engine"
issue_description: |
  # Title: Mobile-First Offline-Tolerant POS & Multi-Language Sync Engine

  ## Problem Statement
  For non-technical business owners operating in challenging network environments (like Fatima, the food cart operator), constant internet connectivity is an unrealistic assumption. Traditional cloud-first point-of-sale (POS) and order management systems degrade poorly or become completely unusable when mobile data drops, leading to lost sales, frustrated customers, and operational chaos. They need a system that captures orders, toggles availability, and queues notifications gracefully offline, automatically reconciling with the central multi-tenant cloud when the network recovers, all while supporting real-time multi-language translation.

  ## Research Report
  ### Market Context & Competitor Analysis
  - **Square / Weebly**: Offers an "Offline Mode" for swipe card payments and cash transactions, but the sync reconciliation often leads to inventory conflicts that the user must resolve manually.
  - **Shopify POS**: Requires a robust connection for full feature parity; offline mode is limited and complex to troubleshoot for non-technical staff.
  - **Toast (Restaurant POS)**: Excellent offline mode for local networks, but requires expensive proprietary hardware (hardline local servers), making it inaccessible for a solo food cart operator.

  ### Findings
  Our core user (Fatima) runs on a low-end Android device with intermittent 4G/3G data. She needs to toggle items as "sold out" offline and trust that this state will propagate to her customer-facing pre-order menu the second connectivity is restored. Furthermore, her customer notifications must be queued locally and processed by the AI Agent reliably.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant App as Mobile App (Flutter PWA)
      participant LocalDB as Local State
      participant Sync as Edge Sync Gateway
      participant Backend as OHC Cloud (Go + Postgres)
      participant Agent as Operations Agent (AI)

      App->>LocalDB: 1. Record Order (Offline)
      LocalDB-->>App: UI Updates Instantly
      App->>LocalDB: 2. Enqueue Sync Event
      Note over App, Sync: Network Restored
      App->>Sync: 3. Push Event Queue (Idempotent)
      Sync->>Backend: 4. Reconcile & Persist
      Backend->>Agent: 5. Trigger Workflow (e.g. Notify Customer)
      Backend-->>Sync: 6. Ack & Push State Updates
      Sync-->>App: 7. Update LocalDB
  ```

  ### UI Wireframes & Mobile UX Flow (375px)
  - **Screen 1: Order Dashboard (375px viewport)**
    - Top bar: Clear "Offline Mode" indicator (amber warning icon) replacing the green "Online" dot.
    - Main content: Large, readable list of pending orders (44x44px touch targets for "Complete" buttons).
    - Bottom sheet: "Menu Management" quick toggle.
  - **Screen 2: Menu Management**
    - List of menu items with large toggle switches for "Available" / "Sold Out".
    - Toggles respond instantly via optimistic UI updates, even offline.
  - **Mobile UX Flow:**
    1. User opens the app in a cellular dead zone.
    2. User toggles "Spicy Chicken" to "Sold Out".
    3. UI instantly reflects the change; an "Unsynced Changes" badge appears.
    4. Network reconnects; badge spins and disappears as the background sync worker flushes the queue to the backend.

  ### AI Agent Integration Points
  - **Conflict Resolution Agent**: If an offline order is placed for an item that sold out online simultaneously, the Agent intervenes, drafting an apologetic multi-language SMS to the customer proposing an alternative, awaiting owner approval.
  - **Translation Agent**: Localizes the UI on-the-fly and translates incoming customer pre-order notes into the owner's preferred language natively in the offline cache.

  ### Key Design Decisions
  - **Optimistic UI with Local-First Persistence**: The Flutter app will treat local device storage as the primary source of truth for reads and initial writes, ensuring zero-latency interactions regardless of network state.
  - **CRDTs (Conflict-free Replicated Data Types) & Event Sourcing**: Use an event-sourced queue for mutations rather than simple state overwrites. Each action is stamped with a local vector clock/timestamp to allow the backend to merge changes predictably.
  - **Idempotency**: All sync payloads must carry unique identifiers to prevent double-charging or duplicate orders upon network retries.

  ## Implementation Prompt
  **To the Implementer Agent:**
  Implement the Mobile-First Offline-Tolerant Sync Engine for the OHC platform.
  1. Build the frontend local-first persistence layer, ensuring the UI remains fully interactive without a network connection.
  2. Implement an optimistic UI update mechanism for toggling item availability and processing cash/offline orders.
  3. Create the backend Sync Gateway in Go capable of receiving batch event payloads, deduping via idempotency keys, and applying them to the database with multi-tenant row-level security.
  4. Ensure the Operations AI Agent is hooked into the event stream to handle any inventory conflicts gracefully.
  5. The acceptance criteria: A user must be able to load the app, disconnect from the internet, perform 3 state-mutating actions (e.g., mark an item sold out, complete an order), and have those actions successfully reconcile with the backend upon reconnection without data loss or UI blocking.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
