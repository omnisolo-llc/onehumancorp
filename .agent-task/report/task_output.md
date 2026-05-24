issue_title: "[architecture] Autonomous Local Delivery Dispatch Engine"
issue_description: |
  # Problem Statement
  Small businesses like Maya's bakery or Fatima's food cart struggle with local delivery logistics. Relying on third-party marketplace apps (UberEats, DoorDash) incurs massive 30% commission fees, destroying margins. Trying to manage in-house delivery requires manual route planning, managing driver schedules, and manual SMS updates to customers, which is impossible while baking or cooking. They need an "invisible" dispatch engine that automatically routes orders to the most efficient delivery method (in-house driver or flat-fee white-label delivery networks like Uber Direct / DoorDash Drive) without any manual routing or phone calls.

  # Research Report
  **Market Analysis:**
  - **Shopify:** Requires installing third-party apps like Zapiet or Routific, forcing the merchant to pay additional monthly fees and learn new, complex routing software.
  - **Square/Wix:** Offer basic local delivery zones, but leave the actual dispatching and routing entirely up to the merchant.
  - **Delivery Marketplaces (UberEats, DoorDash):** Charge 15-30% on the subtotal. However, their white-label APIs (Uber Direct, DoorDash Drive) charge a flat fee (e.g., $7 per delivery) regardless of order size, preserving merchant margins.

  **The OHC Opportunity:**
  OHC can abstract delivery entirely. When an order is placed, the OHC Operations Agent automatically calculates the most cost-effective way to deliver it, batches it with other local orders, and either dispatches an in-house driver via the OHC Driver Companion app or pings a white-label delivery API—all while the OHC Support Agent sends real-time tracking SMS to the customer.

  # Design Doc

  ## Architecture Diagram
  ```mermaid
  erDiagram
      ORDER ||--o{ DISPATCH_JOB : triggers
      DISPATCH_JOB }|--|| DELIVERY_NETWORK : routed_to
      DISPATCH_JOB }|--|| IN_HOUSE_DRIVER : routed_to
      DISPATCH_JOB {
          string status
          float cost
          datetime estimated_arrival
      }
      TENANT_CONFIG ||--o{ DISPATCH_JOB : dictates_rules
  ```

  ```mermaid
  sequenceDiagram
      participant Customer
      participant OrderLedger
      participant OpsAgent as AI Operations Agent
      participant RouteOptimizer
      participant DeliveryNetwork as Uber Direct / Driver App
      participant CSAgent as AI CS Agent

      Customer->>OrderLedger: Places Local Delivery Order
      OrderLedger->>OpsAgent: Order Created Event
      OpsAgent->>RouteOptimizer: Request Batching & Route
      RouteOptimizer-->>OpsAgent: Optimal Route & Cost
      OpsAgent->>DeliveryNetwork: Dispatch Delivery Request
      DeliveryNetwork-->>OpsAgent: Driver Assigned & ETA
      OpsAgent->>CSAgent: Forward ETA
      CSAgent->>Customer: SMS: "Your order is on the way! ETA 15 mins."
  ```

  ## Mobile-First UX Flow (375px)
  - **Home Dashboard Card (Translucent Glass):** "3 Active Deliveries" with a mini live map showing driver dots. Tap to expand.
  - **Delivery Hub:**
    - Simple toggle: "Accept Local Deliveries" (On/Off).
    - Slider: "Max Delivery Radius" (e.g., 5 miles).
    - Cost Rules: "Charge Customer $5" or "Free over $50".
    - Driver Setup: "Use OHC Fleet Network (Flat $7/trip)" OR "Invite In-House Drivers".
  - **Interaction:** No routing tables, no manual assignment buttons. The merchant just sees a live status feed: "Order #102 picked up by Dave. Delivering in 8 mins."

  ## AI Agent Integration Points
  - **Operations Agent:** Monitors the order queue, batches geolocated orders to minimize trips, and negotiates/dispatches with white-label API networks.
  - **Customer Support (CS) Agent:** Monitors the driver's GPS coordinates and proactively texts the customer if there is a delay (e.g., "Traffic is heavy, your cake will arrive 10 mins later than expected").

  # Implementation Prompt
  **Objective:** Implement the backend architecture, data model, and AI agent coordination for the Autonomous Local Delivery Dispatch Engine.

  **User Journey (CUJ):**
  1. A merchant enables Local Delivery in 1 tap, setting a 5-mile radius and choosing the white-label fleet.
  2. A customer within 5 miles orders and pays the delivery fee.
  3. The Ops Agent automatically batches this with another order nearby, books a single driver via the Uber Direct API, and the CS Agent texts both customers tracking links.
  4. The merchant does nothing except hand the bags to the driver who arrives at the counter.

  **Acceptance Criteria:**
  - Create the core `DispatchJob` and `RouteBatch` entities with strict multi-tenant isolation.
  - Implement the Ops Agent capability to trigger on new orders and evaluate routing.
  - Expose a simple GraphQL/REST mobile API for the Delivery Hub toggles.
  - (Do not prescribe specific database column types or exact function signatures—focus on the domain services and agent transitions).

  **Priority:** P1
  **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
