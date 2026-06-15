issue_title: "Autonomous Staff & Contractor Task Orchestration Architecture"
issue_description: |
  **Title**: Autonomous Staff & Contractor Task Orchestration Engine

  **Problem Statement**:
  Operators like Jun (Location Manager) and Nora (Agency Principal) spend hours daily manually assigning tasks, tracking progress, and managing escalations. Existing SMB tools (like Homebase or Sling) offer manual scheduling, but lack intelligent, proactive work routing. OHC needs an invisible agentic architecture that automatically turns demand (e.g., a new project or an incoming delivery) into prioritized tasks, routes them to available staff/contractors, and provides an owner-ready summary, eliminating the "micromanagement" tax.

  **Research Report**:
  - **Competitor Analysis**: Tools like DingTalk, Feishu/Lark, and Microsoft Copilot provide task assignment but require heavy manual setup and technical administration. Wix and Shopify lack native operational task routing for staff.
  - **Market Gap**: SMBs don't want a "project management tool"; they want an "assistant" that just tells staff what to do based on incoming demand, adapting in real time.
  - **OHC Differentiation**: Utilizing the Operations Agent ("The Manager"), OHC can parse an incoming accepted quote or service booking and automatically fan it out into localized tasks for staff, adjusting for real-time availability and notifying Jun or Nora only on exceptions.

  **Design Doc**:
  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  graph TD
      A[Demand Source: Approved Quote / Booking] --> B[Event Mesh]
      B --> C{Operations Agent - The Manager}
      C --> D[Staff Availability Cache - Redis]
      C --> E[Task Dispatch & Routing Engine]
      E --> F[(PostgreSQL - Task Ledger)]
      E --> G[Staff Mobile View - 375px]
      G -->|Task Completion/Escalation| H[Status Webhook]
      H --> C
      C -->|End of Day| I[Owner Vitality Feed - Summaries]
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **Staff Work View**: A clear, distraction-free mobile screen. The current assigned task is a prominent translucent glass card. A single swipe action marks it "Done" or "Issue".
  - **Owner/Manager View (Jun/Nora)**: A high-level radar. Instead of a list of 50 tasks, they see: "3 Staff Active. 1 Task Escalated (Supply Missing). 85% On Track." Tapping "Escalated" brings up an AI-drafted resolution (e.g., "Draft a message to supplier?").
  - **Mobile UX**: Touch targets are at least 44x44px. The design uses OHC Premium Tokens (Apple/Ubiquiti-style), clean hierarchy, and offline-tolerant state caching so staff in basements or poor network zones can still check off tasks.

  ### AI Agent Integration Points
  - **Operations Agent ("The Manager")**: Listens for new demand events, reads the required skill tags, and assigns tasks using the PostgreSQL Task Ledger and Redis Redlock for staff scheduling conflicts.
  - **Decision Assistant ("The Analyst")**: Consumes task completion events to generate the "End of Day Summary" for the owner.

  ### Key Design Decisions
  - **Zero-Setup Routing**: The system infers task requirements from the invoice or booking, eliminating the need for owners to manually build task templates.
  - **Offline Resilience**: Mobile client uses eventual consistency. Staff actions are cached locally and synced when network returns.
  - **Tenant Isolation**: Strict RLS on `tasks` and `staff_availability` tables.

  ### Product-Use Evidence & Dogfooding (Simulated)
  - **Persona**: Jun (Location Manager)
  - **Flow Attempted**: Log in, view approved booking, navigate to team schedule, and attempt to assign tasks.
  - **Gap Observed**: The current interface only allows viewing the booking. Jun must manually text staff or use a third-party app to assign the physical work. This breaks the "One Assistant" promise.
  - **Post-Fix Expected Flow**: The booking is automatically broken into tasks and dispatched. Jun sees a single "Tasks Auto-Assigned" card on the home feed.

  **Implementation Prompt**:
  **User-Facing Outcome**: Jun receives an accepted quote for a major clean-up. The Operations Agent automatically creates 5 sub-tasks, assigns them to the 3 staff members on shift today based on their roles, and pushes notifications to their phones. Jun only sees an update when a task is blocked.
  **CUJ & Acceptance Criteria**:
  1. Implement the `Task` and `StaffAvailability` PostgreSQL tables with row-level security (`tenant_id`).
  2. Implement a `TaskRoutingService` that the Operations Agent uses to automatically distribute sub-tasks based on an incoming `Project` or `Booking` event.
  3. Create a mobile-first `Staff Work View` UI component that displays assigned tasks with swipe-to-complete interactions.
  4. Build Playwright E2E tests: A manager logs in, views a booking, and the system shows AI-assigned tasks to simulated staff. A simulated staff completes a task, and the manager feed updates.
  5. Ensure 100% unit test coverage for the task distribution algorithm and API routes.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
