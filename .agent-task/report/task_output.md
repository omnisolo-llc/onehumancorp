issue_title: "Implement Autonomous Staff Coordination & Shift Resolution System"
issue_description: |
  # Research Report: Autonomous Staff Coordination & Shift Resolution System

  ## Problem Statement
  Jun (Location Manager, 31) manages daily operations for a physical site. When a team member calls in sick or requests sudden time off, Jun loses 30-60 minutes manually texting other staff to find coverage, updating the calendar, and reassigning daily tasks. Existing POS or scheduling tools require manual interaction to broadcast open shifts and lack deep integration with the daily operational task list. Jun needs an automated assistant to handle this operational friction.

  ## Research Report
  - **Competitor Analysis:** Tools like 7shifts, Homebase, and When I Work provide shift broadcasting and messaging. However, they are passive tools. The manager still has to open the app, draft a message, select eligible employees, and monitor responses.
  - **The OHC Opportunity:** By leveraging the Operations Agent, OHC can detect a "Shift Dropped" event (e.g., an employee texts the OHC system "I'm sick"), autonomously verify eligible team members who aren't working and haven't exceeded overtime, draft a shift-coverage request, and surface a 1-tap approval card to Jun via the Unified Agent Feed.

  ## Design Doc
  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  graph TD
      A[Employee SMS/App: 'I am sick'] --> B(Event Mesh Gateway)
      B --> C[Operations Agent]
      C -->|Query| D[PostgreSQL: Schedule & Staff DB]
      C -->|Identify Replacements| E[Operations Logic]
      E -->|Draft Coverage Request| F[Action Required Queue]
      F --> G[Unified Agent Feed Mobile 375px]
      G -->|1-Tap Approve| H[Dispatch SMS to Replacements]
      H --> I[Staff Accepts Shift]
      I --> J[Auto-update Schedule & Notify Jun]
  ```

  ### Mobile UX Flow (375px)
  1. Employee sends an SMS or in-app message dropping their shift.
  2. The system triggers an event to the Operations Agent.
  3. Jun receives an "Action Required" card in his Unified Agent Feed: "Sarah called out sick for today's 2 PM shift. I found 3 eligible staff members to cover. Send coverage request?"
  4. Jun taps "Approve & Send".
  5. The Operations Agent sends the requests to the staff via SMS.
  6. When a staff member accepts, the Operations Agent updates the central schedule and pushes an FYI notification to Jun.

  ### AI Agent Integration Points
  - **Operations Agent ("The Manager"):** Listens for absence notifications, queries schedule and staff constraints (overtime, roles), and manages the lifecycle of the coverage request.

  ### Key Design Decisions
  - Shift management must be natively integrated with the Unified Agent Feed, not a separate "Calendar" module that needs manual monitoring.
  - SMS fallback for staff communication is crucial, as retail/food staff may not have the OHC app installed.

  ## Implementation Prompt
  **User-Facing Outcome:** When a staff member drops a shift, the location manager receives a single card in their Agent Feed proposing to text eligible replacements. One tap sends the requests, and the system auto-updates the calendar when someone accepts.

  **CUJ & Acceptance Criteria:**
  1. Trigger a simulated "Shift Dropped" webhook event.
  2. Verify that the Operations Agent correctly identifies eligible replacement staff from the database.
  3. Verify that an "Action Required" card appears in the Unified Agent Feed with the drafted coverage request.
  4. Tapping "Approve & Send" successfully marks the coverage request as active and mocks sending SMS messages.
  5. Provide Playwright E2E tests covering the flow: shift drop event, seeing the draft in the feed, approving it, and verifying the schedule state.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
