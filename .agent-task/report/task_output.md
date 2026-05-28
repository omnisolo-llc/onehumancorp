issue_title: "Design Autonomous Local Delivery & Dispatch Engine"
issue_description: |
  # Title: Autonomous Local Delivery & Dispatch Engine

  ## Problem Statement
  Maya (Baker, 28) and Fatima (Food Cart Operator, 50) have thriving businesses that require local delivery. However, managing local deliveries manually is chaotic. Maya receives DMs from customers asking "Where is my cake?" while she is baking. She has to coordinate with local couriers or her own delivery staff, track their locations, calculate optimal routes, and update customers all via messy group chats and manual text messages. Existing delivery dispatch software (like Onfleet or Routific) is overly complex, requires separate costly subscriptions, and forces non-technical owners to integrate APIs manually. They need a zero-touch system that automatically groups orders, dispatches local drivers, optimizes routes, and provides real-time Uber-like tracking to customers without requiring manual intervention from the business owner.

  ## Research Report
  *   **Competitor Analysis**:
      *   **Onfleet**: Powerful but strictly for enterprise or dedicated delivery businesses. Pricing is too high for micro-SMBs, and the interface is overwhelming.
      *   **Shopify Local Delivery**: Basic route planning but lacks autonomous dispatch, real-time tracking for customers, and native AI handling of customer inquiries about delivery status.
      *   **DoorDash/UberEats**: Takes 20-30% commissions. Destroys margin for local small businesses who want to manage their own local delivery or hire independent couriers directly.
  *   **The OHC Differentiator**: OHC must provide a native, zero-config dispatch engine embedded directly within the commerce flow. It will use AI Operations Agents to autonomously batch orders by geographic proximity and dispatch to registered drivers, while AI CS Agents intercept customer "Where is my order?" inquiries and answer them using real-time GPS tracking data.

  ## Design Doc

  ### High-Level Architecture
  ```mermaid
  graph TD;
      Customer[Customer on Storefront] -->|Places Local Delivery Order| Gateway[Zero-Trust Edge Gateway];
      Gateway --> KAIROS[KAIROS Orchestration Hub];
      KAIROS --> OrderLedger[(Global Order Ledger)];
      KAIROS --> DispatchEngine[Local Dispatch Engine];
      DispatchEngine --> RouteOptimizer[Route Optimization & Batching Service];
      RouteOptimizer --> EventMesh[Hybrid Event Mesh];
      EventMesh -->|Dispatch Push| DriverApp[OHC App: Driver View];
      DriverApp -->|Real-Time GPS| EventMesh;
      EventMesh --> Tracker[Customer Tracking Page];
      EventMesh --> CSAgent[AI CS Agent];
      CSAgent -->|Answers "Where is my order?"| Customer;
  ```

  ### Key Design Decisions & Invariants
  *   **Zero Trust & Security**: All driver location updates are authenticated via SPIFFE/SPIRE ensuring multi-tenant isolation. Location data is strictly segregated per organization ID.
  *   **Mobile-First UX Flow**:
      *   **Business Owner (375px)**: A simple toggle "Enable Local Delivery". The owner sees a clean, translucent glass dashboard showing a map with pulsing dots representing active drivers and pending orders. No complex configuration required.
      *   **Driver (375px)**: A distraction-free UI with high-contrast, large tap targets for "Accept Route", "Navigate", and "Mark Delivered (with Photo)". Optimized for one-handed use in a car or on a bike.
  *   **AI Department Coordination**:
      *   **Operations Agent**: Continuously monitors the order queue. Automatically groups nearby orders within a configurable time window and dispatches them to available drivers via push notifications.
      *   **Customer Success Agent**: Hooks into the unified inbox. When a customer messages "Where's my cake?", the CS agent queries the real-time location stream and responds conversationally (e.g., "Your driver, Alex, is 2 blocks away!").
  *   **Performance Targets**: Driver GPS updates must be processed with sub-200ms latency to provide smooth tracking to customers. The driver app must support offline caching so delivery completion (and photo proof) can be recorded even in dead zones and synced when connectivity is restored.

  ## Implementation Prompt
  **Task for Implementer**: Build the core Autonomous Local Delivery & Dispatch Engine backend services and the mobile-first UI components for both the business owner and the driver.
  - The solution must allow a business owner to toggle local delivery on.
  - It must provide a simple API/service boundary for the Operations Agent to batch orders.
  - It must expose real-time location streams (via WebSockets or SSE) for the customer tracking page.
  - Build the UI using macOS-style Translucent Glass materials and clean dashboard cards. Keep all complex settings (like custom geofencing or advanced routing constraints) hidden behind an "Advanced Settings" switch. Ensure the driver interface is highly legible and passes the grandmother test.
  - Ensure all queries are scoped by `organization_id` for strict multi-tenancy.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
