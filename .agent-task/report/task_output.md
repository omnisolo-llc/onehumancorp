issue_title: "[Architecture] Invisible Field Service & Local Delivery Routing Engine"
issue_description: |
  # [architecture] Invisible Field Service & Local Delivery Routing Engine

  ## Problem Statement
  Small business owners like Carlos (the handyman fielding jobs across town) and Maya (the baker delivering custom wedding cakes) struggle with logistics. Existing solutions like ServiceTitan or Jobber are extremely complex, expensive, and require dedicated dispatchers. Our users rely on their phones and cannot manage manual route planning, ETA notifications, or dynamic re-routing when a job runs late. They need an invisible routing engine that automatically sequences their day, notifies customers of ETAs, and adjusts in real-time, all managed from a simple mobile dashboard without any configuration.

  ## Research Report
  *   **ServiceTitan / Jobber:** Highly capable but designed for multi-truck fleets with dispatchers. The UI is desktop-first and dense. Overwhelming for a solo operator like Carlos.
  *   **Shopify Local Delivery:** Very basic. It offers list-based local delivery but lacks dynamic field service time-slot management, real-time routing, or automated SMS ETAs for service businesses.
  *   **Wix / Squarespace:** No native real-time routing or field service logistics. Rely entirely on disjointed third-party apps.
  *   **OneHumanCorp (OHC) Differentiation - "Invisible Dispatch":** Instead of requiring Carlos to plan his route, the **Operations Agent** automatically calculates the most efficient route based on calendar appointments and traffic, while the **CS Agent** proactively texts customers (e.g., "Carlos is 15 mins away"). This completely eliminates the dispatcher role, turning a 375px mobile screen into an enterprise-grade logistics hub.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      BOOKING_EVENT ||--o{ OPERATIONS_AGENT : "Triggers"
      OPERATIONS_AGENT }|--|| ROUTING_ENGINE : "Calculates"

      ROUTING_ENGINE {
          string tenant_id "Multi-tenant isolation"
          string spiffe_identity "Zero Trust routing"
          json waypoints "Encrypted address data"
      }

      ROUTING_ENGINE ||--o{ CS_AGENT : "Dispatches ETA"
      CS_AGENT ||--o{ CUSTOMER_SMS : "Sends"
      ROUTING_ENGINE ||--o{ MOBILE_UI : "Syncs Route"
  ```

  ### UI Wireframes & 375px Baseline
  **Core Layout: macOS-style Translucent Glass + Ubiquiti UniFi Modular Dashboard Cards**
  *   **Global Viewport:** 375px width (Mobile First). No horizontal scrolling.
  *   **App Bar:** Blurred glass top nav with the business logo and an "On Duty / Off Duty" toggle.
  *   **Map & Timeline View:**
      *   The top 40% of the screen displays a sleek, minimal map with the day's route line.
      *   The bottom 60% is a vertically scrolling timeline of cards representing jobs/deliveries.
      *   Each card has a frosted glass background (`rgba(255, 255, 255, 0.05)` with `backdrop-filter: blur(10px)`).
      *   **Actions:** 1-tap "Start Job", "Complete", or "Running Late" (which auto-notifies the next customer).
  *   **Zero-Config Magic:** No setup screens for routing. The engine implicitly reads the unified calendar and calculates it daily.

  ### Mobile UX Flow
  1. **Morning Briefing:** Carlos opens the OHC app. The AI presents his optimized route for the day across 4 jobs.
  2. **En Route:** Carlos taps "Start Job 1". The Operations agent pings the CS agent to SMS the customer: "Carlos is on the way. ETA 9:15 AM."
  3. **Delay Handling:** A job runs 30 minutes over. Carlos taps "Running Late". The Operations agent recalculates the route, and the CS agent texts subsequent customers about the updated ETA, handling replies if they need to reschedule.
  4. **Advanced Settings (Hidden):** Vehicle capacity, home base address, and buffer times are hidden in an "Advanced Settings" menu.

  ### AI Agent Integration Points
  *   **Operations Department:** Continuously monitors calendar events, physical addresses, and traffic APIs to maintain the optimal route state in the background.
  *   **Customer Service (CS) Department:** Triggered by Operations state changes to manage outbound ETA notifications and handle inbound replies (e.g., "I'm not home yet, can you come in 20 mins?").

  ### Key Design Decisions (Why, not How)
  *   **Implicit Logistics:** Users should not need to explicitly build routes. The system should derive the route from the unified calendar and order ledger.
  *   **Proactive CS:** Automating "I'm on my way" texts reduces customer anxiety and inbound calls to the business owner by 80%.
  *   **Zero-Trust Isolation:** Physical addresses are highly sensitive PII. The `ROUTING_ENGINE` must strictly enforce multi-tenant isolation. A tenant must never be able to infer the routing data of another tenant.

  ## Implementation Prompt
  **To the Implementer Swarm:**
  Your goal is to build the underlying architecture and UI for the "Invisible Field Service & Local Delivery Routing Engine" so a user like Carlos can manage his daily jobs efficiently from a mobile device without manual dispatching.

  **Customer User Journey (CUJ):**
  1. Carlos has 4 bookings for the day with different addresses.
  2. The system automatically displays an optimized timeline and map view in his mobile app.
  3. Carlos taps "Start" on the first job. The system automatically sends an ETA SMS to the customer.
  4. If Carlos indicates he is running late via a quick action, the system auto-notifies subsequent customers of their new ETAs.

  **Acceptance Criteria:**
  *   **Mobile Parity:** The UI must be implemented perfectly for a 375px viewport using the described Translucent Glass aesthetics and card layouts.
  *   **Automated Sequencing:** Given a list of daily bookings with addresses, the backend must return a logically sequenced route.
  *   **Agent Triggering:** State changes in the mobile UI (e.g., "Start", "Running Late") must trigger the backend AI agents to draft or send SMS notifications.
  *   **Isolation Guarantee:** Implement strict multi-tenant boundary checks so routing/address data is only accessible to the authenticated `organization_id`.
  *   **Simplicity:** Do not expose complex logistics settings (TSP algorithms, geofences) in the primary UI. Hide any configuration in an "Advanced" toggle.

  ### Performance & Offline Targets
  *   **Latency:** Route calculation must complete in under 500ms at the P95 level to ensure the UI feels instant.
  *   **Offline-capability:** The day's active route map and job manifest must be aggressively cached on the device so Carlos can view job details, complete tasks, or mark himself late even with zero cellular connectivity. Changes sync when reconnected.
  *   **Payload Target:** Delta updates for ETA recalculations should be under 2KB per push notification event to minimize battery and data usage in the field.

  ## Estimated Scope
  Medium
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
