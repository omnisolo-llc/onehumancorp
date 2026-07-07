issue_title: "[research] Autonomous Proposal to Invoice Lifecycle Engine"
issue_description: |
  # Autonomous Proposal & Invoice Lifecycle Engine

  ## Problem Statement
  Service-based and agency owners (like Nora the Agency Principal and Carlos the Field Service Owner) spend an inordinate amount of time managing the lifecycle of a project before and after the actual work is done. They manually draft proposals from templates, send them for client approval, manually create tasks once approved, and later manually follow up on unpaid invoices. Existing platforms (like HoneyBook or QuickBooks) require complex workflow setups, disconnected tools, and do not proactively draft documents or chase payments autonomously.

  ## Research Report
  **Competitor Analysis:**
  - **HoneyBook / Dubsado:** Provide proposal and invoice workflows but require extensive manual template creation and trigger setup. They lack an active AI that understands the context of an Instagram DM and immediately generates a highly specific proposal.
  - **QuickBooks / Xero:** Excellent for accounting but terrible for initial client intake and proposal drafting. They are reactive, waiting for the user to input data.
  - **OHC Opportunity:** By utilizing our AI agents (The Ambassador for intake, The Salesperson for proposals, The Accountant for invoices), we can collapse the "Intake -> Proposal -> Approval -> Task Creation -> Invoice -> Payment Collection" lifecycle into a single, zero-click autonomous flow where the owner only needs to tap "Approve" on their mobile feed.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Client Intake via DM/Form] --> B[The Ambassador Agent]
      B --> C{Context & Intent DB}
      C --> D[The Salesperson Agent]
      D -->|Drafts| E[Proposal & Estimate]
      E --> F[Owner Mobile Feed - 375px]
      F -->|1-Tap Approve| G[Client Approval Gateway]
      G -->|Client Approves| H[Operations Agent]
      H -->|Generates| I[Project Tasks & Schedule]
      H --> J[The Accountant Agent]
      J -->|Schedules & Sends| K[Invoices & Reminders]
      K -->|Payment via Stripe| L[Ledger & Reconciliation]
  ```

  ### Mobile UX Flow (375px First)
  1. **Intake Notification:** Owner receives a mobile notification: "Nora, you have a new brand design request from Acme Corp."
  2. **Proposal Draft Card:** The Agent Feed displays a card with a drafted proposal (scope, timeline, price) based on Nora's past similar projects and current availability.
  3. **1-Tap Action:** The card features a primary "Approve & Send Proposal" button (minimum 44x44px touch target) and a secondary "Edit Details" button.
  4. **Invoice Auto-Pilot:** Once the client accepts and signs via a mobile-friendly web link, the feed updates to "Project Acme Corp active. Tasks created. Deposit invoice sent."

  ### AI Agent Integration Points
  - **The Salesperson:** Analyzes the intake request against past successful proposals and pricing models stored in the tenant's memory to draft a highly accurate initial proposal.
  - **The Operations Agent:** Triggers automatically upon proposal acceptance to block out calendar time and generate a checklist of operational tasks.
  - **The Accountant:** Monitors project milestones (if configured) or time elapsed to automatically draft and send invoice reminders, intelligently varying the tone based on the client's payment history.

  ### Key Design Decisions
  - **Zero-Setup Templates:** The system does not ask the user to build templates. It uses LLMs to synthesize past work and standard industry practices into a bespoke proposal for each client.
  - **Unified State Machine:** A single database entity `ProjectLifecycle` tracks the state from lead to paid invoice, allowing all AI agents to share the exact same context without race conditions.

  ## Implementation Prompt
  **User-Facing Outcome:** When a new service request comes in, the owner opens the OHC app to find a complete proposal already drafted. They tap "Approve," and the system handles client signature, initial deposit collection, and task creation without any further manual data entry.

  **CUJ & Acceptance Criteria:**
  1. Intake request hits the system (e.g., via a webhook from a website form).
  2. The Salesperson Agent generates a `Proposal` entity linked to the prospect.
  3. The drafted Proposal appears as an Action Card on the mobile feed (375px layout).
  4. The user taps "Approve" -> The proposal is sent to the client via email/SMS link.
  5. The client opens the link, accepts the proposal, and pays the deposit via Stripe.
  6. The system automatically transitions the state, creates `Task` entities for the Operations Agent, and queues the final `Invoice`.
  7. Automated Playwright E2E tests must verify the owner approval flow and the client acceptance flow on mobile viewport settings.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
