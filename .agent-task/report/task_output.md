issue_title: "Core Architecture: Offline-First Mobile Sync Engine for Field Operations"
issue_description: |
  ## Title
  Core Architecture: Offline-First Mobile Sync Engine for Field Operations

  ## Problem Statement
  For business owners like Carlos (a field service owner who runs a repair and home-improvement service entirely from an Android phone), consistent internet access is a luxury, not a guarantee. When Carlos is in a client's basement, out in rural areas, or experiencing a flaky mobile network, he still needs to view service requests, log route notes, accept estimates, and mark tasks as complete.

  Currently, operations that require constant network connectivity fail silently, leave the app in a hanging state, or lose critical business data when the connection drops. The owner needs an assistant that works offline seamlessly, storing their actions locally and syncing them invisibly in the background the moment a connection is restored, without ever showing technical error messages like "Network Timeout" or "API Error".

  ## Research Report
  ### Market Context & Competitor Analysis
  - **ServiceTitan & Jobber**: These vertical field-service tools heavily invest in offline mobile capabilities. They cache daily schedules and customer data locally so technicians can work in dead zones, syncing work orders when they return to coverage.
  - **Shopify POS & Square**: Both offer offline modes for taking payments and queuing operations. Square can cache credit card transactions and sync them later, ensuring business doesn't stop during outages.
  - **Notion AI / Mobile**: Employs a local-first architecture for document editing, using CRDTs to merge changes when coming back online.
  - **Gap in current SMB platforms (Wix, GoDaddy, Squarespace)**: Most of these platforms assume a constant connection for backend management. A web-first approach means their mobile administrative apps often break when offline.

  **Opportunity for OHC**: By building a resilient, offline-first mobile synchronization engine, OHC can capture the massive demographic of on-the-go operators (Carlos, Fatima) who are poorly served by web-dependent SaaS tools.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant App as Mobile App (375px)
      participant LocalDB as Local Store (SQLite/IndexedDB)
      participant SyncQueue as Background Sync Engine
      participant API as OHC API Gateway
      participant DB as Multi-Tenant Postgres
      participant Agent as Operations Agent

      App->>LocalDB: Read cached daily schedule
      App->>LocalDB: Write task update (e.g. "Completed")
      LocalDB->>SyncQueue: Enqueue mutation event
      App-->>App: Optimistic UI Update (Translucent Success Card)

      Note over SyncQueue,API: When network is restored
      SyncQueue->>API: Flush mutation queue (w/ Idempotency Key)
      API->>DB: Apply mutation
      API->>Agent: Trigger subsequent actions (e.g. Draft invoice)
      API-->>SyncQueue: Ack sync success
      SyncQueue->>LocalDB: Clear queued event
  ```

  ### UI Wireframes & Screen Flow (375px)
  1. **Feed View**: The main Agent Feed displays Carlos's tasks for the day. A subtle top banner or icon (e.g., a unified Ubiquiti-style token) indicates "Working Offline - Changes Saved".
  2. **Task Detail View**: Carlos taps a task to view customer details (cached locally) and taps "Mark Complete".
  3. **Optimistic Update**: A clean, premium Translucent Glass confirmation appears: "Task Complete. We'll update the system when you're back online." The task card turns green immediately.
  4. **Background Sync**: Once signal returns, the sync indicator disappears, and a notification silently confirms: "Sync complete. Operations Assistant is drafting your invoice."

  ### Mobile UX Flow
  - **Zero Blockers**: No loading spinners for local writes. The UI updates optimistically.
  - **Truthful States**: When offline, features that strictly require server-side AI (like drafting a completely new custom proposal from scratch) gracefully disable with a clear, non-technical explanation ("Connect to internet to generate new AI proposals").
  - **Conflict Resolution**: Last-write-wins at the field level, or AI-assisted conflict resolution if a team member updated the same record.

  ### AI Agent Integration Points
  - **Operations Assistant**: Subscribes to the delayed sync events. When the offline tasks finally hit the server, the Operations Assistant processes them sequentially to avoid overwhelming the owner with out-of-order notifications.
  - **Work Triage**: If an offline sync results in a conflict (e.g., Carlos marked a job complete but the customer cancelled it 10 minutes ago online), the Work Triage agent generates an Action Card for Carlos to decide how to proceed.

  ### Key Design Decisions
  - **Local-First Writes**: All critical operations (status changes, notes, capturing leads) write to a local persistent store first.
  - **Idempotency**: Every sync request must include an idempotency key to prevent double-processing if the mobile connection drops during the sync API call.
  - **Optimistic UI**: Essential for the "grandmother test" — the app must feel fast and responsive regardless of underlying network conditions.

  ## Implementation Prompt
  **For the Implementer Agent:**
  Your mission is to implement the local-first caching and sync queue for the Mobile Tasks & Feed interface.

  **User Facing Outcome**: When Carlos (or any user) loses internet connection, they can still view their daily tasks, mark them as complete, and add notes. The app will save these changes instantly in the UI and automatically sync them to the backend when connection is restored, without throwing any network errors.

  **CUJ**:
  1. User opens the app while online to load the daily feed.
  2. User goes offline (e.g., airplane mode).
  3. User completes a task and adds a note. The UI immediately reflects the success state.
  4. User goes back online. The app silently syncs the task completion to the backend.
  5. The UI accurately reflects that the data is synchronized and backed up.

  **Acceptance Criteria**:
  - Introduce a local persistence layer for the core Feed/Task data.
  - Implement a background sync queue that captures state mutations.
  - Update the mobile views (375px) to use optimistic updates and provide clear offline indicators using our premium design tokens.
  - Ensure API calls triggered by the sync queue use idempotency keys.
  - Provide complete E2E tests (using Playwright) demonstrating the flow: load data, disconnect network (mock offline state in Playwright), mutate state, reconnect network, and verify backend sync.
  - No database schemas or specific API endpoint designs are prescribed here; design them to best fit the multi-tenant architecture.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
