issue_title: "[Research] Autonomous Field Service Estimator & Routing"
issue_description: |
  # Research Report: Autonomous Field Service Estimator & Dynamic Routing

  ## 1. Problem Statement
  Field service owners (e.g., Carlos the Handyman, HVAC technicians, cleaners) operate almost entirely from their mobile phones while driving between jobs. Their primary pain points are losing leads because they cannot answer the phone or generate quotes while working, and inefficient scheduling that causes them to crisscross the city, wasting fuel and time.
  Existing solutions (like Jobber or Housecall Pro) are powerful but complex. They require the owner to manually build quotes, map routes, and manage calendars. They are software suites to administer, rather than assistants doing the work.

  ## 2. Research Report
  - **Market Context**: The home service market is highly fragmented, with millions of sole operators or micro-teams. Speed to lead is critical; if an owner doesn't respond to a quote request within 15 minutes, the customer moves to the next name on Google or Yelp.
  - **The OHC Opportunity**: OHC can differentiate by offering "Zero-Touch Quoting" and "Agentic Routing". Instead of the owner generating a quote, the Sales/Operations Agent receives a request (e.g., via SMS, web form, or voice bot), asks clarifying questions, generates an estimate based on the owner's pricing model, and dynamically slots the job into the most geographically efficient time window on the calendar.
  - **Competitor Gaps**:
    - *Jobber/Housecall Pro*: High cognitive load; requires manual data entry and schedule optimization by the user.
    - *Thumbtack/Yelp*: Pure lead generation, offering no operational or routing support.
    - *Google Local Services*: Good for visibility, but disjointed from scheduling and quoting.

  ## 3. Design Doc
  ### Data Model (PostgreSQL)
  - `ServiceArea`: Geographic regions the owner serves (e.g., zip codes or a radius from a central point).
  - `Estimate`: A generated quote linked to a Customer, ServiceType, and potentially media (e.g., photos uploaded by the customer).
  - `JobRoute`: A sequence of confirmed Bookings for a given day, optimized for travel time.
  - `LocationContext`: Cached geocoordinates for job sites.

  ### AI Integration
  - **Sales Agent (Intake)**: Interacts with the customer (via SMS/web) to gather details. E.g., "You need a faucet replaced. Can you send a picture of the current one?" Uses LLM vision capabilities to assess the job complexity.
  - **Operations Agent (Router)**: Once an estimate is approved, it reviews the owner's calendar and existing job locations. It proposes a booking slot that minimizes drive time (e.g., "I have a technician in your neighborhood on Tuesday at 2 PM").

  ### Architecture Diagram
  ```mermaid
  erDiagram
    CUSTOMER ||--o{ ESTIMATE : requests
    ESTIMATE ||--o| BOOKING : becomes
    BOOKING ||--|| LOCATION_CONTEXT : happens_at
    JOB_ROUTE ||--|{ BOOKING : contains
    OWNER ||--o{ JOB_ROUTE : executes
    OWNER ||--o{ SERVICE_AREA : defines
    SALES_AGENT }|--|{ ESTIMATE : drafts
    OPERATIONS_AGENT }|--|{ JOB_ROUTE : optimizes
  ```

  ### Key Design Decisions and Why
  - **Agentic Generation over Manual Forms**: Instead of having Carlos fill out a quote form, the Sales Agent interacts directly with the customer and uses vision capabilities to generate the quote. This removes the administrative burden from Carlos while he is driving or working.
  - **Dynamic Routing integrated with Bookings**: Jobber requires manual route planning. By having the Operations Agent automatically slot new approved estimates into the `JobRoute` based on geographic proximity, we minimize drive time without Carlos having to act as a dispatcher.
  - **Mobile-First Owner Feed**: The design avoids traditional calendar views. Carlos needs a "Today's Route" view that acts like a GPS navigator (e.g. Map pins + "Navigate to Next Job" button), acknowledging that his primary work environment is behind the wheel of a truck on a 375px phone screen.

  ### Mobile UX Flow (375px)
  1. **Owner View (The Feed)**: Carlos opens the app and sees:
     - An alert: "New estimate approved: Faucet Replacement ($150). Scheduled for Tuesday 2 PM."
     - A "Today's Route" card showing a map with pins 1, 2, 3, and a "Navigate to Next Job" button that opens Apple/Google Maps.
  2. **Customer View**: Receives an SMS link to a lightweight, branded mobile page showing the estimate, a photo of the problem, and a one-tap "Accept & Book" button.

  ## 4. Implementation Prompt
  **Feature Name**: Autonomous Field Service Estimator & Routing
  **Target Persona**: Carlos the Handyman
  **Outcome**: Carlos can focus on repairing homes while the OHC Sales Agent handles incoming quote requests, uses customer-provided photos to generate estimates, and the Operations Agent dynamically schedules approved jobs to optimize his daily driving route.

  **Next Actions**:
  1. Design the `Estimate` and `JobRoute` database schemas.
  2. Build the Sales Agent capability to conduct intake (text + image processing) and generate standard estimates.
  3. Implement the Operations Agent logic to calculate driving distance/time (using a basic heuristic or external API) and suggest optimal scheduling slots.
  4. Create the mobile-first "Today's Route" UI card for the owner's feed, including deep links to native navigation apps.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
