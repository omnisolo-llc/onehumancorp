issue_title: "Architecture: Autonomous Agentic Field Service Routing & Dispatch Engine"
issue_description: |
  ### Problem Statement
  Carlos (field service owner) needs a way to manage on-site jobs, travel routes, and dispatch logistics directly from his Android phone without needing complex routing software. Currently, OHC handles bookings and quoting, but it lacks a dedicated spatial-aware dispatch capability that understands drive times, location boundaries, and on-site job statuses (en route, on-site, completed) integrated seamlessly with the KAIROS AI OS.

  ### Research Report
  *   **Target Persona:** Carlos (handyman, 42). Runs everything from an Android phone. Needs route notes, service requests, estimates, deposits, and on-site navigation.
  *   **Competitive Analysis:**
      *   **Housecall Pro / Jobber:** Feature-rich but heavily menu-driven. They require significant setup for service zones and route optimization.
      *   **Square / Wix:** Weak on field service routing. Mostly focused on basic calendar appointments without spatial context.
  *   **Gap in OHC:** We have unified booking and quoting, but lack the geographical/routing data model and the agent coordination to handle "I'm running late" or "Optimize my route for today". The system needs a background DispatchAgent that recalculates travel buffers between jobs dynamically.

  ### Design Doc
  **Architecture Diagram:**
  ```mermaid
  graph TD
      subgraph KAIROS Orchestrator
          TL[(Shared Task List)]
          V[(pgvector Memories)]
          Dispatcher[Dispatch & Routing Agent]
      end

      subgraph Field Service Domain
          RouteCache[(Route/Geospatial Cache)]
          JobState[Job Status State Machine]
      end

      subgraph Teammate Mesh
          Mesh[Redis/Centrifugo]
      end

      subgraph Mobile Interface
          App[Android App 375px]
          Map[Map/Glassmorphism Cards]
      end

      App <-->|Pub/Sub Updates| Mesh
      Mesh <--> TL
      TL --> Dispatcher
      Dispatcher --> RouteCache
      Dispatcher --> JobState
      JobState -->|Status Change| Mesh

      classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
      class TL,V,Dispatcher,RouteCache,JobState,Mesh,App,Map premium;
  ```

  **Mobile UX Flow (375px First):**
  1.  **Morning Briefing:** Carlos opens the app and sees a Translucent Glass card: "3 Jobs Today. 1st job in 45 mins (15 min drive)."
  2.  **En-Route Tap:** One tap on "Navigate". Background agent texts the customer: "Carlos is on the way, ETA 10:15 AM."
  3.  **On-Site & Quote:** Arrives, assesses the work. Generates an AI quote in 1 tap, presents it to the customer.
  4.  **Completion & Pay:** Taps "Job Done". Prompted with Tap-to-Pay on his Android phone. Agent logs memory for future follow-up.

  **AI Agent Integration Points:**
  *   **Dispatch Agent:** Listens to new bookings on the Shared Task List. Automatically assigns travel buffer times based on geospatial distance.
  *   **Customer Comms Agent:** Listens to `JobStatus` changes (e.g., `EN_ROUTE`) via the Teammate Mesh and automatically sends SMS updates to the customer.

  **Key Design Decisions:**
  *   Use PostGIS/pgrouting (or similar geospatial awareness) within the Tenant database, maintaining strict RLS.
  *   Decouple travel time calculation from the booking engine, processing it asynchronously via the KAIROS queue.
  *   UI must use the OHC Premium Token library (glassmorphism, 44x44px touch targets).

  ### Implementation Prompt
  Implement the Field Service Routing & Dispatch Engine in the backend and the "Today's Route" view in the frontend.
  *   **Outcome:** The owner (Carlos) should be able to view their daily jobs sorted by route efficiency, tap to change job status (En Route, On Site, Done), and have the system automatically notify the customer.
  *   **CUJ:** From the mobile UI, Carlos selects a booked job, taps "Start Travel", which updates the status and triggers a customer notification.
  *   **Acceptance Criteria:**
      *   A new Domain model/DB schema for `ServiceRoute` and `JobLocation` with strict tenant isolation.
      *   An API endpoint for updating on-site job status.
      *   A Teammate Mesh event broadcast on status change.
      *   Playwright E2E test verifying the complete mobile-first flow (starting from login to changing a job status to 'Done') with zero mock data.
      *   Mobile UI layout optimized for 375px viewport adhering to the Visual Excellence Mandate.
  *   **Priority:** P1
  *   **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
