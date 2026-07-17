issue_title: "[Research] Offline-First Mobile Sync Architecture & Local State Resiliency"
issue_description: |
  # Research Report: Offline-First Mobile Sync Architecture & Local State Resiliency

  ## Problem Statement
  Business owners operating in the field or in low-connectivity environments (e.g., Fatima running a food cart with spotty cellular data, or Carlos doing handyman repairs in a basement) struggle with cloud-dependent applications. When network connectivity drops, standard SaaS tools experience endless loading spinners, failed state mutations (lost bookings, unrecorded payments), and data inconsistencies. This cloud-first dependency directly blocks their ability to operate their business and erodes trust in the software.

  ## Research Report
  - **Competitive Landscape:** Platforms like Shopify and Wix are inherently cloud-first. A dropped connection during a critical POS transaction or inventory update typically results in a failure or an unusable app state. Their mobile apps are wrappers around web views or API clients that require constant connectivity.
  - **Modern Solutions:** Industry-leading productivity apps (like Linear or Notion) employ "Local-First" architectures, where the UI reads and writes exclusively to a local database (like SQLite), and synchronization happens asynchronously in the background.
  - **OHC Opportunity:** While OHC's backend has components like JourneyApps PowerSync and a robust PostgreSQL central ledger, the mobile frontend (Flutter/PWA) must adopt a strictly Local-First paradigm. By combining local embedded databases with OHC's AI Agents for conflict resolution, we can provide a zero-latency, 100% offline-capable experience that feels like magic to the non-technical owner.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Mobile Flutter App UI] -->|Read / Write| B[(Local Embedded SQLite DB)]
      B --> C[Background Sync Queue]
      C -->|Network Restored| D(PowerSync / API Gateway)
      D --> E[(OHC Cloud PostgreSQL)]
      D --> F[Operations Agent]
      F -->|Conflict Resolution| E
  ```

  ### Mobile UX Flow (375px)
  1. **Connectivity Loss:** When the network drops, a subtle, unobtrusive grey dot or "Offline mode" badge appears in the top navigation bar. No blocking modals or error dialogues interrupt the user.
  2. **Zero-Latency Mutation:** Carlos taps "Complete Job" and marks an invoice as paid. The UI updates instantly. A success toast appears.
  3. **Local Persistence:** The mutation is written to the local SQLite database and queued for synchronization.
  4. **Background Reconnection:** When Carlos drives out of the basement and regains cellular signal, the background queue synchronizes with the OHC cloud. The offline badge smoothly transitions back to an online indicator.

  ### AI Agent Integration Points
  - **The Operations Agent (Conflict Resolution):** If an offline sale occurs simultaneously with an online web purchase for the final inventory item, the Operations Agent intercepts the sync conflict. It automatically prioritizes the offline POS sale (since physical goods changed hands), updates the remote ledger, and triggers the Customer Success Agent to gracefully notify the online purchaser of the inventory mismatch.

  ## Implementation Prompt
  **Feature Name:** OHC Local-First Mobile Sync Engine
  **Target Persona:** Carlos the Handyman / Fatima the Food Cart Operator
  **Outcome:** The mobile app must remain 100% functional for core CRUD operations (viewing tasks, updating bookings, recording sales) regardless of network status.

  **Next Actions for Engineering:**
  1. Integrate a local embedded database (e.g., SQLite via drift or PowerSync local client) into the Flutter/mobile codebase.
  2. Rearchitect the primary data repositories so that the UI reads and writes *only* to the local database, achieving immediate optimistic UI updates.
  3. Implement a background synchronization queue that reliably pushes local mutations to the OHC backend when network connectivity is available, handling retries with exponential backoff.
  4. Enhance the Operations Agent to detect and logically resolve timestamp/inventory conflicts arising from offline mutations.

  **Acceptance Criteria:**
  - A user on a 375px mobile viewport can toggle "Airplane Mode", create a new booking or complete a task, see the UI update instantly, and have the data successfully sync to the backend once Airplane Mode is disabled.

  **Priority:** P0
  **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
