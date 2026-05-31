issue_title: "OHC Magic Dispatch: AI-Driven Shipping & Local Delivery Engine"
issue_description: |
  # Architecture Brief: Autonomous Shipping & Local Delivery Orchestrator

  ## Problem Statement
  Small business owners (Maya the Baker, Priya the Boutique Owner, Fatima the Food Cart Operator) struggle with configuring complex shipping zones, calculating exact box weights, and organizing local deliveries. Maya needs to route her cake deliveries without manually plotting maps. Priya needs to ship nationwide without spending hours learning carrier rate logic. They need an intelligent, zero-configuration fulfillment orchestrator that "just works." If they can't enable nationwide shipping or local delivery with a single tap, OHC has failed the simplicity test.

  ## Research Report
  - **Competitive Benchmark**: Shopify requires complex "Shipping Profiles" and third-party apps for local delivery routing. Wix has static rate tables. OHC needs a dynamic, AI-configured engine.
  - **SMB Pain Points**: Configuring carrier integrations (USPS, FedEx) is technically daunting. Calculating dimensional weight is confusing. Local delivery routing is highly inefficient for self-delivering owners.
  - **The OHC Advantage**: AI agents can automatically estimate box dimensions from product photos, instantly fetch the cheapest carrier rates (via Shippo/EasyPost), and automatically sequence local delivery routes (via Google Maps API).

  ## Design Doc

  ### High-Level Architecture (Mermaid.js)
  ```mermaid
  graph TD
      Order[New Order Received] --> Dispatcher[The Operations Agent]
      Dispatcher -->|Analyzes Cart| Logic{Fulfillment Type}

      Logic -->|Local Delivery| LocalRoute[Local Route Engine]
      Logic -->|Nationwide| Shipping[Carrier Rate Engine]

      Shipping -->|API| Shippo[Shippo/EasyPost Integration]
      Shippo -->|Best Rate| Label[Auto-Generate Label]
      Label --> Thermal[Thermal Printer Mesh]

      LocalRoute -->|Geo-Sequence| Map[Route Optimizer]
      Map --> DriverApp[Driver/Owner Mobile App]

      Thermal --> Notify[The Ambassador Agent]
      DriverApp --> Notify
      Notify --> Customer[SMS/Email Status Update]
  ```

  ### Mobile UX Flow (375px First)
  1. **Fulfillment Setup Card**: Single-tap toggle for "Offer Local Delivery" and "Offer Nationwide Shipping".
  2. **Order Detail Screen**: "Ready to Ship" button automatically purchases the cheapest label.
  3. **Local Route Screen**: A clean, map-based interface showing the optimal delivery sequence for the day's local orders.
  4. **Thermal Print Integration**: One-tap "Print Label" that sends the label directly to a Bluetooth/network thermal printer without driver installations.

  ### AI Agent Integration Points
  - **The Operations Agent**: Predicts box sizes and weight based on the cart's contents, selects the cheapest carrier automatically, and batches local deliveries.
  - **The Ambassador Agent**: Sends proactive, conversational updates to the customer ("Maya is 10 minutes away with your cake!").

  ## Implementation Prompt
  **To Implementer Agent:**
  Implement the "Magic Dispatch" Shipping and Local Delivery engine.
  1. Build the Data Model for Fulfillment Logic (Local vs. Carrier).
  2. Integrate a multi-carrier API (e.g., Shippo or EasyPost) to fetch rates dynamically behind the scenes.
  3. Build the Local Delivery route sequencer that optimizes stops.
  4. Ensure the UI allows users to fulfill an order with exactly one tap on mobile (375px), generating a label or a route instantly.
  Do not prescribe specific database schemas—design the data layer for multi-tenant isolation. All actions must be instrumented with OpenTelemetry.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
