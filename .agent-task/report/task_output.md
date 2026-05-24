issue_title: "Autonomous Hybrid Delivery & Local Fulfillment Mesh"
issue_description: |
  # Title: Autonomous Hybrid Delivery & Local Fulfillment Mesh

  ## Problem Statement
  Small business owners like Fatima (food cart) and Maya (baker) deal with highly complex, localized logistics. They need to handle instant local pickups, scheduled pre-orders, third-party delivery dispatch (e.g., DoorDash Drive, Uber Direct), and even personal local delivery routes. Currently, managing these disparate fulfillment methods requires multiple apps, manual route planning, and constant context switching. When an order changes or a delivery is delayed, the merchant has to manually inform the customer and the driver, leading to errors, cold food, and angry customers. A unified, autonomous hybrid delivery mesh is needed to invisibly orchestrate local fulfillment across personal staff, external fleets, and customer pickups.

  ## Research Report
  *   **Current Architecture Limits:** OHC currently relies on basic shipping integrations (Shippo, EasyPost) and generic calendar systems which cannot handle hyper-local, real-time dispatch, fleet routing, and dynamic preparation timing.
  *   **Competitor Analysis:**
      *   *Shopify:* Has local delivery routing but relies heavily on third-party apps for complex real-time fleet orchestration and on-demand courier dispatch.
      *   *Square/Toast:* Strong in restaurant pickup/delivery, but rigid and heavily focused on traditional food service rather than a unified platform for all business types (e.g., Maya's custom cakes).
      *   *Wix:* Basic local delivery features but lacks real-time AI dispatch and multi-provider fleet orchestration.
  *   **Discovery:** We need an Autonomous Hybrid Delivery Mesh that treats personal staff drivers, customer pickups, and on-demand external couriers (Uber, DoorDash) as interchangeable nodes in a real-time fulfillment graph. The Operations AI agent must automatically dispatch the most efficient method, calculate prep times dynamically, and keep customers informed without merchant intervention.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      ORDER-INGESTION ||--|| AI-OPERATIONS-AGENT : "Triggers"
      AI-OPERATIONS-AGENT ||--o{ DELIVERY-FLEET-ROUTER : "Calculates optimal path"
      DELIVERY-FLEET-ROUTER }|--|| STAFF-DRIVER-APP : "Dispatches"
      DELIVERY-FLEET-ROUTER }|--|| ON-DEMAND-COURIER-API : "Dispatches"
      DELIVERY-FLEET-ROUTER }|--|| CUSTOMER-PICKUP-TRACKER : "Orchestrates"
      AI-OPERATIONS-AGENT ||--o{ INVENTORY-PREP-ENGINE : "Manages lead time"
      AI-CS-AGENT ||--|| CUSTOMER-COMMUNICATION : "Updates via SMS/WhatsApp"
  ```

  ### UI Wireframes & Mobile UX Flow (375px)
  *   **Merchant View (OHC Mobile App - 375px):**
      *   **Unified Fulfillment Radar:** A clean dashboard card utilizing the macOS-style Translucent Glass aesthetic. It shows active orders on a mini-map with real-time dots (blue for staff, green for customer pickup, purple for external courier).
      *   **Auto-Dispatch Toggle:** A simple "Grandmother Test" approved toggle: "Let AI manage delivery." When enabled, the AI automatically assigns the best courier or staff member based on cost and time.
  *   **Driver View (OHC Mobile App - 375px):**
      *   **Minimal Route UI:** Staff drivers see a highly optimized, high-contrast, Uber-like interface for their next stop. One-tap "Arrived" and "Delivered" buttons.
  *   **Customer View (Mobile Web/SMS):**
      *   **Live Tracker:** A sleek, unbranded live tracking page showing driver progress and AI-generated ETA updates (e.g., "Your cake is carefully on its way!").

  ### Key Design Decisions
  *   **Abstracted Fulfillment Nodes:** Personal drivers and Uber Direct are treated as identical entities in the data model with differing cost/latency weights, allowing the AI to optimize routing instantly.
  *   **Dynamic Prep-Time Coupling:** Delivery dispatch is strictly coupled with the `INVENTORY-PREP-ENGINE`. The system back-calculates when to request a courier based on real-time kitchen/prep load to ensure food/goods aren't sitting cold.
  *   **Zero-Touch Exception Handling:** If a driver cancels or is delayed, the `AI-OPERATIONS-AGENT` automatically re-dispatches to a fallback method and the `AI-CS-AGENT` texts the customer. The merchant is only notified if all fallbacks fail.

  ### AI Agent Integration Points
  *   **Operations Agent:** Constantly evaluates delivery costs vs. speed, handles dispatching, and monitors prep times.
  *   **Customer Service (CS) Agent:** Proactively texts customers about delays or arrival times ("Hi, Carlos is 2 mins away!").
  *   **Finance Agent:** Reconciles on-demand courier fees against order margins.

  ## Implementation Prompt
  Implement the Autonomous Hybrid Delivery & Local Fulfillment Mesh. The outcome must provide a unified ledger and routing engine that abstracts customer pickups, staff deliveries, and 3rd-party on-demand couriers. Ensure the Operations AI can seamlessly evaluate and dispatch the optimal fulfillment method based on cost, distance, and real-time prep capacity. Include a comprehensive state machine for delivery status (Preparing, Dispatched, Arrived, Completed, Exception). Do not prescribe specific courier APIs (e.g., Uber vs. DoorDash) but define a robust provider interface. Acceptance criteria: The system can accurately calculate dynamic prep-times, dispatch to simulated local couriers, and automatically re-route an order upon a simulated driver failure, entirely without merchant manual intervention. Ensure strict multi-tenant isolation.

  ## Estimated Scope
  Large

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []