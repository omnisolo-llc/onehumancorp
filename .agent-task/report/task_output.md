issue_title: "Architecture: Autonomous Field Service Scheduling & Routing Engine"
issue_description: |
  # Title: Autonomous Field Service Scheduling & Routing Engine

  ## Problem Statement
  Small business owners like Carlos (handyman) and Leo (music tutor) operate outside traditional brick-and-mortar stores. For them, static calendar slots fail because they do not account for physical travel time between client locations, localized traffic patterns, or proximity constraints. Consequently, they experience double bookings, lateness, inefficient routing, and exhausted working hours. They require an intelligent, location-aware scheduling and routing system that autonomously optimizes daily routes, auto-pads travel times, and dynamically clusters bookings without requiring a degree in logistics.

  ## Research Report
  *   **Current Architecture Limits:** OHC’s current capacity and booking ledgers focus on static time slots (e.g., a haircut or digital meeting). There is no native geospatial awareness, routing optimization layer, or dynamic travel padding logic embedded in the availability mesh.
  *   **Competitor Analysis:**
      *   *Jobber / Housecall Pro:* Feature-rich for field services and handle routing well, but they are far too complex, expensive, and require dedicated dispatchers. They fail the "grandmother test" for solo operators.
      *   *Wix / Squarespace / Shopify:* Rely on basic static calendar plugins (like Acuity or Calendly) which completely lack geographic intelligence or travel time routing.
  *   **Discovery:** OHC requires a multi-tenant, geospatial-aware scheduling engine. The engine must compute travel times between proposed job sites dynamically and push updated availability constraints to the edge caching layer instantly, enabling AI agents to accurately negotiate booking times based on the merchant's physical location trajectory throughout the day.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      CUSTOMER-AI-AGENT ||--o{ BOOKING-NEGOTIATOR : "Requests Slot"
      BOOKING-NEGOTIATOR ||--o{ GEOSPATIAL-ROUTER : "Checks Travel Feasibility"
      GEOSPATIAL-ROUTER ||--o{ ROUTE-CACHE-KV : "Reads Matrix"
      BOOKING-NEGOTIATOR }|--|| CAPACITY-LEDGER : "Reserves Time & Travel Slot"
      CAPACITY-LEDGER ||--o{ MULTI-TENANT-DB : "Strict Tenant Isolation"
      AI-OPERATIONS-DEPT ||--o{ CAPACITY-LEDGER : "Monitors Route Progress"
      AI-OPERATIONS-DEPT ||--o{ CUSTOMER-SMS : "Dispatches ETAs"
  ```

  ### UI Wireframes & Mobile UX Flow (375px)
  *   **Merchant View (OHC Mobile App - Day View - 375px):**
      *   **Interactive Daily Map Card:** A primary dashboard card adopting macOS Translucent Glass styling. It shows the daily route on a simplified map.
      *   **Timeline with Travel Padding:** Below the map, a vertical UniFi modular timeline displays jobs as solid blocks and automatically calculated travel time as translucent, striped blocks.
      *   **One-Tap Dispatch:** An "I'm on my way" prominent button that triggers the AI Operations department to handle the rest.
  *   **Customer Booking View (Mobile Safari):**
      *   When an AI agent sends a booking link, the calendar UI only exposes slots that fit into the merchant's optimized daily route, masking all the geospatial complexity behind a clean date-picker.

  ### AI Department Coordination
  *   **Operations Department:** Monitors the merchant's location (if opted-in) or explicit "dispatched" signals. It dynamically recalibrates subsequent travel blocks based on real-time traffic or delays and automatically sends branded ETA SMS updates to upcoming clients, maintaining memory of the specific job details to answer customer questions like, "Is he bringing the specific paint we talked about?"

  ### Technical Integrity & Zero Trust
  *   **Multi-Tenancy:** Geospatial data and routing matrices are strictly isolated per tenant using SPIFFE/SPIRE identity assertions on all routing service boundaries.
  *   **Mobile-First Performance:** Route calculation must occur server-side with optimized edge-caching to guarantee sub-100ms availability responses during booking negotiations, preventing timeout failures on low-end Android devices.

  ## Implementation Prompt
  **User Journey & Outcomes:**
  *   Carlos (Handyman) opens his app to see his daily schedule automatically organized to minimize driving.
  *   A new customer asks Carlos's AI agent for a quote and time slot. The AI agent only offers times that fit seamlessly into Carlos's existing route, automatically padding 30 minutes for travel based on the distance from the previous job.
  *   When Carlos taps "Heading to next job," the customer receives an automatic, branded SMS with an accurate ETA, handled entirely by the AI Operations Department.

  **Acceptance Criteria:**
  1.  **Geospatial Availability:** The booking engine must reject or hide time slots that violate the calculated travel time from the preceding/succeeding appointments.
  2.  **Dynamic Padding:** Travel time must be injected into the ledger as reserved, non-bookable capacity.
  3.  **UI Verification:** The daily timeline must visually distinguish between "Job Time" and "Travel Time" on a 375px viewport using the designated design system tokens.
  4.  **No Manual Overrides Needed:** The system must function fully autonomously; the merchant should never have to manually calculate or input drive times.

  ## Priority & Scope
  *   **Priority:** P1
  *   **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []