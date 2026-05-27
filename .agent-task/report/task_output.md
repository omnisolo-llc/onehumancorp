issue_title: "Implement Autonomous Local Delivery & Courier Dispatch Mesh"
issue_description: |
  **Problem Statement:**
  Maya (baker) and Fatima (food cart) need an autonomous way for local delivery to "happen" automatically. Currently, they have to manually coordinate drops or manage unreliable local couriers. They need a system that calculates delivery fees based on distance, schedules pickups when items are ready, and provides tracking without any manual effort.

  **Research Report:**
  Currently, the SMB market relies heavily on fragmented solutions or expensive marketplace aggregators:
  - **Shopify Local Delivery:** Offers route optimization for self-delivery, but requires merchants to download a separate local delivery app and manually assign routes. It does not seamlessly dispatch third-party couriers out-of-the-box without complex third-party app integrations.
  - **Square On-Demand Delivery:** Integrates directly with DoorDash and Uber Eats for courier dispatch from Square Online. This is a strong feature, but requires significant upfront configuration.
  - **Marketplace Delivery (DoorDash/UberEats native):** Takes massive commissions (15-30%) and owns the customer relationship.

  **The OHC Opportunity:**
  By integrating white-label delivery APIs (Uber Direct, DoorDash Drive) directly into our core ledger and autonomous operations engine, we can offer "Zero-Touch Local Delivery". When a local order is placed, the OHC Finance Agent calculates the exact delivery fee. When the OHC Operations Agent sees the item is "Ready", it autonomously dispatches the courier, negotiates any live issues via the AI CS Agent, and updates the customer.

  **Design Doc:**
  **1. Data Model (Entity-Relationship Diagram)**
  ```mermaid
  erDiagram
      TENANT ||--o{ DELIVERY_ZONE : defines
      DELIVERY_ZONE ||--o{ COURIER_PROVIDER_CONFIG : utilizes
      ORDER ||--|| DELIVERY_DISPATCH : triggers
      DELIVERY_DISPATCH }o--|| COURIER_PROVIDER_CONFIG : routes_through
      DELIVERY_DISPATCH ||--o{ DISPATCH_EVENT : logs
      DELIVERY_DISPATCH {
          uuid id
          uuid order_id
          uuid tenant_id
          string status
          decimal quoted_fee
          decimal actual_fee
          string tracking_url
          string courier_name
          timestamp pickup_time
          timestamp dropoff_time
      }
      COURIER_PROVIDER_CONFIG {
          uuid id
          string provider_type
          jsonb api_credentials
          decimal markup_percentage
          boolean is_active
      }
  ```

  **2. Architecture Diagram (Sequence Flow)**
  ```mermaid
  sequenceDiagram
      actor Customer
      participant CheckoutEngine
      participant FinanceAgent
      participant OpsAgent
      participant DispatchMesh
      participant UberDirectAPI

      Customer->>CheckoutEngine: Enter delivery address (Local)
      CheckoutEngine->>FinanceAgent: Request live delivery quote
      FinanceAgent->>DispatchMesh: Query available couriers & rates
      DispatchMesh->>UberDirectAPI: Fetch delivery estimate
      UberDirectAPI-->>DispatchMesh: Rate: $6.50
      DispatchMesh-->>FinanceAgent: Rate: $6.50 + $0.50 markup
      FinanceAgent-->>CheckoutEngine: Display $7.00 Delivery Fee
      Customer->>CheckoutEngine: Complete Checkout

      Note over OpsAgent: Time passes, product is prepared
      actor Merchant
      Merchant->>OpsAgent: Mark order "Ready for Pickup"
      OpsAgent->>DispatchMesh: Initiate Dispatch
      DispatchMesh->>UberDirectAPI: Create Delivery Task
      UberDirectAPI-->>DispatchMesh: Tracking URL & Driver Info
      DispatchMesh->>OpsAgent: Broadcast Tracking Info
      OpsAgent->>Customer: SMS: "Your driver is on the way! [Link]"
  ```

  **3. Mobile-First UX Flow (375px)**
  - **Merchant (Maya/Fatima) Perspective:**
    - **Onboarding (Automated):** AI asks if they want auto-delivery within 5 miles. One tap "Yes, turn on Auto-Delivery".
    - **Order Dashboard:** Clean badge "Local Delivery". Tap prominent "Mark Ready & Call Driver" button.
    - **Active Dispatch View:** Order card expands with real-time mini-map and driver details.
  - **Customer Perspective:**
    - **Checkout:** "Local Delivery (Arrives in ~45 mins) - $7.00".
    - **Tracking Experience:** SMS link to OHC-hosted tracking page (translucent glass styling) showing live map.

  **4. AI Agent Integration Points**
  - **Finance Agent:** Dynamically queries Dispatch Mesh during checkout for live quotes.
  - **Operations Agent:** Monitors order status. Calls Dispatch Mesh when marked "Ready".
  - **Customer Support (CS) Agent:** Intercepts courier issues (e.g., driver lost) via webhook, messages customer, updates instructions invisibly to merchant.

  **Implementation Prompt:**
  To the Implementer Agent: Implement the backend infrastructure and mobile-first UI for Autonomous Local Delivery & Courier Dispatch Mesh.
  1. Implement the multi-tenant data structures for DELIVERY_DISPATCH and COURIER_PROVIDER_CONFIG with strict row-level security.
  2. Create the core service interface for the Dispatch Mesh.
  3. Implement a "Mock" courier provider simulating quote/dispatch webhooks.
  4. Build the "Active Dispatch View" card for the 375px mobile merchant dashboard.

  **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
