issue_title: "Implement Staff Operations and Task Coordination (Jun Persona)"
issue_description: |
  # Research Report: Staff Operations & Task Coordination System

  ## 1. Problem Statement
  Location Managers like Jun (the given persona in the initial prompt) manage the day-to-day operations of a physical site. They don't own the company but own the daily result. They struggle with coordinating staff, delegating tasks, escalating issues, tracking supply needs, and providing daily summaries to the owner. Traditional SaaS tools are either purely HR focused or too generic, lacking native integration with the business's core operational agent layer.

  ## 2. Research Report
  - **Market Context:** Existing platforms for small businesses (e.g., Homebase, Sling, Square Team Management) handle scheduling well but fail at contextual task execution. When an order comes in or a pickup is delayed, these tools don't automatically assign the corrective task to an available staff member.
  - **The OHC Opportunity:** OHC's Operations Agent ("The Manager") can natively orchestrate staff tasks. By implementing a unified Staff capability, OHC can assign dynamic tasks (e.g., "Table 4 needs a refill," "Restock flour," "Handle unhappy customer at register") and create a feedback loop that rolls up to a shift summary for the owner.
  - **Competitor Gaps:**
    - *Square:* Strong point-of-sale team management, but no AI agent to suggest or coordinate impromptu tasks.
    - *Homebase/Sling:* Excellent for scheduling and time-clocks, but disconnected from actual business demand signals (orders, messages, inventory).

  ## 3. Design Doc
  ### Data Model (PostgreSQL)
  - `StaffMember`: Represents an employee or contractor, linked to a Tenant. Includes roles (e.g., barista, delivery driver, shift lead).
  - `Shift`: Represents a block of time a StaffMember is working.
  - `Task`: A specific, actionable item assigned to a StaffMember or Shift, with states (Pending, In Progress, Completed, Blocked).
  - `Escalation`: A specific type of task or alert meant for the location manager (Jun) or owner.

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Location Manager App] -->|Create Escalation| B(Task API)
      B --> C[PostgreSQL: Tasks & Escalations]
      C --> D{Operations Agent}
      E[Real-time Events: Inventory/Orders] --> D
      D -->|Assign Dynamic Task| F[Staff Member App]
      D -->|End of Shift| G[Daily Shift Summary]
  ```

  ### AI Integration
  - **Operations Agent ("The Manager"):**
    - Monitors real-time events (e.g., low inventory, new custom cake order) and generates `Task` records.
    - Analyzes completed tasks and shift events to generate a plain-language "Daily Shift Summary" for Jun to send to the owner.
    - Handles escalation: If a task remains incomplete or a customer complaint is logged, the agent flags it as an `Escalation`.

  ### Mobile UX Flow (375px)
  1. **Staff View (The "Now" Screen):** A highly focused, large-touch-target screen showing only the tasks assigned to the current user right now. "Swipe to complete" interaction.
  2. **Manager View (Jun's Dashboard):** A holistic view of who is on shift, current task completion rates, and an "Attention Needed" section for escalations.
  3. **End of Shift:** An AI-generated draft of the shift report that Jun can review, edit, and send with one tap.

  ## 4. Implementation Prompt
  **Feature Name**: OHC Staff Task Coordination System
  **Target Persona**: Jun the Location Manager
  **Outcome**: Jun can manage a shift where the Operations Agent automatically assigns tasks based on business events, tracks completion, and generates an owner-ready shift summary, all from his phone.

  **Next Actions**:
  1. Implement the `StaffMember`, `Shift`, and `Task` database entities with strict multi-tenant isolation.
  2. Create the "Staff View" UI with large touch targets for completing assigned tasks on a 375px screen.
  3. Create the "Manager View" UI for Jun to see active shifts and pending escalations.
  4. Extend the Operations Agent to generate tasks dynamically based on simulated business events and to draft the end-of-shift summary.

  **Priority**: P1
  **Estimated Scope**: Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
