issue_title: "Architectural Design: Unified Agentic Project & Proposal Pipeline for Agency Principals"
issue_description: |
  # Mission Queue Protocol: Agentic Project & Proposal Pipeline

  ## Problem Statement
  Small business operators like Nora (Agency Principal) manage high-value client engagements that require project intake, dynamic proposal drafting, multi-stage approvals, and automated invoice tracking. Currently, OneHumanCorp (OHC) handles basic bookings and unified inbox messaging, but lacks a stateful, long-running project pipeline architecture. Owners are forced to piece together disjointed tasks and documents manually, which breaks the OHC promise of an "assistant-led flow that prevents loose ends."

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Notion/Asana/Monday:** Excellent at task tracking but lack native financial primitives (invoicing, deposits) and require extensive manual setup.
  - **HoneyBook/Dubsado:** Tailored for creatives but rely heavily on static templates and rigid workflows rather than intelligent, agentic generation.
  - **OHC Opportunity:** By introducing a `Proposal` and `ProjectState` entity tied to the existing Ledger and CRM structures, we can utilize the "Sales & Revenue Assistant" to autonomously draft proposals based on client intake DMs, and the "Operations Assistant" to convert approved proposals into tracked project milestones with automated invoice reminders.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Client Intake via Inbox/Form] --> B[Work Triage Agent]
      B --> C[Sales & Revenue Assistant]
      C --> D[(PostgreSQL: Proposals Table)]
      C --> E[Proposal Approval UI]
      E -->|Client Approves| F[Operations Assistant]
      F --> G[(PostgreSQL: Project Milestones Table)]
      F --> H[Ledger & Invoicing System]
      H --> I[Finance & Decision Assistant]
      I --> J[Owner Summary Feed]
  ```

  ### Mobile UX Flow (375px First)
  1. **Feed Notification:** Owner sees a translucent glass card in their feed: "New Intake from Acme Corp. Proposal drafted."
  2. **Proposal Review:** Tapping the card opens a vertical, native-feeling proposal preview. The owner can tap "Edit with AI" to tweak scope or pricing, or hit "Send to Client".
  3. **Client View:** Client receives an SMS/Email link to a branded, mobile-optimized proposal with a "Sign & Pay Deposit" floating action button.
  4. **Active Project Card:** Once paid, the proposal transitions into an Active Project card on the owner's dashboard, showing milestone progress and upcoming payment milestones.

  ### AI Agent Integration Points
  - **Sales & Revenue Assistant:** Triggered by new leads. Analyzes past similar projects and current intake text to draft a line-item proposal and scope of work.
  - **Operations Assistant:** Watches for `proposal.approved` events via the event mesh to generate project milestones and task assignments.
  - **Finance & Decision Assistant:** Schedules Stripe Payment Intents based on milestone completion dates and sends automated follow-ups.

  ### Multi-Tenancy & Security
  - All new tables (`proposals`, `project_milestones`) must strictly include `tenant_id` with Row Level Security (RLS) enabled.
  - State transitions managed via distributed locks (`ohc:lock:{tenant_id}:proposal:{id}`) to prevent double-billing or concurrent edits.

  ## Implementation Prompt
  **To the Implementer:**
  Implement the backend data model and API layer for the Agentic Project & Proposal Pipeline.
  - Create the database migrations for `proposals` and `project_milestones` ensuring RLS is applied.
  - Develop the gRPC/REST API endpoints for creating, retrieving, updating, and transitioning the state of a proposal.
  - Integrate the Sales Assistant to automatically draft proposals when an intake event is published to the event mesh.
  - Build the corresponding mobile-first UI components in the Tauri/Flutter shell using the translucent glass design tokens. Ensure all touch targets are at least 44x44px.
  - **Acceptance Criteria:** A user can receive an intake message, the system auto-drafts a proposal, the user reviews and approves it on a 375px screen, and the proposal successfully transitions to an active project state. Ensure 100% test coverage and Playwright E2E verification.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
