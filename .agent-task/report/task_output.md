issue_title: "[Architecture] Zero-Touch Smart Service Dispatch & Route Optimization Engine"
issue_description: |
  ## Title
  [Architecture] Zero-Touch Smart Service Dispatch & Route Optimization Engine

  ## Problem Statement
  Service operators like Carlos the Handyman and other mobile businesses (e.g., mobile pet grooming, home cleaning) spend hours each week manually coordinating their schedules, estimating travel times between jobs, and communicating ETA updates to clients. When a job runs late or a cancellation occurs, the entire day’s schedule breaks down. This manual triage causes lost revenue, delayed services, and frustrated customers. The owner needs an assistant that automatically groups nearby jobs, optimizes travel routes, adjusts the calendar dynamically, and texts clients with precise arrival times—all without requiring the owner to manually manipulate a complex dispatch dashboard.

  ## Research Report
  - **Market Context:** Small field service businesses heavily rely on point solutions like Jobber, Housecall Pro, or Route4Me. However, these tools are often overly complex "dispatch boards" designed for desktop monitors and dedicated dispatchers, not a single owner/operator working from an Android phone.
  - **Competitive Analysis:**
    - **Shopify/Wix:** Built primarily for physical or digital goods. They have basic appointment booking (e.g., Wix Bookings) but lack any awareness of geographic locations, travel time buffers, or route optimization.
    - **Jobber/Housecall Pro:** Powerful dispatching but complex to set up. Requires manual intervention to optimize routes and often charges premium fees for automated routing.
    - **OHC Opportunity:** OHC can differentiate by leveraging the "Operations Agent" to handle dispatch autonomously. Instead of giving Carlos a complex map interface to drag-and-drop appointments, OHC proactively suggests the most efficient daily route and handles client communication automatically when delays occur.
  - **Data/References:** Field service operators lose an average of 15% of their billable hours to inefficient routing and manual schedule adjustments. Automated ETA notifications reduce customer no-shows and "where are you" inquiries by over 40%.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      subgraph Frontend "OHC Mobile App (375px)"
          Feed[Agent Feed]
          RouteView[Daily Route & Active Job]
          Actions[One-Tap Actions: 'Running Late', 'En Route']
      end

      subgraph Backend "OHC Core & APIs"
          Sync[Sync Engine]
          Routing[Geospatial Routing Service]
          Comms[Omnichannel Comms via Twilio/WhatsApp]
      end

      subgraph AI "Agent Departments"
          OpsAgent[Operations Agent: The Dispatcher]
          CSAgent[Customer Success Agent]
      end

      subgraph Data & Infra
          DB[(PostgreSQL + PostGIS)]
          Redis[(Redis - Distributed Locks)]
      end

      Feed --> OpsAgent
      RouteView <--> Sync
      Actions --> OpsAgent

      OpsAgent <--> Routing
      OpsAgent --> DB
      OpsAgent --> CSAgent

      CSAgent --> Comms
  ```

  ### UI Wireframes & Screen Flow (375px First)
  1. **Morning Briefing Card:** User opens the OHC app. The Agent Feed presents a card: "Your route today is optimized. 4 stops, starting at 9:00 AM. [Start Day]"
  2. **Active Job Screen:** Displays the current job, client address, and an embedded map preview. Large touch buttons (44x44px min): `Navigate`, `En Route`, `Start Job`, `Complete`.
  3. **Delay Intervention Card:** If the user taps "Running Late" (or if the phone's GPS indicates a delay), a modal appears. The Ops Agent proposes: "Notify the next 2 clients of a 30-minute delay? [Approve & Send]".

  ### Mobile UX Flow
  - **Constraints:** Zero map-based drag-and-drop. The interface relies entirely on chronological task cards.
  - **Interactivity:** One-thumb operation for status updates. Transitions are fluid (using Glassmorphism tokens) to indicate state changes without full page reloads.

  ### AI Agent Integration Points
  - **Operations Agent:** Monitors the calendar, calculates optimal routes using geospatial APIs (e.g., Google Maps/Mapbox), and inserts travel time blocks between appointments. It detects anomalies (e.g., overlapping bookings or impossible travel times) and proposes schedule corrections.
  - **Customer Success Agent:** Drafts and sends SMS/WhatsApp updates to clients based on the Operations Agent's status triggers (e.g., "Carlos is 15 minutes away").

  ### Key Design Decisions
  - **Autonomous over Manual:** We prioritize AI-driven schedule optimization over providing a complex drag-and-drop calendar UI. The owner approves changes rather than making them manually.
  - **Geospatial Awareness:** Incorporating PostGIS or similar capabilities to allow proximity-based booking (e.g., only offering Tuesday morning slots to clients in the North Zone because Carlos is already scheduled there).
  - **Offline Resilience:** The daily route and client details must be cached locally on the device to handle areas with poor cellular reception.

  ## Implementation Prompt
  **Objective:** Implement the Zero-Touch Smart Service Dispatch & Route Optimization Engine for field service operators.

  **User-Facing Outcome:** The user (e.g., Carlos) should open the app and see an automatically optimized daily schedule that accounts for travel time between locations. When a job is delayed, the user can tap "Running Late," and the system will automatically draft notifications for subsequent clients and adjust the calendar, requiring only a single tap to approve.

  **Critical User Journey (CUJ):**
  1. The user logs in and views their daily schedule, which includes 3 service appointments at different locations.
  2. The system has automatically inserted appropriate travel time blocks between the appointments based on location data.
  3. The user taps "Running Late" on the first appointment.
  4. The Operations Agent calculates the cascading delay and presents an Action Card: "Drafting delay notifications for the next 2 clients. Approve?"
  5. The user taps "Approve", and the schedule updates while the notifications are sent.

  **Acceptance Criteria:**
  - The feature must be fully functional on a 375px mobile viewport.
  - The system must dynamically calculate and insert travel times between geographically distinct bookings.
  - The delay propagation logic must accurately update subsequent appointments.
  - The UI must use the standard Agent Feed Action Card component for approvals.
  - Include end-to-end Playwright tests verifying the delay propagation and approval flow.
  - Do not prescribe specific database schema additions or API routes; design the internal data models to support this flow securely and efficiently.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
