issue_title: "Design & Research: Offline-Tolerant Autonomous Pre-Order & Fulfillment Queue Engine"
issue_description: |
  # Research Report: Offline-Tolerant Autonomous Pre-Order & Fulfillment Queue Engine

  ## Problem Statement
  Small business operators like Fatima (food cart operator) and Jun (location manager) process high-velocity, real-world fulfillment tasks (e.g. food prep, pickup timing, delivery handover). Currently, standard web-based order queues and POS systems require constant internet connectivity. In low-data environments or during network drops, these operators lose access to their active order lists, leading to missed pickups, upset customers, and halted operations. Standard sync mechanisms fail to provide a robust, invisible local-first experience. Operators need an autonomous, offline-tolerant fulfillment queue that functions seamlessly without network availability and gracefully syncs and resolves state conflicts when connectivity returns.

  ## Research Report
  - **Competitor Analysis:** Legacy platforms like Shopify and Wix provide cloud-centric fulfillment queues that freeze or fail when offline. Square POS has offline payment capabilities but struggles with complex, multi-state pre-order fulfillment synchronization across devices. Enterprise solutions (e.g. Toast) offer local networks but require expensive proprietary hardware and heavy technical setup.
  - **OHC Opportunity:** OneHumanCorp can differentiate by treating offline resilience as a core primitive via a Local-First architecture using Conflict-Free Replicated Data Types (CRDTs) and an Optimistic Mutation Engine. By doing so, OHC enables non-technical owners to operate flawlessly on standard, low-end mobile devices, with AI agents invisibly handling complex background syncing and conflict resolution.

  ## Design Doc
  ### Architecture Diagram (Mermaid)
  ```mermaid
  graph TD
      A[Mobile App - Fulfillment Queue UI] --> B(Local SQLite SIPDB & CRDT Sync Engine)
      B --> C{Network Connectivity Check}
      C -- Offline --> D[Optimistic UI Update & Local Spooling]
      C -- Online --> E[Hybrid Event Mesh / Sync API]
      E --> F(Operations Agent: State Conflict Resolution)
      F --> G[Global Multi-tenant PostgreSQL DB]
      G --> H[Global Redis Cache]
  ```

  ### Mobile UX Flow (375px First)
  1. **Order Feed Screen:** Displays a vertical list of pending pre-orders as large, touch-friendly cards (minimum 44x44px touch targets).
  2. **Offline Indicator:** A subtle, non-intrusive status pill at the top of the screen (e.g., "Offline - Orders tracking locally").
  3. **Fulfillment Action:** Operator taps a massive "Mark Ready" button on an order card.
  4. **Optimistic UI:** The card instantly animates to the "Completed" state with a clear success visual (e.g., green checkmark), regardless of network status.
  5. **Background Sync:** The action is spooled locally. When the network connects, the Operations Agent syncs the state invisibly. If a conflict occurs (e.g. customer cancelled while offline), the agent flags the order for operator review with a plain-language summary.

  ### AI Agent Integration
  - **Operations Agent:** Listens to the incoming sync mesh. It autonomously handles deterministic conflict resolutions (e.g., timestamp-based reconciliation) and prepares plain-language escalation summaries for complex conflicts (e.g., "Customer asked to cancel, but you already marked it ready while offline. How would you like to handle this?").

  ### Key Design Decisions
  - **Local-First / Optimistic Mutations:** Ensure the UI never blocks on a network call. All state changes are written locally first.
  - **Translucent Glass UI:** Adopt OHC Premium Token library standards. Use macOS-style Translucent Glass materials (e.g., `background: rgba(255, 255, 255, 0.65)`, `backdrop-filter: blur(30px) saturate(210%)`) to create a clear, readable hierarchy that works well outdoors in sunlight.
  - **No Technical Jargon:** Error states must never show "Sync Error" or HTTP codes.

  ## Implementation Prompt
  Implement the core architectural foundations for the Offline-Tolerant Fulfillment Queue Engine.
  1. **Backend:** Define the CRDT-friendly data model for `OrderFulfillmentState` within the PostgreSQL database, ensuring multi-tenant RLS is strictly enforced. Implement the synchronization endpoints that accept batched, optimistic local events from the mobile client.
  2. **Operations Agent Integration:** Scaffold the logic where the Operations Agent intercepts sync events, handles basic timestamp-based conflict resolution, and queues complex conflicts for human review.
  3. **Frontend (Tauri/Flutter):** Implement the local queue storage mechanism and the background sync worker that intelligently flushes the queue when network connectivity is detected. Do not prescribe specific libraries, but focus on the end-to-end data flow and robust error handling.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
