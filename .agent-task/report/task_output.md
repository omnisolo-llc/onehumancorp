issue_title: "Architecture Design: Zero-Touch Automated Intake & Proposal Drafting System (The Closer)"
issue_description: |
  # Title: Zero-Touch Automated Intake & Proposal Drafting System (The Closer)

  ## Problem Statement
  Service professionals and agency principals (like Nora, the Agency Principal, and Carlos, the Field Service Owner) spend countless unbillable hours processing project inquiries, scoping requirements, and drafting quotes or proposals. Traditional CRM and quoting tools (like HubSpot or QuickBooks) are highly manual: the owner must parse the client's email, mentally estimate the scope, manually construct line items, and chase approvals. This friction leads to delayed responses, lost deals, and administrative burnout. A non-technical owner needs a system that seamlessly turns a messy client request into a professional, actionable proposal without manual data entry.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **HubSpot / Salesforce:** Enterprise-grade but overwhelmingly complex for micro-agencies. They act as passive databases requiring manual state changes.
  - **HoneyBook / Dubsado:** Popular among creatives but rely heavily on static templates and manual questionnaire reviews. They lack the autonomous intelligence to draft a unique proposal based on natural language intake.
  - **QuickBooks / FreshBooks:** Excellent for accounting but poor at the initial sales and scoping phase; estimates are manual.
  - **OHC Opportunity:** By leveraging the Sales & Revenue Assistant (The Closer) alongside the Operations Assistant, OHC can intercept an inquiry (via email, web form, or DM), autonomously extract the project requirements, cross-reference past similar projects in the Knowledge database to estimate costs, and draft a ready-to-send proposal. The owner receives an "Action Required" card in their mobile feed to simply review, adjust, and approve.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Client Inquiry: Email/Form/DM] -->|Webhook| B(Omnichannel Intake Gateway)
      B --> C{Intake Intent Analyzer}
      C -->|Project Request| D[The Closer Agent]
      D -->|Query Past Scopes| E[(Knowledge & History DB)]
      D -->|Check Availability| F[(Operations & Resource DB)]
      D -->|Draft Proposal| G[Proposal Generation Engine]
      G --> H[Action Required Queue]
      H --> I[Owner Mobile Feed 375px]
      I -->|1-Tap Approve or Edit| J[Proposal Dispatcher]
      J --> K[Stripe Billing/Invoice Preparation]
      J --> L[Client Email/SMS with Secure Link]
  ```

  ### Mobile UX Flow (375px First)
  1. **Intake Notification:** The owner (Nora) receives a push notification and sees a priority card in her OHC feed: "New Project Request: Brand Redesign for Acme Corp".
  2. **Review Draft:** Tapping the card opens a split view.
     - *Top:* A summary of the client's messy request (e.g., "We need a new logo and website...").
     - *Bottom:* The AI-generated proposal draft, complete with suggested line items (Logo Design: $1,500, Web Dev: $3,000), timeline, and a deposit request.
  3. **Adjustment (Native UI):** Line items are presented as touch-friendly cards (≥ 44x44px). Nora can slide to adjust pricing or tap to add/remove a generated task.
  4. **Approval:** A prominent, sticky "Approve & Send" button at the bottom of the viewport. Upon tapping, the system emails the client a beautiful web-hosted proposal with an integrated Stripe checkout for the deposit.
  5. **Offline/Low-Data Resilience:** The drafted proposals are cached locally. If Nora approves while on the subway (offline), the action is queued locally and dispatched via eventual consistency once the connection is restored.

  ### AI Agent Integration Points
  - **The Closer (Sales Agent):** Analyzes the raw unstructured intake text, extracts key entities (budget, timeline, deliverables), and constructs the proposal data model.
  - **The Archivist (Knowledge Agent):** Provides context to The Closer by retrieving similar past projects (e.g., "The last 3 brand redesigns took 40 hours and billed $4,500").
  - **The Manager (Operations Agent):** Checks current staff capacity to ensure the proposed timeline is feasible before the draft is presented to the owner.

  ### Key Design Decisions
  - **Zero-Touch Generation:** The AI creates a complete draft rather than presenting the owner with a blank template and a blinking cursor. It is always easier to edit than to create from scratch.
  - **Multi-Tenant Data Isolation:** Proposals, intake inquiries, and pricing intelligence are strictly partitioned using PostgreSQL Row-Level Security (`tenant_id`) and distributed locks to ensure no cross-contamination of agency pricing data.
  - **Idempotent Dispatch:** The proposal approval and sending mechanism utilizes idempotent keys to prevent double-sending if network conditions are flaky during the mobile approval tap.

  ## Implementation Prompt

  **Target Persona:** Nora (Agency Principal) & Carlos (Field Service Owner)

  **User-Facing Outcome:** When a potential client sends a messy request, the owner opens the OHC app to find a fully drafted, priced, and scheduled proposal waiting for their 1-tap approval.

  **Critical User Journey (CUJ) & Acceptance Criteria:**
  1. Create the `Proposal` and `ProposalLineItem` data models with strict tenant isolation.
  2. Implement the ingestion pipeline that routes a "Project Request" event to the Sales Agent.
  3. Develop the AI generation logic (using the standard LLM provider interface) that parses the request, queries historical pricing context, and outputs structured proposal JSON.
  4. Build the mobile-first (375px) UI for reviewing and editing the drafted proposal, utilizing the OHC Premium Token design system (translucent materials, clean layout).
  5. Integrate with Stripe to automatically generate a checkout session for the required deposit upon client acceptance.
  6. Ensure full E2E Playwright test coverage simulating the owner receiving the draft, modifying a line item price, and sending the proposal.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
