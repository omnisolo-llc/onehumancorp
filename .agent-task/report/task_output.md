issue_title: "OHC Autonomous Agency Project Intake & Milestone Billing Engine"
issue_description: |
  # Research Report: Autonomous Agency Project Intake & Milestone Billing Engine

  ## 1. Problem Statement
  Service-based agency owners and independent professionals (e.g., Nora the Agency Principal) struggle with the fragmented lifecycle of client projects. Currently, turning a new lead into a structured project requires jumping between CRMs for intake, document editors for proposals, project management tools for task assignment, and separate accounting software for milestone billing. This context switching causes missed follow-ups, delayed invoicing, and loss of "owner memory" across the project lifecycle.

  ## 2. Research Report
  - **Market Context**: Platforms like Notion AI or ClickUp provide excellent project tracking but lack native financial primitives (invoicing/payments). Financial tools like QuickBooks or Stripe handle billing but don't manage the day-to-day work tasks or client intake. Specialized agency tools (like HoneyBook or Dubsado) attempt to merge these, but they rely heavily on manual template creation and lack proactive, agent-driven management.
  - **The OHC Opportunity**: By natively integrating project tracking, document generation, and payment milestones, and powering them with OHC's AI Agents, we can eliminate the "glue work" of agency management.
  - **Competitor Gaps**:
    - *HoneyBook/Dubsado*: Good for solo freelancers, but often require heavy manual setup of workflows and templates; lack multi-agent proactive intelligence.
    - *Monday/Asana*: Excellent task management, zero native financial/billing integration.
    - *Stripe*: Perfect billing, zero project/task context.

  ## 3. Design Doc

  ### Architecture (Mermaid.js)

  ```mermaid
  erDiagram
      TENANTS ||--o{ PROJECTS : "manages"
      PROJECTS ||--o{ TASKS : "contains"
      PROJECTS ||--o{ MILESTONES : "bills via"
      TENANTS ||--o{ CUSTOMERS : "serves"
      CUSTOMERS ||--o{ PROJECTS : "commissions"

      PROJECTS {
          string id PK
          string tenant_id FK
          string customer_id FK
          string title
          string status "intake, proposal, active, completed"
          timestamp created_at
      }

      MILESTONES {
          string id PK
          string tenant_id FK
          string project_id FK
          string title
          bigint amount_cents
          string status "draft, pending, paid"
          string payment_link
      }

      TASKS {
          string id PK
          string tenant_id FK
          string project_id FK
          string description
          string assignee_id
          string status "todo, in_progress, done"
      }
  ```

  ### Mobile UX Flow (375px)
  1. **Project Intake**: The owner (Nora) reviews a new project request in the Work Triage feed.
  2. **Proposal Draft**: With one tap, the Sales Agent drafts a proposal and sets up initial payment Milestones based on similar past projects.
  3. **Project Board**: A clean, touch-friendly 375px Kanban-style view of project Tasks and their associated Milestones.
  4. **Milestone Approval**: When a phase is complete, the Finance Agent pushes a notification to Nora: "Phase 1 complete. Send $5,000 milestone invoice?" One tap approves and sends the Stripe payment link.

  ### AI Agent Integration
  - **Work Triage / Sales Agent**: Parses incoming client emails/forms to automatically generate a draft Project scope and proposed Milestones.
  - **Operations Agent**: Monitors Task completion. When all tasks linked to a Milestone are marked "done", it alerts the Finance Agent.
  - **Finance Agent**: Automatically schedules and drafts invoice reminders for pending Milestones, awaiting the owner's final 1-tap approval before sending.

  ## 4. Implementation Prompt
  **Feature Name**: OHC Autonomous Project & Milestone Billing Engine
  **Target Persona**: Nora the Agency Principal
  **Outcome**: Nora can accept a new client lead, have OHC draft a proposal, auto-generate project tasks, and automatically prompt her to send milestone invoices as work is completed—all from her phone.

  **Next Actions**:
  1. Implement the core Data Models (`Project`, `Task`, `Milestone`) with strict multi-tenant isolation in PostgreSQL.
  2. Develop the Mobile-First Project Dashboard UI (375px Kanban view and Milestone tracker).
  3. Integrate the Sales Agent capability to parse intake text and draft initial Project scopes and Milestones.
  4. Integrate the Operations and Finance Agents to coordinate Task completion with Milestone billing prompts via push notifications.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
