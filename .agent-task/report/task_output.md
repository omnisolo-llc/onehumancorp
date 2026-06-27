issue_title: "Implement Offline-Tolerant Mobile-First Edge Sync Architecture"
issue_description: |
  # Research Report: Offline-Tolerant Mobile-First Edge Sync Architecture

  ## Problem Statement
  Small business owners often operate in environments with poor or intermittent internet connectivity. Field service operators (like Carlos, the handyman) and food cart operators (like Fatima) rely on their mobile devices to manage service requests, process orders, and track inventory. A lack of connectivity can lead to lost data, missed bookings, and a disrupted workflow. They need a system that continues to function seamlessly offline and automatically syncs when connectivity is restored, without requiring any manual intervention or technical troubleshooting.

  ## Research Report
  Our research across the e-commerce and POS landscape indicates that offline capabilities are a significant differentiator. While platforms like Shopify and Square offer basic offline POS functionality, they often struggle with complex, distributed inventory or service booking conflicts once back online.

  Competitors:
  - **Shopify POS:** Offers offline card payments and basic order creation, but inventory sync is purely eventual and prone to conflicts.
  - **Square:** Strong offline payment capabilities, but lacks integrated, agentic workflow automation for post-sync reconciliation.
  - **OHC (Current State):** Highly dependent on real-time connectivity to the central backend. High friction for users in low-connectivity zones.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Mobile Client - Flutter PWA] -->|Reads/Writes| B(Local Edge Cache - SQLite/Powersync)
      B -->|Background Sync - CRDTs/Sync Protocol| C{Sync Gateway - API}
      C -->|Read/Write| D[(Central Ledger - PostgreSQL)]
      D --> E[AI Operations Agent]
      E -->|Resolves Conflicts/Alerts| C
      A -.->|Direct API Call when Online| C
  ```

  ### Key Design Decisions
  - **Local-First Data Storage:** Implement a local SQLite database on the mobile client using a solution like PowerSync or a custom CRDT (Conflict-free Replicated Data Type) based sync mechanism.
  - **Optimistic UI:** All UI interactions (creating an order, booking a service) must immediately update the local state and UI, providing instant feedback.
  - **Background Sync Engine:** A robust background process that queues mutations when offline and flushes them to the central server when a connection is established.
  - **AI-Driven Conflict Resolution:** When sync conflicts occur (e.g., Carlos books a time slot offline that was booked online by a customer in the meantime), the AI Operations Agent automatically triages the conflict. It will attempt to reschedule or alert the owner with a clear, actionable card in the Agent Feed.

  ### Mobile UX Flow (375px)
  1.  **Offline State Indicator:** A subtle, non-intrusive indicator (e.g., a small cloud icon with a line through it) shows the app is in offline mode.
  2.  **Action Execution:** The user performs an action (e.g., "Complete Job" and logs payment). The UI immediately reflects the completion.
  3.  **Syncing State:** When connectivity returns, the indicator changes to a syncing animation.
  4.  **Conflict Resolution (if needed):** If the Operations Agent detects a conflict, a priority "Action Card" appears in the Agent Feed: "Scheduling Conflict Detected for Job X. Customer Y also booked this slot. [Draft Message to Y to Reschedule] [Approve]".

  ### AI Agent Integration Points
  -   **Operations Agent ("The Manager"):** Listens for sync conflict events.
  -   **Customer Success Agent ("The Ambassador"):** Drafts apology/rescheduling messages if a conflict affects a customer.

  ## Implementation Prompt
  **Target Persona:** Carlos (Handyman) and Fatima (Food Cart Operator)
  **Objective:** Implement a local-first sync architecture for the OHC mobile client to support seamless offline operations.
  **Critical User Journey (CUJ):**
  1.  Carlos loses cell service while at a job site.
  2.  He marks the job as "Complete" and records a cash payment in the OHC app.
  3.  The app immediately updates the UI to show the job is done.
  4.  Carlos drives back to an area with cell service.
  5.  The app automatically syncs the job completion and payment to the backend.
  6.  The backend verifies the sync and updates the central ledger without Carlos needing to press a "Sync" button.

  **Acceptance Criteria:**
  -   UI must optimistically update for core operations (booking completion, order creation) without network requests.
  -   A background queue must store mutations and sync them reliably when the network is available.
  -   The backend must gracefully handle eventual consistency and trigger the Operations Agent if a conflict cannot be automatically merged.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []