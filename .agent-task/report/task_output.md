issue_title: "Hyperlocal AI Fleet and Delivery Mesh"
issue_description: |
  # Hyperlocal AI Fleet and Delivery Mesh

  ## 1. Title
  Hyperlocal AI Fleet and Delivery Mesh

  ## 2. Problem Statement
  **The Pain Point:** Small businesses like Maya the baker or Fatima the food cart owner often rely on manual, chaotic processes for local deliveries. They might write down addresses on Post-it notes, manually plug them into Google Maps, text drivers or family members to coordinate drops, and have no way to automatically update customers on ETA. This causes delayed deliveries, lost products, angry customers, and massive time sinks.
  **The Opportunity:** Give non-technical SMB owners a Zero-Touch, fully automated local delivery dispatch, routing, and tracking system. They just mark an order for "Local Delivery" and the AI handles driver assignment, route optimization, customer ETA texting, and proof-of-delivery seamlessly.

  ## 3. Research Report
  **Market Context & Findings:**
  - **Shopify:** Requires third-party apps (like ShipStation or local delivery add-ons) which often have clunky integration, separate billing, and manual dispatching steps. Not truly "zero touch."
  - **Wix:** Has basic local delivery zones based on zip code or radius but lacks dynamic multi-stop route optimization and AI driver dispatching out-of-the-box.
  - **GoDaddy/Squarespace:** Barebones local delivery options. The business owner still has to figure out *how* to deliver it.

  **Why OHC Wins:**
  By natively integrating an AI-driven delivery mesh into the core ledger and order flow, OHC eliminates the need for third-party logistics software. OHC automatically clusters orders by geographic proximity, calculates the most efficient route using real-time traffic data, dispatches instructions to the assigned driver (even if it's just the owner's teenager), and handles all customer communication invisibly.

  ## 4. Design Doc

  ### Architecture Diagram

  ```mermaid
  erDiagram
      ORDER ||--o{ DELIVERY_TASK : "spawns"
      DELIVERY_TASK }|--|| DRIVER : "assigned to"
      DRIVER ||--o{ ROUTE_SEGMENT : "navigates"
      DELIVERY_TASK }|--|| CUSTOMER : "updates"

      ORDER {
          string order_id
          string status
          string delivery_address
      }
      DELIVERY_TASK {
          string task_id
          string status "pending, in_transit, delivered, failed"
          string estimated_eta
          string proof_of_delivery_url
      }
      DRIVER {
          string driver_id
          string name
          string current_location
          boolean is_active
      }
      ROUTE_SEGMENT {
          string segment_id
          int stop_order
          string polyline
      }
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)

  **Screen 1: Delivery Dashboard (Owner View)**
  - **Header:** "Deliveries Today"
  - **Glassmorphism Card 1:** "Maya's Route" (3 stops, 45 mins estimated).
    - *Action:* [Dispatch Route] (Big, thumb-friendly primary button)
  - **Glassmorphism Card 2:** "Unassigned Orders" (2 orders waiting).
    - *Action:* [Auto-Assign] (AI automatically groups and assigns based on location/time).

  **Screen 2: Driver App (Driver View - e.g., Maya's teenager)**
  - **Full Screen Map:** Showing the optimized route.
  - **Bottom Drawer (Swipe up):** Current stop details. "Drop off: Vegan Chocolate Cake for Sarah. 123 Main St."
  - **Action Buttons:** [Navigate] (Opens Apple/Google Maps), [Mark Delivered] (Triggers camera for proof-of-delivery photo).

  **Screen 3: Customer Tracking View (Web/SMS Link)**
  - Clean, branded, minimalist status page.
  - "Your order is 3 stops away. Estimated arrival: 2:15 PM."
  - Live map showing driver proximity (obfuscated slightly for privacy).

  ### AI Agent Integration Points
  - **Operations Department:** The **AI Dispatcher Agent** continuously monitors new local delivery orders, groups them into optimized routes (reducing backtracking), and assigns them to available drivers.
  - **Customer Service Department:** The **AI CS Agent** intercepts replies to ETA SMS messages. If a customer texts "Leave it on the porch!", the AI CS Agent updates the delivery notes for the driver instantly.
  - **Memory & Context:** Remembers gate codes or specific customer delivery preferences for future orders without manual data entry.

  ### Key Design Decisions
  - **Zero-Trust & Multi-Tenancy:** Delivery tasks and driver locations are strictly isolated by `organization_id`. Cross-tenant location leakage is impossible.
  - **Offline-First Driver App:** The driver app must cache the route and delivery details. Proof-of-delivery photos and status updates sync automatically when connectivity is restored to handle cellular dead zones.
  - **Mac-Style Translucent Materials:** The UI will use premium, frosted-glass components for all cards and modals, passing the "grandmother test" by prioritizing clear actions and hiding complex routing algorithms.

  ## 5. Implementation Prompt
  **Outcome:** Implement the "Hyperlocal AI Fleet and Delivery Mesh" backend services and mobile-first UI components.
  **User Journey (CUJ):**
  1. A small business owner receives 5 local delivery orders.
  2. The owner opens the "Deliveries" tab on their phone. The AI has already grouped the 5 orders into two optimized routes and suggested assigning them to Driver A and Driver B.
  3. The owner taps "Dispatch All".
  4. Drivers receive an SMS with a link to their route and start navigating.
  5. Customers automatically receive SMS ETA updates.
  **Acceptance Criteria:**
  - The system must automatically group pending orders into geographic routes.
  - The UI must perfectly render on a 375px viewport using the designated design tokens (translucent glass materials, modular cards).
  - Driver actions (like "Mark Delivered" with photo upload) must function offline and sync upon reconnection.
  - AI CS Agent must be able to parse incoming SMS from customers and update delivery notes.

  ## 6. Priority
  `P1` (High)

  ## 7. Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []