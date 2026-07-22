issue_title: "Autonomous Staff Coordination & Escalation Engine"
issue_description: |
  # Research Report: Autonomous Staff Coordination & Escalation Engine

  ## 1. Problem Statement
  Location managers (like Jun, 31) are responsible for day-to-day operations at a specific site but do not own the overall company. They struggle to coordinate staff tasks, handle local customer feedback, manage supply levels, and surface critical issues to the owner effectively. Current SMB tools treat staff management as a separate HR/scheduling app, detached from the daily operational reality, customer feedback loop, and inventory system.

  ## 2. Research Report
  - **Market Context**: Platforms like Shopify and Wix focus primarily on commerce and lack robust shift/task management out of the box. Dedicated scheduling tools (like When I Work or Homebase) do not natively integrate with the POS, inventory, or customer feedback loops. This forces location managers to constantly switch contexts and manually compile daily summary reports for the owner.
  - **The OHC Opportunity**: By introducing an Autonomous Staff Coordination & Escalation Engine, OHC can unify task assignment, local feedback, and supply monitoring. The AI can automatically flag issues that require owner intervention and compile end-of-day summaries, transforming the location manager's experience from reactive firefighting to proactive management.
  - **Competitor Gaps**:
    - *Homebase / 7shifts*: Excellent for scheduling but completely separated from the core business engine (orders, customer complaints).
    - *Shopify POS*: Has basic staff PINs but lacks task assignment and AI-driven issue escalation.
    - *DingTalk / WeCom*: Comprehensive but can feel overly corporate and complex for a simple food cart or single boutique location.

  ## 3. Design Doc
  ### Architecture Diagram (Data Model & Invariants)

  ```mermaid
  erDiagram
      TENANT ||--o{ LOCATION : operates
      LOCATION ||--o{ STAFF : employs
      LOCATION ||--o{ TASK : has
      LOCATION ||--o{ ESCALATION : generates
      STAFF ||--o{ TASK : assigned_to

      TENANT {
          string id PK
          string name
      }
      LOCATION {
          string id PK
          string tenant_id FK
          string name
          string timezone
      }
      STAFF {
          string id PK
          string location_id FK
          string name
          string role "Manager | Staff"
      }
      TASK {
          string id PK
          string location_id FK
          string assignee_id FK
          string description
          string status "Pending | In Progress | Completed"
          datetime due_date
      }
      ESCALATION {
          string id PK
          string location_id FK
          string context
          string severity "Low | Medium | High"
          string status "Open | Resolved"
      }
  ```

  ### AI Agent Coordination
  - **Operations Agent ("The Manager")**: Monitors task completion rates and inventory thresholds. If tasks are consistently missed or supplies run low, it automatically drafts an escalation.
  - **Customer Success Agent ("The Ambassador")**: Analyzes local customer feedback. If a spike in complaints occurs (e.g., "long wait times"), it automatically creates an urgent task for the location manager and an escalation summary for the owner.
  - **Business Advisory Agent**: Compiles the daily end-of-day report, summarizing completed tasks, resolved escalations, and overall location health for the owner.

  ### Mobile UX Flow (375px First)
  1. **Jun's Dashboard (Location Manager)**: A clean, premium glassmorphism interface prioritizing "Today's Tasks", "Low Supplies", and "Recent Feedback". Touch targets are large (≥ 44x44px).
  2. **Task Assignment**: Jun taps a task to assign it to an on-shift staff member. The state updates optimistically (supporting offline-first capabilities if network drops).
  3. **Escalation Trigger**: If Jun encounters an issue beyond his authority (e.g., broken POS hardware), he taps "Escalate to Owner". The Operations Agent drafts a concise summary.
  4. **Owner's View**: The owner sees a unified feed across all locations, with Jun's escalation highlighted in a muted red pill component for immediate attention.

  ### Key Design Decisions
  - **Location-Scoped Tenancy**: Data must be partitioned not just by `tenant_id`, but also logically scoped by `location_id` to ensure staff only see what is relevant to their site.
  - **Agent-Driven Reporting**: Instead of forcing managers to write reports, the AI agents synthesize data from tasks, POS, and feedback to generate owner-ready summaries.

  ## 4. Implementation Prompt
  **Feature Name**: Autonomous Staff Coordination & Escalation Engine
  **Target Persona**: Jun the Location Manager

  **Outcome**: Jun can manage daily site tasks and seamlessly escalate critical issues to the owner, with AI agents automatically summarizing local performance and customer feedback.

  **Next Actions for Engineering**:
  1. Implement the core Data Models (`Location`, `Staff`, `Task`, `Escalation`) with strict multi-tenant and location-based isolation in PostgreSQL.
  2. Develop the Mobile-First Task & Escalation UI in Flutter, ensuring optimistic updates and large touch targets for fast-paced environments.
  3. Extend the Operations and Customer Success Agents to monitor local activity, draft escalations, and compile the daily location summary.

  **Acceptance Criteria**: Jun can create and assign a task on his mobile device. Jun can trigger an escalation that the AI agent summarizes. The owner receives the escalated summary in their feed. All UI must function correctly on a 375px viewport.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
