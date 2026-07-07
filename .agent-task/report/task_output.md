issue_title: "OHC Autonomous Project Intake & Proposal Agent"
issue_description: |
  # Research Report: Autonomous Project Intake, Proposal Drafting & Client Approval Architecture

  ## 1. Problem Statement
  Service-based businesses, agencies, and independent professionals (e.g., Nora the Agency Principal) often face significant friction in project intake. They spend hours capturing client requirements, drafting proposals manually, seeking approvals, and organizing tasks. Current general platforms (like Shopify or Wix) are built for discrete transactions, not long-term project lifecycles. Traditional CRM and project management tools (like HoneyBook or Dubsado) offer templates but require manual setup and intervention.
  The gap in the market is a unified, agent-driven workflow that autonomously converts a raw client request into a ready-to-sign proposal and subsequent project structure, without requiring the owner to act as a full-time administrator.

  ## 2. Research Report
  - **Market Context**: Platforms such as HoneyBook and Dubsado are popular among service providers because they combine CRM, invoicing, and proposals. However, they rely heavily on static templates and manual trigger configurations. Generative AI tools (like ChatGPT or Jasper) can draft proposals but are disconnected from the user's core operational system (inventory, calendar, pricing, tasks).
  - **The OHC Opportunity**: By introducing an autonomous proposal workflow integrated directly into OHC, we can eliminate the boundary between sales (intake/proposals) and operations (task creation/invoicing). An AI agent can negotiate the details, draft the document, and seamlessly hand off to execution.
  - **Competitor Gaps**:
    - *HoneyBook/Dubsado*: Template-heavy; lacks autonomous conversational intake and proactive drafting.
    - *Shopify/Wix*: Focused entirely on transaction-based commerce, ignoring the proposal-based service sector.
    - *HubSpot*: Highly capable but enterprise-oriented and overly complex for a micro-agency or independent professional.

  ## 3. Design Doc
  ### Data Model & Invariants (PostgreSQL)
  - `ProjectRequest`: Captures raw intent, incoming messages, and extracted requirements.
  - `Proposal`: The structured document drafted by the agent (includes scope, timeline, line items, total cost). Tied strictly to a `tenant_id` for isolation.
  - `Project`: The operational entity created once a `Proposal` is approved. Contains `Tasks`.
  - `Task`: A specific, assignable piece of work under a `Project`.

  ### AI Integration (Department Coordination)
  - **Sales Agent ("The Negotiator")**: Ingests the initial request (via form, email, or DM). It conducts a conversational intake to gather missing details (budget, timeline) and drafts the initial `Proposal`.
  - **Operations Agent ("The Manager")**: Once the `Proposal` is approved by the client, this agent automatically breaks down the scope into a structured `Project` with actionable `Tasks` and deadlines, assigning them to the owner or staff.
  - **Finance Agent ("The Accountant")**: Automatically generates the deposit invoice linked to the approved proposal and schedules milestone payment reminders.

  ### Mobile UX Flow (375px)
  1. **Intake Notification**: The owner receives a push notification: "New project inquiry from Client X. Tap to review."
  2. **Proposal Review**: The owner opens a 375px-optimized card view showing the AI-drafted proposal. The interface uses translucent glass materials (`background: rgba(255, 255, 255, 0.65)`, `backdrop-filter: blur(30px) saturate(210%)`).
  3. **Approval/Edit**: The owner taps "Approve" to send it to the client, or "Edit" to tweak line items using native mobile inputs.
  4. **Post-Approval**: Once the client signs/pays, the owner's dashboard automatically updates to show the new active `Project` and today's `Tasks`.

  ## 4. Implementation Prompt
  **Feature Name**: Autonomous Project Intake & Proposal Agent
  **Target Persona**: Nora the Agency Principal

  **Outcome**: A seamless workflow where a client inquiry is automatically translated into a drafted proposal by the Sales Agent. Upon client approval, the Operations Agent converts the proposal into a project with defined tasks, and the Finance Agent issues the initial invoice.

  **Critical User Journey (CUJ)**:
  1. Nora receives a web form submission: "I need a brand redesign for my new coffee shop."
  2. The Sales Agent creates a `ProjectRequest`, queries Nora's pricing history, and drafts a `Proposal` with standard branding packages.
  3. Nora receives an OHC mobile notification, reviews the drafted `Proposal` in a clean card layout, and taps "Approve & Send".
  4. The client accepts the proposal via a shared link.
  5. The Operations Agent automatically creates a `Project` called "Coffee Shop Branding", populates it with 5 standard tasks, and the Finance Agent emails the deposit invoice.

  **Next Actions for Engineering**:
  1. Implement the `ProjectRequest`, `Proposal`, `Project`, and `Task` entities with strict multi-tenant row-level security.
  2. Build the Sales Agent prompt and workflow to generate structured `Proposal` records from unstructured text inputs.
  3. Create the mobile-first (375px) Proposal Review and Approval UI component using OHC Premium Tokens (Glassmorphism, 16px border-radius containers).
  4. Wire the Operations and Finance Agent triggers to execute upon proposal acceptance.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
