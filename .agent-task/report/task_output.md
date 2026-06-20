issue_title: "Implement Offline-First Distributed Sync Architecture for Field Service Operations"
issue_description: |
  ## Problem Statement
  Field service owners like Carlos (handyman, operates from Android phone) often work in areas with poor or no cellular connectivity (e.g., basements, remote sites). Currently, OHC relies heavily on real-time API calls to the backend. When Carlos tries to generate a quote, capture a signature, or record a deposit while offline, the app fails or blocks the workflow, causing a loss of trust and operational delays. OHC lacks a robust local-first architecture to queue mutations and synchronize state when connectivity is restored, preventing our target field-service operators from adopting the platform reliably.

  ## Research Report
  - **Competitor Analysis**:
    - **Jobber & ServiceTitan**: Both have robust offline modes. They use local SQLite databases on mobile to cache the day's schedule, customer details, and pricing books. They queue actions (mutations) and sync them once online.
    - **Shopify POS**: Uses a local-first approach to allow offline sales, storing transactions and syncing them back to the central ledger later.
  - **OHC Gap**: OHC currently assumes a stable connection to its Postgres database for most AI and operational workflows. We need a way for the AI quote generator and booking system to function (or degrade gracefully) offline, caching context locally, and seamlessly merging state once the device reconnects.

  ## Design Doc
  - **Architecture**:
    - **Local State (Flutter/PWA)**: Implement a local-first embedded database to mirror a specialized subset of the tenant's data: today's bookings, customer histories, and service pricing catalog.
    - **Mutation Queue**: All writes (e.g., create quote, update booking status, capture signature) are written to a local "Mutation Log" rather than directly calling the network.
    - **Sync Engine**: A background worker monitors network connectivity. When online, it flushes the Mutation Log to the backend.
    - **Conflict Resolution**: The central PostgreSQL database uses timestamp-based last-write-wins or deterministic conflict resolution for overlapping edits.
    - **AI Agent Integration ("The Dispatcher")**: When offline, the app uses a smaller, on-device cached ruleset to draft basic quotes. When it syncs, the full Operations Agent reviews the drafted quotes, enriches them with historical context, and automatically sends the finalized versions to the customers.

  - **Mobile UX Flow (375px)**:
    - Header displays a subtle "Working Offline" translucent pill.
    - User can view their daily route, tap a job, and enter details.
    - When tapping "Generate Quote", the UI instantly shows a locally drafted estimate with a badge "Pending Sync".
    - Once back online, the pill disappears, pending items show a satisfying checkmark animation, and the AI agent takes over the background dispatch.

  ## Implementation Prompt
  As an Implementer Agent, your task is to design and implement the foundation for the Offline-First Sync Engine.
  - Create the backend functionality for receiving batched mutation logs from the mobile client and applying them to the Postgres database with conflict resolution.
  - Ensure the API handles idempotency (using a `client_mutation_id`) so duplicate sync retries don't create duplicate records.
  - Implement a backend test simulating an offline client submitting a batch of job updates and quote generations after a network restore.
  - Do NOT prescribe specific UI state management libraries, but ensure the contract allows the Flutter/PWA client to sync seamlessly.
  - Ensure all database queries respect the multi-tenant row-level security (`tenant_id`).

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
