issue_title: "Autonomous Project Lifecycle & Approval Engine"
issue_description: |
  ### Problem Statement
  Nora, an agency principal, currently lacks a unified way to manage the lifecycle of a client project within OHC. While basic quoting exists, it is disconnected from the intake process and doesn't lead to automated task coordination. Nora needs an assistant-led flow that captures client demand, drafts a professional proposal, allows for mobile-first approval, and automatically kicks off the work (tasks and deposits).

  ### Research Report
  The current OHC codebase contains a `quoting` service in `src/server/services/quoting/mod.rs` which handles basic CRUD for quotes and pricing rules. The `onboarding_agent.rs` includes logic for business intake but stops short of ongoing project management.

  Competitive analysis of platforms like HoneyBook and Dubsado shows that agency owners value speed from inquiry to contract. OHC's "Assistant-First" differentiation should be the AI proactively drafting the proposal based on intake data, rather than Nora having to build it from scratch.

  ### Design Doc

  #### Mermaid ER Diagram
  ```mermaid
  erDiagram
      TENANTS ||--o{ PROJECTS : "manages"
      PROJECTS ||--o{ PROJECT_INTAKES : "originated_from"
      PROJECTS ||--o{ PROPOSALS : "contains"
      PROPOSALS ||--o{ PROPOSAL_LINE_ITEMS : "details"
      PROJECTS ||--o{ TASKS : "executes"

      PROJECTS {
          string id PK
          string tenant_id FK
          string customer_id FK
          string title
          string status "active, completed, archived"
      }

      PROJECT_INTAKES {
          string id PK
          string project_id FK
          json intake_data
      }

      PROPOSALS {
          string id PK
          string project_id FK
          string status "draft, sent, approved, rejected"
          timestamp valid_until
          bigint total_amount_cents
      }

      TASKS {
          string id PK
          string project_id FK
          string title
          string status "todo, in_progress, done"
          string assignee_agent_role
      }
  ```

  #### Sequence Diagram
  ```mermaid
  sequenceDiagram
      participant Client
      participant Nora as Nora (Agency Principal)
      participant OHC as OHC Assistant
      participant Ops as Operations Agent
      participant Finance as Finance Agent

      Client->>OHC: Submits Project Inquiry (Form/DM)
      OHC->>OHC: Process Intake (onboarding_agent)
      OHC->>Nora: Notification: "New project inquiry from Acme Corp. Proposal drafted."
      Nora->>OHC: Reviews Draft Proposal (Mobile UI)
      Nora->>OHC: Edits/Approves Proposal
      OHC->>Client: Sends Proposal & Approval Link
      Client->>OHC: Approves Proposal
      OHC->>Ops: Trigger: "Create Project Tasks"
      OHC->>Finance: Trigger: "Request Project Deposit"
      OHC->>Nora: Notification: "Acme Project Started. Tasks assigned to The Manager."
  ```

  #### Mobile UX Flow (375px)
  1. **Home Screen Feed**: A glass-morphism card in the priority feed: "Review Acme Corp Proposal".
  2. **Proposal Draft View**: Clean layout showing extracted client goals, proposed timeline, and line items.
  3. **Interaction**: Nora can tap any line item to edit or toggle "optional" status.
  4. **Approval Action**: A sticky footer with a "Send to Client" button.
  5. **Client View**: A mobile-responsive web link for the client to review and tap "Approve & Pay Deposit".

  ### Implementation Prompt
  Implement the "Autonomous Project Lifecycle & Approval Engine" for the Nora persona.
  1. Create a new `ProjectService` with the defined ER schema in PostgreSQL.
  2. Enhance the `OnboardingAgent` to detect "Agency" project inquiries, creating a `Project` and a draft `Proposal` automatically.
  3. Develop a Flutter-based mobile UI for Nora to review these drafts. Use the OHC Premium Token library for visual excellence (translucent materials, Ubiquiti-style cards).
  4. Integrate with the `Operations Agent` (The Manager) to generate initial tasks upon proposal approval.
  5. Integrate with the `Accountant` to generate a Stripe payment link for the project deposit.
  6. **Acceptance Criteria**:
     - End-to-end journey from inquiry to task generation is functional without manual data entry.
     - Proposal review screen is 100% usable on a 375px viewport.
     - 100% unit test coverage for `ProjectService`.
     - At least 5 Playwright E2E tests covering the Nora CUJ.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
