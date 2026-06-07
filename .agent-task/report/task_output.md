issue_title: "AI-Driven Dynamic Appointment Routing & Field Dispatch Engine"
issue_description: |
  # AI-Driven Dynamic Appointment Routing & Field Dispatch Engine

  ## Problem Statement
  Service professionals like Carlos (The Freelance Handyman, 42) and Leo (The Music Tutor, 22) face significant friction managing their time and logistics. Carlos needs to optimize his route and booking times so he isn't driving back and forth across town. He currently loses leads when he is on a job because he cannot quickly calculate if he can fit a new appointment into his existing route and schedule. He needs an intelligent dispatch engine that works entirely from his Android phone, automatically quoting time windows, factoring in travel times, and handling deposits without him needing to manually cross-reference Google Maps and Calendly.

  ## Research Report
  **Competitive Analysis:**
  - **Jobber / Housecall Pro:** Heavy, expensive desktop-first dispatching tools built for multi-truck companies. Too complex for a solopreneur like Carlos.
  - **Calendly / Acuity Scheduling:** Excellent at pure time-blocking but completely unaware of physical geography, travel time, or dynamically optimizing a daily route.
  - **Wix Bookings / Square Appointments:** Provide basic calendar booking, but lack the AI capability to dynamically propose optimal slots based on the service location and real-time transit data.

  **Gaps Identified:**
  OHC lacks a unified, geo-aware, AI-driven dispatch system. The platform needs an architecture that integrates mapping data, the user's real-time calendar, and the Customer Success AI to automatically triage and route incoming service requests. The owner should just see a cleanly organized day, while the AI negotiates the exact time slots with the client to minimize travel overhead.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD;
      subgraph Mobile Device
          App[OHC Mobile App 375px] --> CalendarUI[Daily Route & Schedule UI];
      end

      App -- "Accept Booking" --> Gateway[OHC API Gateway];

      Gateway --> DispatchEngine[Dynamic Dispatch Engine];
      DispatchEngine --> GeoService[Maps/Routing API];
      DispatchEngine --> MainDB[(Cloud Postgres Ledger)];

      Gateway --> Agents[AI Agent Swarm];

      subgraph Agent Departments
          Agents --> SalesAgent[Sales: Negotiate Time Windows];
          Agents --> OpsAgent[Ops: Optimize Daily Route];
          Agents --> CSAgent[Customer Success: Send ETA SMS];
      end
  ```

  ### Mobile UX Flow (375px First)
  1. **The Route Dashboard:** Carlos opens the OHC app. His "Today" view isn't just a list of times; it's a map showing a dynamically optimized route connecting his 4 jobs, along with expected driving times.
  2. **AI Booking Negotiation:** A new lead messages Carlos asking for a plumbing repair. The Sales AI agent intercepts, checks Carlos's route, and replies: "Hi! Carlos is in your neighborhood tomorrow afternoon. Does 2:00 PM to 4:00 PM work for you?"
  3. **1-Tap Approval:** Carlos sees a "Pending Route Addition" card. He taps "Approve". The Ops Agent shifts his 4:30 PM job slightly and sends an automated update to that customer.
  4. **En Route:** When Carlos finishes a job, he swipes right on the job card. The CS Agent automatically sends an SMS to the next client: "Carlos is on his way and will arrive in approx 15 mins!"

  ### AI Agent Integration Points
  - **Operations Agent:** Constantly monitors the calendar and geographic distribution of jobs to suggest the most efficient driving route.
  - **Sales Agent:** Handles the back-and-forth messaging with new leads, only proposing time slots that make geographical sense based on existing bookings.
  - **Customer Success Agent:** Proactively manages client expectations by sending automated ETA updates or delay warnings.

  ### Key Design Decisions
  - **Geo-Aware Calendar:** The underlying data model for bookings must include geocoordinates and computed travel times as first-class citizens, not just start/end times.
  - **Invisible Negotiation:** The AI handles the "when" based on the "where", removing the manual cognitive load from the service provider.
  - **Mobile-First Map Rendering:** The daily view must seamlessly integrate a native or lightweight map view tailored for a 375px screen, avoiding bloated map SDKs where possible.

  ## Implementation Prompt
  Implement the AI-Driven Dynamic Appointment Routing & Field Dispatch Engine.
  - **User-Facing Outcome:** Service providers (like Carlos) open the app to see a geographically optimized daily route. The AI automatically proposes geographically logical time slots to new leads and handles automated ETA notifications.
  - **CUJ (Critical User Journey):**
    1. A new service request with an address is received.
    2. The system computes travel times against the existing daily schedule.
    3. The Sales AI proposes an optimal time slot to the customer.
    4. Upon customer acceptance, the job is added to the route, and the daily schedule is visually updated.
    5. User completes previous job, triggering automated ETA SMS to the next client.
  - **Acceptance Criteria:**
    - The booking engine must reject or deprioritize time slots that result in overlapping travel times.
    - AI agents successfully parse location data and draft contextual scheduling messages.
    - The UI presents the daily schedule geographically on a 375px screen without horizontal scrolling.
    - Data schemas successfully link appointments, geographical coordinates, and dynamic travel buffers.
    - Zero technical jargon visible to the user.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
