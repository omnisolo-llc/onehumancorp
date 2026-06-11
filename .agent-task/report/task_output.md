issue_title: "[Research] OHC Autonomous Work Scheduling Architecture"
issue_description: |
  # Research Report: OHC Autonomous Work Scheduling Architecture

  ## Problem Statement
  Service-based SMBs (like Carlos the Handyman or Nora the Agency Principal) suffer from inefficient routing, overbooking, and chaotic daily schedules. Existing booking tools (Calendly, Shopify apps) treat time merely as a product, relying entirely on the customer to find an empty slot. They do not optimize for travel time, job duration variance, or staff availability. Business owners spend hours each week playing "calendar Tetris" instead of doing the actual work.

  ## Research Report (Track 1 & 2)
  - **Market Context**: Most platforms (Square Appointments, Wix Bookings) use static time-blocking. If a job finishes early or late, the entire day is disrupted.
  - **The OHC Opportunity**: We can move from *passive booking* to *autonomous scheduling*. By integrating location data, job duration estimates, and real-time staff status, OHC's Operations Agent can proactively assemble the most efficient route and schedule.
  - **Competitor Gaps**:
    - *Jobber / ServiceTitan*: Powerful but overly complex and expensive, targeted at larger fleets, not micro-SMEs.
    - *Shopify/Wix*: No concept of geographic routing or dynamic job durations.

  ## Design Doc (Track 3)
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer Request] --> B{Operations Agent}
      B -->|Checks| C[PostgreSQL: Appointments & Staff]
      B -->|Calculates| D[Routing Engine / Distance API]
      B -->|Applies| E[Redis: Temporary Time Lock]
      B --> F[Propose Optimized Slot to Customer]
      F -->|Accepted| G[Confirm Booking & Update Ledger]
  ```

  ### Data Model
  - `JobTemplate`: Defines estimated duration, skills required, and base price.
  - `StaffProfile`: Work hours, skills, and current location.
  - `Appointment`: State machine (Requested, Scheduled, En-Route, In-Progress, Completed, Cancelled).
  - `RouteOptimizer`: Agent protocol that recalculates the daily schedule when an appointment state changes.

  ### Mobile UX Flow (375px)
  - **The "Daily Run" Screen**: Carlos opens the app to see a clear, chronological list of today's jobs, optimized for driving distance.
  - **One-Tap Actions**: Large, 44x44px touch targets to change state (e.g., "Heading to Job", "Start Work", "Job Done").
  - **Agent Intervention**: If Carlos marks a job "Done" 30 minutes early, the Operations Agent sends a push notification: *"You're ahead of schedule. Want me to text the next client to see if we can arrive early?"*

  ## Implementation Prompt (Track 4)
  **Target Persona**: Carlos the Handyman
  **Outcome**: Carlos receives an optimized daily schedule that minimizes driving time. The Operations Agent handles schedule adjustments automatically if a job runs over or finishes early.

  **Next Actions for Engineering**:
  1. Design the database schema for `Appointments`, `JobTemplates`, and `StaffProfiles` with strict multi-tenant isolation.
  2. Implement the "Daily Run" mobile view (375px optimized) with state transition buttons.
  3. Develop the Operations Agent skill to parse new booking requests, evaluate travel time/distance (using a mocked or lightweight geo-service initially), and propose the optimal time slot to the customer.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
