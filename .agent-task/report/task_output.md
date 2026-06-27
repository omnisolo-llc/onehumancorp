issue_title: "Autonomous Staff Scheduling & Task Orchestration"
issue_description: |
  **Title**: Autonomous Staff Scheduling & Task Orchestration

  **Problem Statement**:
  Jun, a location manager, spends hours every week coordinating staff schedules, handling call-outs, and matching staffing levels to expected demand. Existing platforms (like Shopify or Wix) offer no native staffing capabilities. Specialized third-party tools (like Homebase or 7shifts) treat scheduling as a standalone utility disconnected from live POS data, inventory, and operations. Crucially, when a staff member calls out sick, Jun must scramble manually to find coverage, disrupting his management of the location.

  **Research Report**:
  - **Competitor Landscape**: Tools like Homebase, 7shifts, and Deputy are industry standards for shift scheduling. However, they operate in silos, requiring managers to manually create schedules and individually approve shift swaps or find covers.
  - **AI Integration Gaps**: Competitor AI integration is mostly limited to basic predictive sales forecasting. None offer an autonomous executing agent that can handle the end-to-end communication of a call-out and replacement.
  - **OHC Opportunity**: By deeply integrating staff scheduling with OHC's Operations Agent and unified business data, OHC can eliminate the administrative burden of shift management. The Operations Agent can autonomously handle SMS-based call-outs, query staff availability, and instantly propose optimal shift coverage to the manager in a zero-friction mobile workflow.

  **Design Doc**:

  *Architecture Diagram (Mermaid.js)*
  ```mermaid
  graph TD
      A[Inbound SMS: Staff Call-Out] -->|Twilio Webhook| B(Omnichannel Gateway)
      B --> C{Operations Agent: Intent Parser}
      C -->|Identifies Call-Out| D[Query DB: Shift & Staff Availability]
      D --> E[Draft Reassignment Proposal]
      E --> F[Push Action Card to Manager's Feed]
      F -->|Manager Taps Approve| G[Update Shift Ledger in PostgreSQL]
      G --> H[Dispatch SMS to Replacement Staff]
  ```

  *Mobile UX Flow (375px First)*
  1. Jun's phone buzzes with a native push notification: "Action Required: Shift Coverage."
  2. Jun opens the OHC app to the Agent Feed and sees a new Translucent Glass Action Card:
     - **Context**: "Sam called out sick for tomorrow's 8:00 AM Barista shift."
     - **AI Proposal**: "Alex is available, hasn't reached overtime, and has Barista skills. Reassign shift to Alex?"
  3. The card presents two large, touch-friendly buttons (minimum 44x44px): **[Approve & Notify]** (Primary, Green) and **[Find Someone Else]** (Secondary).
  4. Jun taps "Approve & Notify," resolving the crisis in seconds without typing a single message.

  *AI Agent Integration Points*
  - **Operations Agent ("The Coordinator")**: Monitors shift coverage, processes inbound staff communications (e.g., text messages regarding sickness or delays) via Gemini/GPT-4o intent classification, and safely queries the PostgreSQL `StaffAvailability` and `Shift` tables to draft proposals.
  - **Multi-Tenant Data Isolation**: Staff data (phone numbers, availability) is strictly isolated using PostgreSQL row-level security (`tenant_id`). Locks (Redis Redlock) prevent double-booking staff across overlapping shifts.

  **Implementation Prompt**:
  - **Feature Name**: Agentic Shift Coverage & Staff Coordination
  - **Target Persona**: Jun the Location Manager
  - **Outcome**: A manager can handle a last-minute staff call-out and secure a replacement with a single tap on their mobile phone, completely mediated by the Operations Agent.
  - **Critical User Journey (CUJ)**:
    1. A simulated staff member (Sam) sends an SMS: "I'm sick and can't make my shift tomorrow."
    2. The Twilio webhook ingests the SMS. The Operations Agent classifies the intent as a shift call-out.
    3. The Agent identifies the impacted shift, queries available staff with matching skills (finding Alex), and drafts a reassignment action.
    4. Jun receives an Action Card in his mobile agent feed detailing the call-out and the proposed coverage.
    5. Jun taps "Approve & Notify".
    6. The system updates the `Shift` record and dispatches an SMS to Alex confirming the new shift.
  - **Acceptance Criteria**: Must include Playwright E2E tests simulating the inbound SMS webhook, the generation of the Action Card in the 375px mobile UI, and the successful approval flow updating the PostgreSQL backend. The UI must contain zero mock data.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
