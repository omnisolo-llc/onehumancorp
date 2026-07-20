issue_title: "[Architecture] AI-Driven Staff Task Delegation & Shift Coordination Engine"
issue_description: |
  ## Title
  AI-Driven Staff Task Delegation & Shift Coordination Engine

  ## Problem Statement
  Business owners (like Priya) and location managers (like Jun) struggle to translate daily demand—such as incoming orders, customer bookings, and restock needs—into actionable task lists for their staff. Currently, this coordination happens via chaotic WhatsApp groups, sticky notes, or whiteboards. This disconnect leads to missed tasks, uneven workloads, and a lack of accountability. Traditional software requires the manager to manually create and assign tasks, which is time-consuming and reactive.

  ## Research Report
  - **Traditional Point-of-Sale (Square/Shopify POS)**: They track sales and sometimes employee time-clocks, but they do not automatically generate or assign operational tasks based on business activity.
  - **Task Management Apps (Asana/Trello/Homebase)**: Require manual entry by the manager. Disconnected from the actual sales and booking data.
  - **OHC Opportunity**: Leverage the "Operations Agent" (The Manager) to automatically bridge the gap between demand and execution. When a new order arrives, inventory runs low, or a VIP customer books an appointment, the system autonomously creates the necessary tasks, assigns them to the appropriate on-shift staff member, and tracks completion without the manager needing to intervene.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Demand Events: Orders/Bookings/Inventory] -->|Event Bus| B(Operations Agent - The Manager)
      B --> C{Staff Availability & Skill Engine}
      C -->|Query| D[Shift Roster DB]
      B -->|Generates Task| E[Task Queue DB]
      E --> F[Staff Mobile Interface - 375px]
      F -->|Task Completed| G[Ledger/State Update]
      G -->|End of Shift Summary| H[Manager Dashboard - Jun]
  ```

  ### Mobile UX Flow (375px First)
  1. **Staff View (Employee Mode)**: When an employee clocks in via the OHC app, they are presented with a radically simple "My Shift" feed.
  2. **Action Cards**: Tasks appear dynamically (e.g., "Prepare Order #104 for Pickup at 2 PM", "Restock Vanilla Extract"). Large touch targets (44x44px minimum) for "Mark Done" or "Report Issue".
  3. **Manager View (Jun's Feed)**: Jun sees an aggregated "Shift Health" card. Instead of micromanaging, Jun is only alerted to anomalies (e.g., "Order #104 is 15 minutes late").
  4. **Design Tokens**: Adheres to the OHC Premium Token library. High-contrast translucent cards for critical tasks, fading to subtle backgrounds for completed items.

  ### AI Agent Integration Points
  - **The Manager (Operations Agent)**: Subscribes to the core event bus. Uses LLM-driven logic to break down complex events (e.g., "Large Catering Order Received") into granular sub-tasks ("Prep Ingredients", "Pack Boxes", "Load Van") and distributes them based on the current shift roster and staff roles.
  - **The Advisor (Decision Agent)**: Analyzes task completion times over the week to generate Jun's "Owner-Ready Summary" (e.g., "Staff is overwhelmed between 12 PM - 2 PM; consider adding a part-time shift").

  ### Key Design Decisions
  - **Event-Driven Task Creation**: Tasks are generated natively from business data, not manual data entry.
  - **Role-Based Routing**: The system knows that "Baking" tasks go to the kitchen staff, while "Customer Follow-up" goes to the front desk.
  - **Zero-Friction Completion**: Staff must be able to clear a task in one tap. No complex status dropdowns or mandatory notes unless an issue is reported.

  ## Implementation Prompt
  **User-Facing Outcome**: As a location manager (Jun), I want the system to automatically tell my on-shift employees what to do based on live orders and bookings, so I can focus on customer experience instead of micromanaging task lists.

  **CUJ & Acceptance Criteria**:
  1. Create the `ShiftRoster` and `TaskQueue` data models in PostgreSQL, ensuring multi-tenant row-level security (`tenant_id`).
  2. Implement an event listener where a mock "New Order" event triggers the Operations Agent.
  3. The Operations Agent successfully parses the event and creates a corresponding `Task` assigned to an available staff member on shift.
  4. Build the 375px "Staff Mode" view where the assigned task appears.
  5. Playwright E2E Tests: A user logs in as staff, sees the dynamically generated task in their feed, taps "Complete", and the manager's dashboard reflects the updated state.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
