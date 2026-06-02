issue_title: "Implement Zero-Config Autonomous Local Delivery & Dispatch Engine"
issue_description: |
  # [Architecture] Zero-Config Autonomous Local Delivery & Dispatch Engine

  ## Problem Statement
  Maya (the home baker) sells custom cakes and Fatima (the food cart operator) wants to take pre-orders for pickup or local delivery. Currently, small businesses struggle with managing local deliveries. They have to manually calculate delivery zones, estimate times, dispatch drivers (often themselves or a partner), notify customers, and handle "where is my order" questions. They need a system where they can simply define a delivery radius or zip codes, set a fee, and let the platform automatically handle capacity, route optimization, customer ETA notifications, and driver dispatch, all without complex configuration.

  ## Research Report
  **Market Gap & Competitor Analysis:**
  - **Shopify:** Has local delivery options, but requires third-party apps for route optimization and real-time driver tracking (e.g., Zapiet, Routific) which are expensive ($50-$100/mo) and hard to configure for non-technical users.
  - **Wix/Squarespace:** Very basic local delivery settings (flat rate by zip code). No driver dispatch, no route optimization, no real-time ETA for customers.
  - **UberEats/DoorDash:** Eat 30% of the merchant's margin. Small businesses want to run their own delivery or use local couriers without giving up margin.
  - **Square:** Good for in-store pickup, but weak on active local delivery dispatch and tracking without expensive add-ons.

  **Opportunity:** By building a native, multi-tenant local delivery engine with built-in AI routing (via Operations Department), OneHumanCorp can offer a "toggle-on" local delivery experience. The AI agent manages the delivery schedule, optimizes the route for the owner, and automatically texts the customer a live tracking link, making a one-person bakery feel like an enterprise logistics operation.

  ## Design Doc

  ### Architecture Diagram

  ```mermaid
  erDiagram
      TENANT ||--o{ DELIVERY_ZONE : defines
      DELIVERY_ZONE {
          uuid id
          uuid tenant_id
          geometry polygon
          decimal flat_fee
          int min_order_value
      }
      TENANT ||--o{ ORDER : receives
      ORDER ||--o{ DELIVERY_TASK : creates
      DELIVERY_TASK {
          uuid id
          uuid order_id
          uuid driver_id
          enum status
          timestamp estimated_arrival
          geometry delivery_location
      }
      DELIVERY_TASK ||--o{ ROUTE_PLAN : belongs_to
      ROUTE_PLAN {
          uuid id
          uuid tenant_id
          date delivery_date
          json waypoint_sequence
      }
      DRIVER ||--o{ DELIVERY_TASK : assigned_to
  ```

  ### UI Wireframes & Screen Flow (375px first)
  **1. Setup Flow:**
  - "Enable Local Delivery" toggle switch.
  - Map view (Glassmorphism card): "Tap to draw your delivery area" or "Enter zip codes".
  - Sliders for "Max deliveries per day" and "Delivery Fee".

  **2. Driver/Owner Dispatch App (Mobile View):**
  - **Today's Route:** A clean list of stops organized by optimal route.
  - **Action Buttons:** Large touch targets (44x44px min): "Start Route", "Mark Delivered", "Call Customer".
  - **Live Map:** Mini map showing the next stop.

  **3. Customer Tracking Page (Web/PWA):**
  - Translucent Glass modal showing "Maya's Cakes is on the way!"
  - Live ETA (e.g., "Arriving in 15 mins").
  - Simple visual progress bar: Prep -> Out for Delivery -> Delivered.

  ### Mobile UX Flow
  - The owner opens the OHC app, navigates to "Operations", and sees a daily delivery itinerary.
  - They tap "Start Delivery Route". The app uses the device's location (background location tracking enabled temporarily) to update the ETA for each customer.
  - When they arrive, they tap "Mark Delivered" (which supports uploading a photo of the package at the door).
  - If an issue arises, they can tap "Message Customer," which opens an AI-drafted text (e.g., "Hi, I'm outside but don't see the apartment number").

  ### AI Agent Integration Points
  - **Operations Department:** Automatically groups orders by location and time, generating an optimized route plan for the day using a TSP (Traveling Salesperson Problem) heuristic.
  - **Customer Success Department:** Listens to webhook events from the `DELIVERY_TASK` state machine and automatically sends SMS/WhatsApp updates ("Your cake is next on the route!"). Handles "where is my order?" inquiries by querying the active `ROUTE_PLAN`.
  - **Finance & Payments:** Automatically calculates and captures the delivery fee at checkout based on the customer's address and the tenant's `DELIVERY_ZONE`.

  ### Key Design Decisions
  - **PostGIS for Spatial Data:** Use PostgreSQL's PostGIS extension to store `DELIVERY_ZONE` polygons and perform fast point-in-polygon checks during checkout to determine if a customer is eligible for local delivery.
  - **Background Location Sync:** The driver app will use a low-frequency location sync (e.g., every 30 seconds) via gRPC streaming or WebSockets to update the `DELIVERY_TASK` location, preserving battery life on the owner's phone.
  - **AI-Managed Capacity:** The Operations agent will automatically turn off local delivery for a specific day if the calculated route time exceeds the owner's set working hours, preventing overbooking.

  ## Implementation Prompt
  **Task for Implementer:**
  Implement the core Local Delivery & Dispatch engine.
  1. Add PostGIS schemas to the database to support `DELIVERY_ZONE` (polygons) and `DELIVERY_TASK` (points). Ensure strict tenant isolation.
  2. Create the backend Go service (`DeliveryService`) to manage zones, create tasks, and generate simple route sequences.
  3. Expose REST/gRPC endpoints for the Flutter app to:
     - Configure delivery zones (drawing polygons or zip codes).
     - Fetch the daily delivery itinerary.
     - Update the status of a delivery task (e.g., IN_TRANSIT, DELIVERED).
  4. Implement a background job (AI Operations Agent) that recalculates the optimal route when new orders are added to a day's queue.
  5. Create Playwright E2E tests for the admin setup flow (enabling local delivery and setting a zone) and the checkout flow (customer entering an address and seeing the delivery option).
  **Acceptance Criteria:** A tenant can draw a delivery zone, a customer within that zone can select "Local Delivery" at checkout, and the tenant sees the order in an optimized daily route list on their mobile app.

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []