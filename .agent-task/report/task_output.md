issue_title: "[Architecture] Autonomous Unified Capacity & Inventory Mesh"
issue_description: |
  # Autonomous Unified Capacity & Inventory Reservation Mesh

  ## Problem Statement
  For hybrid businesses like Maya (the baker) or Leo (the music tutor), what they sell isn't just a physical product or just a time slot—it's often a combination of both. Maya sells pre-made vegan cakes (physical inventory) but also offers custom cake design consultations and delivery slots (time capacity). When these are treated as disconnected systems, she ends up double-booking a delivery slot while simultaneously selling out of the cake batter she needs. She shouldn't have to manually reconcile a calendar app with an e-commerce inventory tracker. She needs a unified system where "availability" automatically accounts for both time and physical goods seamlessly.

  ## Research Report
  *   **Shopify**: Dominates physical inventory tracking. However, time-based bookings require third-party apps (like Sesami or Appointo). These apps bolt onto the order flow but do not share a unified underlying reservation state, leading to race conditions during high-traffic sales.
  *   **Wix / Squarespace**: Offer both "Stores" and "Bookings" modules, but they exist as distinct silos. Booking a service does not natively reserve physical resources required for that service.
  *   **Stripe**: Excellent at processing the transaction but relies on the platform (or custom code) to handle the complex logic of resource locking and capacity management before the payment is captured.
  *   **The OHC Gap**: To deliver true "business in a box," OHC must abstract away the difference between a "Product" and a "Service." A unified mesh allows the AI Operations Agent to manage reservations holistically, whether it's reserving 5 cupcakes or 1 hour of Maya's time.

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

  ```mermaid
  sequenceDiagram
      participant Mobile as Mobile App (375px)
      participant OHA as Operations AI Agent
      participant Mesh as Capacity & Inventory Mesh
      participant Cart as Checkout Engine

      Mobile->>Mesh: Request: Add Custom Cake + Delivery Slot (Saturday 2 PM)
      Mesh->>Mesh: Acquire Reservation Hold (Cake Batter + 1hr Delivery Time)
      Mesh-->>Mobile: Hold Secured (Expires in 10 mins)
      Mobile->>Cart: Proceed to Checkout
      Cart->>Mobile: Payment Successful
      Cart->>Mesh: Commit Reservation (Status -> Confirmed)
      Mesh->>OHA: Trigger: Fulfillment Prep
      OHA->>Mobile: Push: "Maya, 1 new custom order confirmed for Saturday."
  ```

  ### UI Wireframes & Screen Flow (375px Mobile-First)
  1. **Unified Catalog Screen**: Translucent Glass cards. A single list showing both "Vegan Cupcakes" (Inventory: 12 left) and "Cake Tasting Consultation" (Availability: 3 slots today). No jarring context switch between "Store" and "Calendar".
  2. **Product + Time Selection**: When a customer selects "Custom Wedding Cake," a smooth bottom-sheet modal slides up. It asks for the flavor (Inventory Check) and the required delivery date/time (Capacity Check) in one fluid form.
  3. **Owner Dashboard Widget**: "Availability" widget showing upcoming bottlenecks (e.g., "You have 3 consultations tomorrow, but are running low on tasting samples.").

  ### Mobile UX Flow
  - **Step 1**: The customer browses the storefront on their phone and selects a hybrid product (e.g., "Guitar Lesson + Rent a Guitar").
  - **Step 2**: The Mesh instantly places a temporary "Hold" on both the 1-hour time slot and the physical guitar inventory while they are in the checkout flow.
  - **Step 3**: If they abandon the cart, the hold expires and both time and inventory are automatically released.
  - **Step 4**: Upon purchase, Leo (the owner) receives a single notification confirming the booking and the reserved equipment.

  ### Performance & Offline Targets
  - **Strict Latency**: Availability checks must return in < 150ms to prevent UI freezing during checkout.
  - **Offline Capability**: Holds are primarily an online function, but the owner's dashboard must cache confirmed reservations locally so they can view their schedule without a connection.
  - **Payload Targets**: The `availability_schedule` JSON must be compressed or paginated, ensuring the API response payload for the unified mesh remains < 50KB over 4G connections.

  ### Design Tokens (Visual Excellence Mandate)
  - **Colors**: Primary Action (Saturate 210% Green), Critical Alert (Soft Crimson), Background (Translucent Light Gray blur 30px).
  - **Spacing**: Modular cards use standard 16px margins, internal paddings of 12px for list items, with an 8px gap between stacked items.
  - **Motion**: 300ms ease-in-out for bottom-sheet modal slides. Checkmark confirmations on success use a fast 150ms spring animation.

  ### AI Agent Integration Points
  - **Operations Agent (The Manager)**: Monitors the `RESERVATION_HOLD` state. If a physical resource runs low but future capacity depends on it, the agent alerts the owner (e.g., "Maya, you have 5 cakes booked for Friday but only enough flour for 2.").
  - **Sales Agent (The Ambassador)**: Uses unified availability data to answer customer DMs (e.g., "Yes, we can deliver a cake this Friday at 4 PM!").

  ### Key Design Decisions & Integrity
  - **Unified Resource Model**: By treating both physical items and time blocks simply as `RESOURCE` entities with different constraint types, the checkout engine only has to interact with a single Reservation Mesh.
  - **Temporary Holds (Optimistic Locking)**: To prevent race conditions (especially for high-demand drops), adding an item to the cart creates a fast-expiring `RESERVATION_HOLD`. If payment fails or times out, it auto-releases.
  - **Zero Trust & Multi-Tenancy**: All availability checks and holds must be strictly scoped to the `tenant_id` at the database level to prevent accidental cross-tenant data leaks.
  - **"Grandmother Test" Approved**: The owner never sees the complex hold logic. They just see a single "Add New Item" button where they can attach a schedule to a physical product seamlessly.

  ## Implementation Prompt
  **Task for Implementer**: Build the core Unified Capacity & Inventory Reservation Mesh.

  **User Journey (CUJ)**:
  1. Maya creates a "Custom Cake Delivery" item in her catalog. It requires 1 physical cake from inventory and a 1-hour delivery time slot.
  2. A customer attempts to buy this item for Saturday at 2 PM.
  3. The system checks availability for BOTH the physical inventory and Maya's schedule at that exact time.
  4. If available, the system places a temporary 10-minute hold on both resources.
  5. Upon successful payment, the hold is converted to a confirmed reservation.
  6. If the 10 minutes pass without payment, the hold is released automatically.

  **Acceptance Criteria**:
  - Implement a single API boundary (the "Mesh") that handles reserving both physical quantity and time slots.
  - Implement the temporary `RESERVATION_HOLD` mechanism with an automatic expiration sweep (e.g., background job or TTL).
  - Provide an endpoint for the checkout engine to convert a hold into a confirmed state.
  - Guarantee strict multi-tenant isolation (all database operations must require `tenant_id`).
  - Do NOT prescribe specific database technologies (e.g., Redis vs Postgres) in the API design; focus on the robust state transition of the hold mechanism.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
