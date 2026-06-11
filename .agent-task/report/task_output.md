issue_title: "[Research] AI Incident Resolution & Escalation Assistant"
issue_description: |
  # AI Incident Resolution & Escalation Assistant

  ## Problem Statement
  Operators like Jun (Location Manager) face complex, fast-moving daily operations. When an issue occurs (e.g., a delivery driver is late, a machine breaks, or a customer dispute escalates), the current process is manual: text the owner, check physical manuals, search past messages, and coordinate a fix while the crisis worsens. There is a gap in existing tools (like Slack or DingTalk) which merely provide communication without active resolution. Owners need an assistant that not only flags the issue but immediately drafts a resolution plan, contacts the right people, and provides a concise executive summary of the escalation.

  ## Research Report
  ### Market Context & Competitor Analysis
  - **DingTalk / Feishu (Lark)**: Excellent at team coordination and structured incident ticketing but require the user to manually drive the process. They alert, but they do not *resolve*.
  - **Slack/Teams**: General-purpose communication. Integrations exist for PagerDuty, but these are IT-centric and far too technical for a food cart operator or boutique owner.
  - **Samsara / Fleetmatics**: For field service, they track assets and flag delays, but do not integrate with customer communication or draft apologies.
  - **AI Opportunity**: Provide an "Operations & Customer Assistant" hybrid that detects anomalies (e.g., from staff input or connected devices), creates a dedicated 'Incident Room', drafts an apology to affected customers, assigns a fix-it task to staff, and summarizes the event for the owner (e.g., "Jun reported the espresso machine broke. I refunded 3 pending orders and texted the repair tech. Approve?").

  ## Design Doc

  ### High-Level Architecture
  The system leverages a specialized `IncidentResolverAgent` coordinated via the central event bus and AI Job Queue.

  ```mermaid
  graph TD
      A[Trigger: Staff Input / Sensor / Triage Feed] --> B(Incident Ingestion API)
      B --> C{IncidentResolverAgent}
      C --> D[Customer Assistant]
      C --> E[Operations Assistant]
      C --> F[Decision Assistant]
      D --> G(Draft Apology / Refund)
      E --> H(Assign Task to Staff/Vendor)
      F --> I(Generate Owner Escalation Summary)
      G --> J[Owner Triage Feed]
      H --> J
      I --> J
      J -->|Approve All| K(Execute Webhooks/DB Updates)
  ```

  ### Mobile UX Flow (375px First)
  1. **Incident Intake (Jun's View)**: A clean, large-button form: "What's wrong?" -> Audio note or quick text ("Espresso machine down").
  2. **Owner Triage Feed (Owner's View)**: A red/orange tinted 'Urgent' card appears at the top of the feed.
     - *Card Text*: "Critical: Espresso Machine Down at Downtown Location. 3 orders affected."
  3. **Resolution Proposal (Bottom Sheet)**: Tapping the card reveals the AI's proposed plan:
     - **Action 1**: Text repair tech (Draft attached).
     - **Action 2**: Refund 3 pending orders and send apology (Drafts attached).
     - **Action 3**: Mark item "Espresso" out of stock on menu.
  4. **One-Tap Execution**: Owner hits "Execute Plan". The translucent UI transitions to a green success state, and all agents dispatch their tasks.

  ### AI Agent Integration Points
  - **Context Memory**: The agent pulls from Knowledge (repair tech contact info), Operations (current pending orders containing espresso), and Sales (menu items).
  - **Concurrency**: Redis distributed locks ensure that if Jun and the Owner try to edit the menu simultaneously during the crisis, state is protected.

  ### Key Design Decisions
  - **Guided Intake**: When stressed, staff can't type paragraphs. Voice-to-text intake is prioritized.
  - **Holistic Resolution**: Instead of one alert, the AI bundles the *operational* fix, the *customer* fix, and the *inventory* fix into a single approval.
  - **No Jargon**: The summary must be plain English. No "Error 500" or "Webhook Failed."

  ## Implementation Prompt
  **Goal:** Build the backend event handling and the owner-facing incident resolution UI card.

  **Critical User Journey (CUJ):**
  1. An incident is logged via a simple API endpoint (simulating staff input).
  2. The `IncidentResolverAgent` analyzes the incident, identifies affected pending tasks/orders, and drafts a comprehensive resolution plan.
  3. The plan appears in the Owner's Triage Feed. The owner reviews the bundled actions and taps "Approve."
  4. The system executes the simulated actions (updating order status, marking inventory out of stock).

  **Acceptance Criteria:**
  - Create the `Incident` entity in the database with strict RLS (`tenant_id`).
  - Implement the `IncidentResolverAgent` logic to aggregate data from multiple domains (orders, inventory) and output a structured resolution plan.
  - Build the mobile-first (375px) Incident Card and Resolution Bottom Sheet in Flutter, using the premium design system.
  - Ensure 100% unit test coverage for the incident logic.
  - Write at least one Playwright E2E test verifying the flow from incident creation to owner approval and final state update.

  ## Priority
  `P1` (High) - Critical for scaling operations and maintaining control during high-stress situations.

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
