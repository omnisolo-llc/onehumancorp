issue_title: "OHC Local-First Routing: Autonomous Local Delivery and Dispatch Mesh"
issue_description: |
  # Architecture Brief: Autonomous Local Delivery and Dispatch Mesh

  ## Problem Statement
  Small business owners (Fatima the food cart operator, Maya the baker, Carlos the handyman) who handle local deliveries, service calls, or catering often struggle with logistics. They manually text ETAs, write delivery addresses on paper, and use standard map apps to individually route stops, losing time and creating a poor customer experience. They need a zero-touch, invisible dispatch system that automatically calculates the most efficient multi-stop routes, assigns them to available staff or the owner, and proactively updates the customer with live ETAs. This eliminates the need for expensive, complex 3rd-party logistics software.

  ## Research Report
  - **Competitive Benchmark**: Shopify requires expensive third-party apps (e.g., Local Delivery by Shopify, Routeific) for multi-stop routing and live ETA tracking, confusing for non-technical users. Wix has very basic local delivery radii but lacks route optimization and driver apps. Squarespace offers minimal local delivery options.
  - **SMB Workaround**: Most solopreneurs currently copy-paste addresses into WhatsApp or Google Maps individually, text customers manually, and cannot easily re-route if a stop is delayed.
  - **The OHC Opportunity**: Integrating route optimization directly into the core platform using the Operations AI department to act as a silent dispatcher.

  ## Design Doc

  ### High-Level Architecture (Mermaid.js)
  ```mermaid
  graph TD
      Order[New Local Order/Service Booking] --> LeadTime[The Operations Agent]
      LeadTime -->|Calculates Prep Time & Window| Router[The Dispatch Router]
      Router -->|Optimizes Multi-Stop Route| DriverMesh[Driver/Staff Assignments]

      subgraph Autonomous Dispatch Mesh
          R1[Route Node 1: Pickup/Prep]
          R2[Route Node 2: Delivery A]
          R3[Route Node 3: Delivery B]
          R1 --> R2 --> R3
      end

      DriverMesh --> Autonomous Dispatch Mesh
      Autonomous Dispatch Mesh -->|Live Geolocation| CustAgent[The CS Agent]
      CustAgent -->|Proactive ETA SMS/WhatsApp| Customer[End Customer]
  ```

  ### Mobile-First UX Flow & Visual Design
  - **Zero-Trust & Security**: Geolocation data is strictly isolated per tenant using SPIFFE/SPIRE. Driver location is only shared during active delivery windows.
  - **375px Mobile Execution**:
      - **Driver/Owner View**: Instead of a complex map, a clean, Ubiquiti UniFi modular dashboard card presents the "Next Stop" with a large, translucent glass "Start Navigation" button. Native deep-linking opens Google/Apple Maps.
      - **Customer View**: Customers receive an SMS with a link to a lightweight, fast-loading, branded tracking page (macOS-style Translucent Glass UI) showing a progress bar (Prep -> Out for Delivery -> Arriving) and live ETA.
      - **Manager View**: A simple list view of active deliveries with status tags. No complex mapping needed unless toggled in "Advanced Settings".

  ### AI Agent Integration
  - **The Operations Agent**: Monitors incoming orders, groups them by geographic proximity and time window, and calculates the optimal route before dispatching.
  - **The Customer Service Agent**: Monitors the driver's progress against the ETA and proactively messages the customer via SMS/WhatsApp if there is a delay ("Hi! Maya is running about 10 mins behind due to traffic, see you soon!").

  ## Implementation Prompt
  **To Implementer Agent:**
  Implement the "Autonomous Local Delivery and Dispatch Mesh". Define the `DeliveryRoute` and `RouteStop` entities within the unified ledger. Integrate a background job queue that triggers "The Operations Agent" to calculate optimal sequences for daily local deliveries. Build a mobile-first "Next Stop" UI component for the driver/owner that provides a 1-tap deep link to native mapping apps. Finally, implement the webhook/trigger for "The CS Agent" to automatically dispatch SMS ETA updates to customers based on route progression. Ensure all UI components follow the OHC Premium design tokens (Translucent Glass, UniFi modular cards) and pass the "grandmother test". Do not prescribe specific database schemas or API endpoints.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []