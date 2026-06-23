issue_title: "Architecture Design: Zero-Config Offline-Tolerant State Synchronization & Local Action Queue"
issue_description: |
  # Mission Queue Protocol Brief

  ## Title
  Zero-Config Offline-Tolerant State Synchronization & Local Action Queue

  ## Problem Statement
  For mobile-first operators like Fatima (food cart, limited English, patchy data connection) or Carlos (field service owner in rural areas), network connectivity is unreliable. When they attempt to log a sale, update inventory, or mark a job as complete, a network failure or latency spike can cause the app to hang, show error modals, or lose critical business data.
  Traditional apps block user interaction or show spinners during these moments. OHC requires a zero-configuration, transparent offline mode where the owner continues working uninterrupted, and the system automatically reconciles the local state with the backend ledger and agent workflows once the connection returns.

  ## Research Report
  - **The Mobile-First Disconnect:** Market leaders like Shopify and Wix often struggle with true offline capability for management tasks. Companion apps (e.g., Shopify POS) have some offline capabilities but require explicit setup or separate flows. Link-in-bio tools are purely online.
  - **Modern Solutions:** Apps like Linear and Notion provide robust local-first experiences using advanced sync engines (CRDTs or local SQLite replicas). PowerSync and WatermelonDB are leading frameworks.
  - **The OHC Differentiator:** While local-first ensures UI speed, OHC needs an "Agentic Local Action Queue". When offline, users aren't just saving data; they are queuing operations (e.g., "Draft this proposal" or "Process this deposit"). The local system must securely queue these intents, and upon reconnection, OHC's Operations Agent must process the queue, resolve any conflicts (e.g., an item sold online while the offline terminal sold it in-person), and proactively notify the owner.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      subgraph Mobile Client 375px
          UI[Flutter PWA / iOS / Android UI]
          LocalDB[Local SQLite Database]
          LocalQueue[Local Action Queue]
          UI -->|Reads/Optimistic Writes| LocalDB
          UI -->|Business Intents| LocalQueue
      end

      subgraph OHC Edge & Backend
          Gateway[API / Sync Gateway]
          CentralDB[(PostgreSQL Ledger)]
          JobQueue[AI Job Queue]
      end

      subgraph AI Department
          OpAgent[Operations Agent]
          FinanceAgent[Finance Agent]
      end

      LocalDB <-->|PowerSync Protocol| Gateway
      Gateway <--> CentralDB
      LocalQueue -->|Uploads upon reconnect| Gateway
      Gateway --> JobQueue
      JobQueue --> OpAgent
      OpAgent -->|Conflict Resolution & Reconciliation| CentralDB
      OpAgent -->|Push Notifications for Exceptions| UI
  ```

  ### Mobile UX Flow (375px)
  1. **Offline State Active:** Fatima opens the app in a cellular dead zone. A subtle, non-intrusive indicator (e.g., a "Sync paused" icon in the header) is visible.
  2. **Action Execution:** She marks 3 pre-orders as "Picked Up" and accepts a new offline cash order.
  3. **Optimistic UI:** The app instantly updates to reflect the changes. No spinners, no errors.
  4. **Network Restored:** The app detects a connection. The sync icon spins briefly.
  5. **Conflict Handling:** If the cash order consumed the last inventory item, but someone bought it online concurrently, the Operations Agent receives the queue, identifies the double-book, and sends an Action Card to Fatima's Agent Feed: "Inventory Conflict: Offline sale of 'Chicken Halal Plate' overlapped with an online order. Suggested action: Refund online order or substitute. [Refund] [Substitute]"

  ### AI Agent Integration Points
  - **The Manager (Operations Agent):** Monitors the reconciliation pipeline. When the sync gateway processes the Local Action Queue, the Manager analyzes the sequence of events. It applies business logic to resolve conflicts without bothering the owner unless human judgment is needed.
  - **Agent Feed:** Any unresolvable conflicts or notable batch sync results (e.g., "Successfully synced 15 offline actions and processed $120 in pending sales") are published to the user's Unified Agent Feed as an actionable card.

  ### Key Design Decisions
  - **Local-First Database:** Utilize a local SQLite replica integrated with PowerSync to provide instant read/write capabilities on the device.
  - **Intent-Based Queuing:** Instead of just syncing raw database rows, critical business actions (like processing a payment or completing a task) are stored as "Intents" in a local queue to be executed by the backend safely, preserving invariants and triggering AI workflows correctly.
  - **Invisible Offline Mode:** Users should not need to "enable offline mode". It must be the default behavior of the application shell.

  ## Implementation Prompt
  **User-Facing Outcome:** The OHC mobile application must remain 100% functional for core operations (viewing work, accepting orders, updating status) even when the device is completely offline (simulated via Chrome DevTools or device airplane mode). When the connection is restored, all offline actions must seamlessly sync to the backend.

  **Critical User Journey (CUJ):**
  1. Log into the OHC app as Carlos (Service Owner).
  2. Disable network connectivity.
  3. Navigate to today's schedule and mark a service booking as "Completed".
  4. Create a new quick invoice for the completed job.
  5. The UI must reflect both the completed status and the new invoice immediately.
  6. Re-enable network connectivity.
  7. The app must silently sync the data to the central PostgreSQL database.
  8. Verify on a separate device/browser session that the service is completed and the invoice exists.

  **Acceptance Criteria:**
  - Introduce a local SQLite data layer that acts as the primary data source for the Flutter UI.
  - Implement a sync manager that handles network transitions gracefully.
  - Establish an Intent Queue for operations that require backend AI processing or third-party API calls (e.g., Stripe).
  - Provide a test-bed to easily simulate offline mode and verify sync behavior.
  - No explicit schema or database tables are prescribed; the implementer must design the sync schema.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []