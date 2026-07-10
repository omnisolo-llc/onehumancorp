issue_title: "Architecture: Agentic Offline-First Field Service Route & Job Management"
issue_description: |
  ## Title
  Agentic Offline-First Field Service Route & Job Management

  ## Problem Statement
  For Carlos, a field service owner (e.g., handyman, repairman), cellular connectivity in customer homes, basements, and remote areas is frequently unreliable or non-existent. Currently, OHC lacks a dedicated offline-first architecture for field service workers. If Carlos loses signal, he cannot view job details, update job status, record customer signatures, or accept offline payments. This creates friction, lost data, and prevents adoption in the field service sector.

  ## Research Report
  - **Persona:** Carlos (Field Service Owner). Relies purely on an Android phone. Needs route notes, estimates, offline functionality, and scheduling sync.
  - **Competitor Analysis:** ServiceTitan, Jobber, and Housecall Pro offer offline modes. They use local SQLite databases combined with an event-sourcing model to queue mutations locally when offline and sync when online. They often lack deep AI integration to summarize routes or draft follow-up messages automatically based on field notes.
  - **Opportunity:** OHC can implement a robust Offline-First PWA/Flutter sync architecture for the Operations Agent. It will allow seamless job status updates offline. When back online, the Agent will resolve conflicts, automatically notify customers of arrival times for the next job, and prepare invoices.

  ## Design Doc (Architecture)

  ### Data Model & Sync Protocol
  - **Local Storage (Flutter/PWA):** Use an embedded local database (e.g., Isar or SQLite via sqflite in Flutter) as the primary read/write target for the mobile UI.
  - **Event Queue (Outbox Pattern):** All state mutations (e.g., `UpdateJobStatus`, `AddJobNote`, `CaptureSignature`) are written as discrete events to a local Outbox table.
  - **Sync Engine (Background):** A background service monitors network state. When online, it flushes the Outbox queue to the backend via a `/sync/mutate` API endpoint.
  - **Conflict Resolution (Backend):** The Operations Agent receives conflicting events and uses Operational Transformation (OT) or Last-Write-Wins (LWW) per field. It can escalate complex conflicts to the owner's feed (Work Triage).

  ### AI Agent Integration
  - **The Operations Agent:** Manages the sync queue. If a job is marked "Completed" offline and synced later, the Operations Agent automatically calculates travel time for the next job and texts the next customer the updated ETA.
  - **The Customer Assistant:** Reads synced field notes and drafts a follow-up "Thank you / Review request" email.

  ### Mobile UX Flow (375px)
  1. **Daily Route Screen:** Displays a vertical timeline of today's jobs.
  2. **Job Details Screen:** Shows address, contact info, issue description, and a map snippet (cached offline). Large 44x44px action buttons: "Start Travel", "Start Job", "Complete Job".
  3. **Offline Indicator:** A subtle top banner (translucent glass) indicates "Offline Mode - Changes saved locally" with a sync pending icon.
  4. **Job Completion Flow:** Carlos taps "Complete", enters a quick note, captures a signature, and the UI immediately updates optimistically.

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant MobileUI as Flutter Mobile UI
      participant LocalDB as Local SQLite (Isar)
      participant SyncEngine as Mobile Sync Engine
      participant BackendAPI as Go Backend (/sync)
      participant OpsAgent as Operations Agent
      participant Postgres as Central DB

      MobileUI->>LocalDB: Read Daily Route (Offline)
      MobileUI->>LocalDB: Write Mutation: UpdateJobStatus(Done)
      LocalDB-->>MobileUI: Optimistic UI Update
      MobileUI->>LocalDB: Write to Local Outbox

      Note over SyncEngine: Network Restored
      SyncEngine->>LocalDB: Read Pending Events
      SyncEngine->>BackendAPI: POST /sync/events
      BackendAPI->>Postgres: Apply Events & Detect Conflicts
      BackendAPI->>OpsAgent: Trigger Post-Job Workflows (ETA, Invoices)
      OpsAgent->>Postgres: Update downstream states
      BackendAPI-->>SyncEngine: Sync Ack
      SyncEngine->>LocalDB: Clear Outbox
  ```

  ## Implementation Prompt
  **User Facing Outcome:** Carlos can view his daily route, tap into a job, read the notes, mark it as completed, and write a quick summary note, all while deep in a customer's basement with zero cellular service. When he drives away and regains signal, the app automatically syncs, the next customer gets a text that he's on the way, and an invoice is generated.

  **Acceptance Criteria:**
  - [ ] Implement a local data caching layer (SQLite/Isar) for the `Job` and `Route` entities.
  - [ ] Implement a generic local Outbox table to store offline mutations.
  - [ ] Implement the Go backend API `/sync/events` to ingest offline mutations.
  - [ ] Hook up the Operations Agent to trigger downstream workflows (ETA SMS, Invoice generation) upon receiving a delayed "Job Completed" sync event.
  - [ ] Write a Playwright E2E test simulating a user toggling offline mode, completing a job, toggling online mode, and verifying the backend state and Agent actions.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
