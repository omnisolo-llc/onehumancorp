issue_title: "AI-Automated Staff Scheduling & Task Orchestration"
issue_description: |
  ## Problem Statement
  Location managers like Jun spend hours each week manually scheduling staff shifts, handling call-outs, and tracking task completion (e.g., "clean the espresso machine", "stock inventory"). Existing tools (like When I Work or Homebase) are disconnected from the core business operations (sales, bookings, inventory) and require manual oversight. Jun needs an autonomous system that links staff scheduling directly to business demand and dynamically reassigns tasks based on real-time availability and skillsets.

  ## Research Report
  - **Competitors**:
    - *Homebase/When I Work*: Good for simple scheduling but operate in silos. They don't know if a busy period is coming based on online bookings or pre-orders.
    - *Sling*: Offers task management but lacks AI-driven autonomous reassignment.
    - *Square Team Management*: Integrates with POS but is reactive, not proactive.
  - **OHC Opportunity**: By integrating staff scheduling natively with OHC's Operations Agent, we can predict busy periods using historical data and online pre-orders, auto-generate optimal schedules, and autonomously route tasks to on-shift staff via the Teammate Mesh.
  - **Data & References**: Poor scheduling and task management lead to a 15-20% decrease in operational efficiency in quick-service and retail environments.

  ## Design Doc
  ### System Overview
  ```mermaid
  graph TD
      subgraph Mobile UI "Tauri App (375px)"
          ManagerView[Manager Dashboard]
          StaffView[Staff Task Feed]
      end

      subgraph Backend "Rust + PostgreSQL"
          API[Staff Mesh API]
          ScheduleEngine[Scheduling Engine]
          TaskRouter[Task Router]
      end

      subgraph AI Agents
          OpsAgent[Operations Agent]
          AdvisorAgent[Advisor Agent]
      end

      subgraph DB
          Postgres[(PostgreSQL)]
      end

      ManagerView --> API
      StaffView --> API
      API --> ScheduleEngine
      API --> TaskRouter
      ScheduleEngine --> Postgres
      TaskRouter --> Postgres

      OpsAgent --> ScheduleEngine : Auto-generates schedule based on demand
      OpsAgent --> TaskRouter : Dispatches real-time tasks
      AdvisorAgent --> ManagerView : Flags staffing shortages
  ```

  ### Mobile UX Flow (375px First)
  1. **Manager View (Jun)**: Sees a unified feed of the current shift. Any unassigned tasks or call-outs are flagged by the Ops Agent with a proposed solution (e.g., "Call in Sarah, she's available").
  2. **Staff View**: A simple checklist of tasks for their shift, with push notifications for real-time adjustments (e.g., "Rush hour: register 2 needs backup"). 44x44px touch targets for checking off tasks.

  ### AI Agent Integration Points
  - **Operations Agent (The Manager)**: Predicts staffing needs based on booked appointments/pre-orders. If an employee calls out via SMS, the agent parses the message, updates the schedule, and drafts a shift-cover request to available staff.
  - **Advisor Agent**: Analyzes labor costs vs. revenue in real-time and provides recommendations (e.g., "Labor cost is 45% today, consider sending one person home early").

  ## Implementation Prompt
  **Feature Name**: OHC Autonomous Staff Scheduler & Task Router
  **Target Persona**: Jun the Location Manager
  **Outcome**: Jun no longer manually builds schedules. The Ops Agent creates them based on demand. Staff receive clear, dynamic task lists on their phones, and call-outs are handled autonomously.

  **Next Actions**:
  1. Implement `Shift`, `StaffProfile`, and `Task` models in PostgreSQL with multi-tenant isolation.
  2. Develop the Operations Agent capability to auto-generate schedules based on historical order volume and upcoming bookings.
  3. Build the Mobile-First (375px) Manager and Staff views in the Tauri app.
  4. Integrate Twilio SMS for autonomous shift-cover requests (Ops Agent -> Staff).

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
