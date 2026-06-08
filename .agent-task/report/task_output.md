issue_title: "Offline-First Mobile Field Service Operations & AI Route Optimization"
issue_description: |
  ## Title
  Offline-First Mobile Field Service Operations & AI Route Optimization

  ## Problem Statement
  Field service owners like Carlos (Handyman) operate primarily from their mobile devices (Android) in environments with unreliable internet connectivity (e.g., inside basements or remote areas). Currently, OHC lacks a robust offline-first architecture that allows field workers to view their schedules, access route notes, draft estimates, and capture signatures without an active connection. Furthermore, the routing of service calls is manual, leading to inefficient travel times and missed opportunities for immediate follow-ups.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Jobber / ServiceTitan:** These platforms offer offline modes for field workers to view jobs and capture notes, but their interfaces are complex and lack proactive AI assistance. They function as digital clipboards rather than intelligent assistants.
  - **Square Appointments:** Good for fixed locations but lacks offline robustness and route optimization specifically tailored for field services.
  - **OHC Opportunity:** By leveraging our PowerSync integration and an offline-first Flutter architecture, OHC can provide a seamless offline experience. The "Operations Assistant" can proactively optimize daily routes, draft estimates based on offline notes taken during a visit, and queue them for sync once connectivity is restored.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Mobile App - 375px] -->|Reads/Writes Local SQLite| B(PowerSync Local DB)
      B <-->|Background Sync when Online| C[PowerSync Service]
      C <-->|Replication| D[PostgreSQL Central Ledger]
      D --> E[Event Mesh]
      E --> F[Operations Agent]
      E --> G[Sales Agent]
      F -->|Route Optimization| D
      G -->|Draft Estimates| D
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **Daily Roster Feed:** A clean, touch-friendly list of today's jobs, cached locally.
  - **Job Detail View:** Displays customer details, service requested, and past history. Includes large touch targets for "Start Job", "Add Note", and "Complete".
  - **Offline Indicator:** A subtle, non-intrusive indicator (e.g., a small cloud with a slash icon) showing the app is offline but fully functional.
  - **Draft Estimate Card:** Upon completing a job and adding notes, the app generates a draft estimate locally (if AI model is edge-capable) or queues the notes. Once online, the Sales Agent drafts the final estimate and pushes a notification for approval.

  ### AI Agent Integration Points
  - **Operations Agent (The Route Manager):** Analyzes the day's jobs and location data to suggest the most efficient route. It dynamically updates the schedule if a job is cancelled or takes longer than expected.
  - **Sales Agent:** Reads the unstructured notes taken by the field worker (e.g., "Customer needs new piping under sink") and drafts a professional, itemized estimate.

  ### Key Design Decisions and Why
  - **Offline-First with PowerSync:** Ensures zero disruption to the user's workflow regardless of connectivity. The app must function identically whether online or offline.
  - **Agentic Drafting:** The user shouldn't have to manually create an estimate from scratch after a long day. The AI does the heavy lifting, turning quick notes into professional quotes.
  - **Mobile-First UX:** Emphasize large touch targets (>= 44x44px) and high contrast for visibility in outdoor environments.

  ## Implementation Prompt
  **User-Facing Outcome:** Carlos starts his day, opens the OHC app, and sees his optimized route. He goes to a job site with no signal, completes the work, jots down a quick note about needing a follow-up repair, and marks the job complete. When he drives back to a coverage area, the app silently syncs. He then receives a notification: "Draft estimate ready for follow-up repair." He taps "Approve" and the quote is sent to the customer.

  **Critical User Journey (CUJ) & Acceptance Criteria:**
  1. The user logs into the app and views the daily roster while online.
  2. The network connection is simulated as offline.
  3. The user opens a job, adds notes, and marks it as complete. The UI must reflect these changes instantly without errors.
  4. The network connection is restored. The local changes must sync to the central database automatically.
  5. The AI agent processes the synced notes and generates a draft estimate, which is surfaced as an actionable card in the user's feed.
  6. E2E tests must verify the offline data capture, the subsequent sync, and the generation of the draft estimate card.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
