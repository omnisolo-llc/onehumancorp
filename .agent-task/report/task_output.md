issue_title: "[Architecture] Unified Capacity & Inventory Reservation Mesh"
issue_description: |
  # [Architecture] Unified Capacity & Inventory Reservation Mesh

  ## Problem Statement
  For hybrid businesses, what they sell isn't just a physical product or just a time slot—it's often a combination of both. Maya sells pre-made cakes (physical inventory) but also offers custom cake design consultations (time capacity). When treated as disconnected systems, she ends up double-booking. She needs a unified system where "availability" automatically accounts for both time and physical goods seamlessly.

  ## Research Report
  - **Shopify:** Dominates physical inventory tracking. Time-based bookings require third-party apps that do not share a unified reservation state.
  - **Wix / Squarespace:** Offer both "Stores" and "Bookings" modules, but they exist as distinct silos.
  - **OHC Gap:** To deliver a true "business in a box," OHC must abstract away the difference between a "Product" and a "Service." A unified mesh allows the AI Operations Agent to manage reservations holistically.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ RESOURCE : owns
      RESOURCE ||--o{ RESERVATION_HOLD : has
      RESOURCE {
          string id PK
          string type "Inventory | Capacity"
          int total_quantity
          json availability_schedule "For capacity"
      }
      RESERVATION_HOLD {
          string id PK
          string resource_id FK
          int quantity
          timestamp expires_at
          string status "Pending | Confirmed | Released"
      }
      TENANT ||--o{ ORDER : processes
      ORDER ||--o{ RESERVATION_HOLD : confirms
  ```

  ### Mobile UX Flow (375px First)
  - **Step 1:** Customer browses the storefront and selects a hybrid product.
  - **Step 2:** The Mesh instantly places a temporary "Hold" on both the 1-hour time slot and physical inventory during checkout.
  - **Step 3:** If they abandon the cart, the hold expires and both time and inventory are automatically released.
  - **Step 4:** Upon purchase, the owner receives a single notification confirming the booking and the reserved equipment.

  ### Key Design Decisions
  - **Unified Resource Model:** By treating both physical items and time blocks simply as `RESOURCE` entities with different constraint types, the checkout engine only has to interact with a single Reservation Mesh.
  - **Temporary Holds (Optimistic Locking):** To prevent race conditions, adding an item to the cart creates a fast-expiring `RESERVATION_HOLD`. If payment fails or times out, it auto-releases.

  ## Implementation Prompt
  Build the core Unified Capacity & Inventory Reservation Mesh.
  Implement a single API boundary (the "Mesh") that handles reserving both physical quantity and time slots. Implement the temporary `RESERVATION_HOLD` mechanism with an automatic expiration sweep. Provide an endpoint for the checkout engine to convert a hold into a confirmed state. Guarantee strict multi-tenant isolation.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
