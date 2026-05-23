issue_title: "Hyperlocal Fleet & Delivery Routing Engine"
issue_description: |
  # Autonomous Hyperlocal Fleet & Delivery Routing Engine

  ## Problem Statement

  Small business owners—like Fatima (food cart) and Maya (custom cakes)—often manage their own local deliveries or coordinate small teams of drivers. They lack the resources to use complex enterprise logistics software like Onfleet or Routific. Instead, they rely on manual text messages, Google Maps multi-stop routes (which have strict limits), and frantic phone calls to drivers to ask "Where are you?". Customers are left in the dark without live tracking, leading to an overwhelming amount of "Where is my order?" (WISMO) support queries. The entire delivery coordination is a fragmented, high-friction, completely manual process that limits their ability to scale local sales.

  ## Research Report

  **Competitor Landscape:**
  - **Shopify:** Supports local delivery options but primarily just tags the order. Live routing, driver apps, and real-time customer tracking require expensive third-party apps (e.g., Zapiet, EasyRoutes) which start at $30-$50/month and add significant configuration complexity.
  - **Wix:** Basic local delivery zones are supported. Routing requires external tools.
  - **Square:** Good for local pickup and basic delivery fulfillment tracking but doesn't natively provide optimized fleet routing and live tracking URLs for customers without add-ons.
  - **Dedicated Logistics (Onfleet, Tookan):** Too complex and expensive for a single baker or a 2-person food cart team. Setup takes days and requires technical API integrations.

  **The Opportunity:**
  OneHumanCorp can build an invisible, zero-config Hyperlocal Fleet & Delivery Routing Engine. When a user creates a local delivery order, the Operations AI automatically groups it with other local orders, optimizes the route based on real-time traffic, dispatches it to a simple driver-view via SMS/WhatsApp, and automatically texts the end customer a live tracking link. No separate app downloads are required for drivers or customers.

  ## Design Doc

  ### Architecture Diagram

  ```mermaid
  erDiagram
      DELIVERY_ORDER {
          string id
          string tenant_id
          string customer_id
          string destination_address
          string status "pending, routed, out_for_delivery, delivered"
          timestamp promised_time
      }
      DELIVERY_ROUTE {
          string id
          string tenant_id
          string driver_id
          string status "active, completed"
          json route_geometry
      }
      DRIVER_SESSION {
          string id
          string phone_number
          string current_location
          string magic_link_url
      }

      DELIVERY_ROUTE ||--o{ DELIVERY_ORDER : contains
      DRIVER_SESSION ||--o{ DELIVERY_ROUTE : assigned_to
  ```

  ```mermaid
  sequenceDiagram
      participant C as Customer
      participant M as Merchant (Maya)
      participant OAI as Operations AI
      participant D as Driver

      C->>M: Places order for local delivery
      M->>OAI: New order created
      OAI->>OAI: Groups pending orders based on proximity & time windows
      OAI->>M: Proposes optimized route
      M->>OAI: 1-Tap Approve Route
      OAI->>D: SMS with Magic Link to Driver Session
      OAI->>C: SMS with Live Tracking URL
      D->>D: Driver opens link, marks "On the way"
      D->>OAI: Sends location pings
      OAI->>C: Updates live tracking map
      D->>D: Driver marks "Delivered" & uploads photo
      OAI->>M: Order status marked complete
  ```

  ### Mobile UX Flow (375px First)

  1. **Merchant Routing Dashboard (App/Web):**
     - Translucent glass card layout displaying "3 Pending Local Deliveries".
     - "Optimize Route" large primary action button.
     - A map card showing the AI-proposed route with drop sequence.
     - "Dispatch to Driver" button, opening a simple modal to select a staff member or enter a phone number.

  2. **Driver Magic Link View (Mobile Browser, Zero App Install):**
     - Clean, high-contrast UI suitable for outdoor viewing.
     - Large map at the top.
     - List of stops below in a carousel or list.
     - Large, swipeable "Mark Arrived" and "Complete Delivery" buttons.
     - Button to "Navigate" which deep-links to Google Maps / Apple Maps.
     - Optional camera button for proof-of-delivery photo.

  3. **Customer Tracking View (Mobile Browser):**
     - Minimalist map showing driver's live location.
     - ETA countdown text (e.g., "Arriving in ~15 mins").
     - "Contact Driver" button (routes through an anonymized proxy or OHC Inbox).

  ### AI Agent Integration Points

  - **Operations Agent:** Monitors incoming local delivery orders and periodically runs route optimization (TSP algorithms) to propose efficient delivery batches.
  - **Customer Service Agent:** Automatically intercepts WISMO ("Where is my order?") messages via SMS or Instagram DM and responds with the live tracking link and ETA, entirely handling the inquiry.
  - **Communication Mesh:** Handles sending the magic links via Twilio/MessageBird and routing anonymized driver-customer calls.

  ### Key Design Decisions

  - **Zero-App Install for Drivers:** To ensure flexibility (e.g., Maya asking her brother to deliver a cake), drivers receive a magic web link via SMS instead of needing to download a specialized driver app.
  - **Multi-Tenant Privacy:** Driver location data and customer addresses must strictly adhere to the `TenantRegistry` boundaries.
  - **AI-Managed WISMO:** The primary goal is reducing merchant friction. The AI proactively sending tracking links and fielding inquiries is critical.

  ## Implementation Prompt

  **User-Facing Outcome:**
  Merchants can seamlessly dispatch local delivery orders to drivers with 1-click optimized routing. Drivers follow a simple web-based manifest without downloading an app. Customers receive SMS tracking links and can see their delivery arrive in real-time.

  **Critical User Journeys (CUJ):**
  1. Merchant views unfulfilled local orders and taps "Optimize Route."
  2. Merchant assigns the route to a driver's phone number.
  3. Driver receives an SMS, opens the web link, and navigates to the first stop.
  4. Customer receives an SMS with a tracking link showing the driver's progress.
  5. Driver completes delivery, triggering an automatic status update and notification.

  **Acceptance Criteria:**
  - The backend can calculate and store an optimized sequence for a set of delivery addresses.
  - Drivers can access a secure, mobile-optimized web view via magic link to view their route and update statuses.
  - Customers can access a public, read-only tracking page that displays the delivery status.
  - The UI adheres to the macOS-style Translucent Glass and UniFi modular dashboard design system.
  - The entire flow works flawlessly on a 375px mobile viewport.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []