issue_title: "[architecture] Autonomous Hyper-Local Delivery & Route Optimization Engine"
issue_description: |
  # Multi-Tenant Autonomous Hyper-Local Delivery & Route Optimization Engine

  ## Problem Statement

  Maya, our 28-year-old baker, delivers custom cakes across the city. Currently, she manually types addresses into Google Maps, tries to guess the most efficient order of stops, and texts customers one by one with rough ETAs. When she hires a part-time driver for busy weekends, she has to print out lists of addresses and has no idea when deliveries are completed until the driver returns. Fatima, running her halal food cart, faces similar issues when she wants to start offering local delivery within a 5-mile radius.

  Small business owners need an invisible, zero-config way to manage local deliveries, instantly dispatch drivers (even temporary ones), automatically optimize routes, and provide live, Uber-style tracking to customers—all without paying $150+/month for enterprise software like Onfleet.

  ## Research Report

  **Market Analysis:**
  - **Shopify & Wix**: Offer basic "local delivery" options at checkout but lack native, advanced route optimization and driver dispatching. They require merchants to install third-party apps.
  - **Third-Party Solutions (Onfleet, Routific, Circuit)**: Highly capable but built for larger fleets. They are expensive (often $50-$150+ per month), require complex setup, and force drivers to download native apps from the app store and create accounts.
  - **The Gap**: Micro-businesses and solopreneurs need built-in, pay-as-you-go (or free tier) route optimization that "just works" out of the box. Drivers (often friends, family, or gig workers) need frictionless access via a simple web link, not a heavy app. Customers expect modern, real-time tracking via SMS or WhatsApp.

  **Strategic Opportunity for OneHumanCorp (OHC):**
  By baking a multi-tenant hyper-local delivery mesh directly into the platform, OHC can own the entire post-purchase fulfillment experience. We eliminate the need for third-party logistics software.

  ## Design Doc

  ### Architecture Diagram

  ```mermaid
  erDiagram
      TENANT ||--o{ DELIVERY_BATCH : creates
      DELIVERY_BATCH ||--o{ DELIVERY_STOP : contains
      ORDER ||--|| DELIVERY_STOP : fullfils
      DELIVERY_BATCH ||--o| DRIVER_SESSION : assigned_to

      TENANT {
          string id
          string name
          json delivery_settings
      }
      DELIVERY_BATCH {
          string id
          string tenant_id
          string status
          datetime scheduled_for
          json optimized_route_data
      }
      DELIVERY_STOP {
          string id
          string order_id
          int sequence_index
          string status
          json proof_of_delivery
      }
      DRIVER_SESSION {
          string id
          string phone_number
          string magic_link_token
          datetime expires_at
      }
  ```

  ```mermaid
  sequenceDiagram
      autonumber
      actor Merchant as Maya (Merchant)
      participant OHC_UI as OHC Mobile App
      participant AI_Ops as Operations Agent
      participant RouteEngine as Route Optimizer
      actor Driver as Driver
      actor Customer as Customer

      Merchant->>OHC_UI: Select orders & tap "Dispatch Delivery"
      OHC_UI->>AI_Ops: Trigger delivery batch creation
      AI_Ops->>RouteEngine: Send addresses & time windows
      RouteEngine-->>AI_Ops: Return optimized sequence & ETAs
      AI_Ops->>Driver: Send SMS with Magic Link (PWA)
      Driver->>Driver: Open PWA, view stops, tap "Navigate"
      Driver->>Driver: Arrive, tap "Mark Delivered", capture photo
      Driver->>AI_Ops: Submit Proof of Delivery
      AI_Ops->>Customer: Send SMS "Delivered!" with photo
  ```

  ### Mobile UX Flow (375px Viewport First)

  **1. Merchant Dispatch View:**
  - **Screen:** "Local Deliveries" tab.
  - **Layout:** A clean, Unifi-style card list showing pending local orders. A floating action button (FAB) at the bottom reads "Optimize & Dispatch (X Orders)".
  - **Action:** Tapping the FAB shows a bottom sheet to enter a driver's phone number or select "I am driving".

  **2. Driver PWA View (Magic Link):**
  - **Screen:** Frictionless web app (no login).
  - **Layout:** Large, high-contrast map card at the top showing the current destination. Below the map, a massive primary button: "Open in Google Maps/Apple Maps".
  - **Action:** Once at the destination, the driver swipes right on a "Swipe to complete" slider. A translucent glass modal prompts for an optional photo (Proof of Delivery).

  **3. Customer Live Tracking View:**
  - **Screen:** Sent via SMS link.
  - **Layout:** Map showing driver location (if location sharing is enabled) or simply a sequence tracker (Preparing -> Out for Delivery -> Arrived). Minimalist, on-brand for the merchant.

  ### AI Agent Integration Points

  - **Operations Agent**: Automatically runs a background job at 8 AM daily to group the day's local deliveries into logical batches based on geography. It proactively suggests these batches to the merchant.
  - **CS (Customer Service) Agent**: If a customer replies to the delivery SMS (e.g., "Gate code is 1234, leave it on the porch"), the CS Agent intercepts this, understands the context, and instantly updates the driver's notes in the PWA.

  ### Key Design Decisions & Why
  - **Frictionless Driver PWA**: Drivers receive a magic link via SMS that expires in 24 hours. No app store downloads, no account creation. This solves the massive friction of hiring temporary drivers or having friends help out.
  - **Background Route Optimization**: Route planning is offloaded to a background queue, allowing the merchant to immediately continue using the app while the system calculates the optimal TSP (Traveling Salesperson Problem) route.
  - **Zero-Trust Multi-Tenancy**: Driver sessions are strictly cryptographically scoped to a single delivery batch for a single tenant. They cannot access the merchant's full order history or other customers' data.

  ## Implementation Prompt

  **Outcome:** Build the core backend logic, data models, and API endpoints for the Hyper-Local Delivery Mesh.
  **Core User Journey (CUJ):**
  1. A merchant selects 5 pending orders and submits them to be batched for delivery.
  2. The system asynchronously calculates an optimized sequence for the stops.
  3. The system generates a secure, time-limited magic link for a driver session.
  4. The driver accesses the link, sees the ordered stops, and updates the status of a stop to "Delivered", uploading a photo.

  **Acceptance Criteria:**
  - Create the schema/data models for Delivery Batches, Stops, and Driver Sessions ensuring strict multi-tenant isolation.
  - Implement an API endpoint to ingest an array of order IDs and return a generated Delivery Batch.
  - Integrate a basic route optimization heuristic or hook for external TSP calculation.
  - Implement the magic link generation logic (JWT or secure token) for the driver PWA.
  - Implement the endpoint for the driver to update a stop status and upload a proof-of-delivery asset.
  - All endpoints must verify multi-tenant scoping and driver token authorization.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
