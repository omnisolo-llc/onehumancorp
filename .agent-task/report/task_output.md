issue_title: "Research: Offline-First Sync Engine for Field Operations"
issue_description: |
  # Research Report: Offline-First Sync Engine for Field Operations

  ## Problem Statement
  Field service owners like Carlos (Handyman) operate in environments with highly unreliable cellular networks (e.g., basements, remote job sites). Current web-first and cloud-first designs either block operations entirely when offline or fail silently. Without a robust offline-first synchronization engine, Carlos cannot view job details, capture signatures, or draft quotes while on-site. He needs an assistant that works offline and gracefully reconciles with the central server when connectivity is restored, ensuring no data loss or double-booking.

  ## Research Report
  - **Market Context**: Most small-business field service apps (e.g., Jobber, Housecall Pro) provide offline capabilities, but these are often limited to reading pre-cached data, with writes either disabled or poorly queued.
  - **The OHC Opportunity**: Implementing a structured Event-Sourcing offline sync engine on the mobile app (Flutter) that captures user intents as discrete operations. This enables Carlos to continue working seamlessly. Once online, OHC's Operations Agent can resolve any conflicts and update the PostgreSQL central ledger.
  - **Competitor Gaps**:
    - *Jobber*: Solid offline read, but write actions can be clunky and prone to sync errors.
    - *Square*: Good offline payment queue, but lacks a generalized offline state for tasks/quotes.

  ## Design Doc
  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  sequenceDiagram
      participant App as Flutter Mobile App
      participant Local as Local SQLite (Device)
      participant Net as Network Sync Layer
      participant Agent as Operations Agent (Cloud)
      participant DB as PostgreSQL (Central Ledger)

      App->>Local: Write (Quote Draft) - Offline
      Local-->>App: Confirmed (Local)
      Note over App,Local: Device reconnects to network
      Net->>Local: Fetch pending operations
      Net->>Agent: Submit Operations Queue
      Agent->>DB: Apply Operations & Conflict Resolution
      DB-->>Agent: Success
      Agent-->>Net: Sync Acknowledgment
      Net->>Local: Update Local State & Clear Queue
  ```

  ### Mobile UX Flow (375px)
  1. **Offline Indicator**: A subtle, unobtrusive banner or icon indicating "Working Offline".
  2. **Job View**: Carlos views pre-fetched job details seamlessly.
  3. **Action State**: When Carlos completes a job or creates a quote, the UI immediately reflects the success state but marks the item with a small "Pending Sync" icon.
  4. **Restoration**: Upon network reconnection, a background task syncs the queue, and the "Pending Sync" icon transitions to a solid "Confirmed" checkmark.

  ### AI Agent Integration Points
  - **Operations Agent**: Serves as the intelligent conflict resolver during sync. If a job was updated by a dispatcher while Carlos was offline, the agent merges non-conflicting fields and creates an "Action Required" notification for Carlos if a hard conflict occurs.

  ### Key Design Decisions
  - **Event-Sourced Local Queue**: Instead of updating local models directly, actions are stored as an ordered queue of events (e.g., `CREATE_QUOTE`, `UPDATE_JOB_STATUS`). This ensures ordered execution and easier conflict resolution on the backend.
  - **Optimistic UI Updates**: The app assumes success for offline operations, providing immediate feedback to the user without waiting for the network.

  ## Implementation Prompt
  **Feature Name**: OHC Offline-First Sync Engine
  **Target Persona**: Carlos the Handyman
  **Outcome**: Carlos can view jobs, create quotes, and capture signatures in a basement with no cellular service. The app syncs seamlessly in the background once he returns to his truck.

  **Next Actions**:
  1. Initialize a local SQLite database in the Flutter app to cache job schedules, customer data, and the operational event queue.
  2. Develop the Sync Manager service in the Flutter app to monitor network connectivity and manage the queue submission lifecycle.
  3. Implement the backend Sync API endpoint (`/api/sync`) to receive event queues and integrate with the Operations Agent for conflict resolution.
  4. Update the UI to include intuitive "Offline" and "Pending Sync" visual states according to the Translucent Glass design tokens.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
