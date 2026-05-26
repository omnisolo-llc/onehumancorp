issue_title: "Implement Invisible Omnichannel Fulfillment & Logistics Mesh"
issue_description: |
  **Estimated Scope**: Large

  **Problem Statement:**
  For a small business owner like Priya (boutique owner) or Maya (custom cakes), figuring out how to get their product to the customer is often the most anxiety-inducing part of the operation. Currently, small business owners must manually negotiate carrier rates, calculate box sizes (dimensional weight), figure out shipping zones, print labels using clunky desktop software, and handle local delivery zones or pickup windows.

  If Fatima (food cart) wants to handle local pre-orders, and Priya wants to ship nationwide, they both face massive friction. If they guess shipping costs wrong, it eats their entire profit margin. If a customer asks "Where is my package?", they have to manually find tracking numbers and reply.

  We need an **Invisible Omnichannel Fulfillment & Logistics Mesh** that handles all this complexity behind the scenes. When an order is placed, the system should automatically know the cheapest way to deliver it (shipping, local delivery, or scheduled pickup), automatically generate a ready-to-print label or a local delivery route, and let an AI Operations Agent handle tracking updates and customer inquiries invisibly.

  **Research Report:**
  Small business owners suffer significantly from shipping complexity, often losing up to 15% margin to inefficient practices and spending significant time answering WISMO ("Where is my order?") tickets. Existing solutions like Shopify require heavy manual configuration for box sizes, shipping zones, and radiuses, while Wix/Squarespace simply offload the problem to third-party dashboards like ShipStation.

  *Shopify*: Offers Shopify Shipping with discounted rates and label printing, but requires significant setup (box sizes, weights, shipping profiles). Local delivery requires configuring complex radiuses or zip code lists manually.
  *Wix & Squarespace*: Both integrate with Shippo or ShipStation, pushing the complexity onto third-party dashboards. They do not natively abstract the logistics decision-making well.
  *OneHumanCorp (OHC) Target State*: OHC will not ask users to configure "Shipping Profiles." Instead, OHC uses an AI agent to analyze product photos/descriptions to estimate weights and dimensions, automatically negotiates commercial rates via underlying APIs, and generates 1-tap printable labels or local delivery routes on the phone.

  **Design Doc:**
  *Architecture Diagram*
  ```mermaid
  erDiagram
      TENANT ||--o{ ORDER : receives
      ORDER ||--|| FULFILLMENT_INTENT : triggers
      FULFILLMENT_INTENT {
          string method "SHIPPING | LOCAL_DELIVERY | PICKUP"
          string status "PENDING | LABEL_READY | IN_TRANSIT | DELIVERED"
      }
      FULFILLMENT_INTENT ||--o{ CARRIER_RATE : fetches
      CARRIER_RATE {
          string carrier "USPS | FedEx | LocalCourier"
          float cost
      }
      FULFILLMENT_INTENT ||--|| SHIPPING_LABEL : generates
      SHIPPING_LABEL ||--|| TRACKING_EVENT : tracks

      TENANT ||--o{ INVENTORY_LOCATION : has
      INVENTORY_LOCATION {
          boolean supports_pickup
          boolean supports_shipping
      }

      AI_OPERATIONS_AGENT ||--o{ FULFILLMENT_INTENT : monitors
      AI_OPERATIONS_AGENT ||--o{ TRACKING_EVENT : ingests
  ```

  ```mermaid
  sequenceDiagram
      participant Customer
      participant CheckoutEngine
      participant LogisticsMesh
      participant CarrierAPI
      participant AI_Ops_Agent
      participant Merchant_Mobile

      Customer->>CheckoutEngine: Initiates checkout (Address provided)
      CheckoutEngine->>LogisticsMesh: Request fulfillment options
      LogisticsMesh->>CarrierAPI: Fetch real-time rates (background)
      LogisticsMesh-->>CheckoutEngine: Returns options (Ship, Pickup, Local)
      Customer->>CheckoutEngine: Completes order
      CheckoutEngine->>LogisticsMesh: Confirm fulfillment intent
      LogisticsMesh->>CarrierAPI: Purchase cheapest label
      LogisticsMesh-->>Merchant_Mobile: Push notification: "New Order! Label ready to print."
      CarrierAPI-->>LogisticsMesh: Tracking updates
      LogisticsMesh->>AI_Ops_Agent: Webhook event (In Transit)
      AI_Ops_Agent-->>Customer: Invisible SMS/Email: "Your package is on the way!"
  ```

  *UI Wireframes & Mobile UX Flow (375px Viewport)*
  Principles: macOS-style Translucent Glass materials + Ubiquiti UniFi modular dashboard cards. "Grandmother test" applied.

  Screen 1: Order Details & Fulfillment (Merchant View)
  - Header: Translucent glass sticky header. "Order #1042" + Customer Name.
  - Card 1 (Action Required): Large button: `[ Print Shipping Label ]` (Primary color, rounded corners). Subtext: "USPS Ground Advantage • $4.20 (Paid via balance)"
  - Card 2 (Order Summary): 2x Vegan Chocolate Cake, 1x Candles.
  - Card 3 (AI Operations Status): "Agent Operations: Monitoring tracking. Will notify customer on delivery."

  Screen 2: One-Tap Print / Dispatch
  - When `[ Print Shipping Label ]` is tapped, a bottom sheet slides up showing native iOS/Android print dialog (AirPrint/Cloud Print) targeted at their configured thermal printer or standard printer.
  - For local delivery, the button is `[ Start Delivery Route ]`, which opens a mapped route with one-click "Mark Delivered" actions.

  Mobile UX Flow:
  1. Merchant receives a push notification: "Order #1042 ready to fulfill."
  2. Taps notification -> Opens Order Details.
  3. Taps "Print Label".
  4. Applies label to box. The system automatically marks it as fulfilled and the AI agent texts the customer the tracking link.

  *AI Agent Integration Points*
  - AI Operations Agent: Monitors all active tracking numbers. If a package is delayed, it proactively texts the customer ("Hey, just a heads up, the carrier is running a day late...").
  - AI Catalog Agent: When a new product is added, this agent uses computer vision on the uploaded photo to estimate weight and dimensional volume, eliminating the need for the merchant to manually enter 12x12x4 inches / 2 lbs.

  *Key Design Decisions*
  1. Zero-Configuration Shipping Profiles: Merchants do not set up shipping rules. The AI estimates size/weight, and the system quotes flat or calculated rates automatically during checkout based on the delivery distance.
  2. Abstracted Carriers: The merchant doesn't care if it's USPS or UPS. They just see "Standard Shipping" and the system picks the most cost-effective option meeting the SLA.
  3. Multi-Tenant Isolation: Carrier credentials (if BYO) and default sender addresses are strictly isolated per tenant using the `TenantRegistry` bounds.

  **Implementation Prompt:**
  To the Implementer Agent:
  Your task is to build the core architecture for the Invisible Omnichannel Fulfillment & Logistics Mesh.

  User-Facing Outcome:
  When a customer checks out, the system should dynamically offer Shipping, Local Delivery, or Pickup based on the merchant's capabilities and the customer's location. When the order is placed, the merchant should see a single action button on their mobile dashboard to "Print Label" or "Start Route". The system must automatically buy the cheapest label matching the required delivery speed and track it.

  Core User Journeys (CUJs):
  1. Checkout Rating: As a customer, I enter my address and instantly see accurate shipping or local delivery options.
  2. 1-Tap Fulfill: As a merchant, I open an order and tap a single button to generate a shipping label, without needing to type in box dimensions or select a carrier.
  3. Proactive Tracking: As a customer, I receive a personalized SMS from the AI Operations Agent when my package ships and when it is delivered.

  Acceptance Criteria:
  - The data model must support multi-tenant isolation for fulfillment intents, shipping labels, and local delivery zones.
  - The system must provide an interface (or hook) for an AI agent to ingest tracking state changes and trigger communications.
  - Provide a robust mock or integration boundary for carrier APIs (like Shippo or EasyPost) so E2E tests can run without hitting real rate-limits.
  - Implement the backend logic and the corresponding UI dashboard cards following the UniFi/Glass design system guidelines for a 375px viewport.
  - Do not prescribe specific database schemas or API endpoints in this brief; design the clean abstractions and ensure all E2E tests pass. Ensure mobile-first responsiveness.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
