issue_title: "Implement Autonomous Embedded Local Delivery & Route Optimization Engine"
issue_description: |
  # Mission Queue Protocol: Autonomous Embedded Local Delivery & Route Optimization Engine

  ## Problem Statement
  Small business owners such as Maya (a custom cake baker) and Fatima (a food cart operator) struggle with managing local deliveries. Current e-commerce platforms like Shopify or Wix require complex third-party routing apps (like Routeific or Onfleet) which cost upwards of $40-$100/mo. These owners often resort to manually texting customers ETAs and using Google Maps on their phones to string together multi-stop routes. This causes delayed deliveries, fragmented communication, and high cognitive load while driving.

  ## Research Report
  **Competitor Systems Audit:**
  - **Shopify & Wix:** Local delivery is treated as an afterthought. Users must install expensive plugins (e.g., Zapiet, Onfleet) to get route optimization and driver tracking.
  - **DoorDash/UberEats:** While they handle the logistics, they take 20-30% commissions, destroying SMB margins.
  - **Circuit / Route4Me:** Good standalone apps for routing, but completely disconnected from the underlying sales ledger and CRM.

  **Gaps Identified:**
  OHC lacks a native, zero-configuration local delivery module that integrates directly with the omnichannel ledger. We need a system where the "Operations Agent" can automatically calculate the most efficient route for daily deliveries, dispatch ETAs to customers via the "Customer Success Agent", and provide the owner with a seamless, 375px mobile-first driver view.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD;
      subgraph Mobile Device
          App[OHC Mobile App 375px] --> DeliveryUI[Driver Route UI];
          DeliveryUI --> LocalDB[(Local Cache CRDT)];
      end

      App -- "Sync Route/Status" --> Gateway[OHC API Gateway];

      Gateway --> RouteEngine[Routing & Dispatch Engine];
      RouteEngine --> MainDB[(Cloud Postgres Ledger)];
      RouteEngine --> MapProvider[Google/OSM Maps API];

      Gateway --> Agents[AI Agent Swarm];

      subgraph Agent Departments
          Agents --> OpsAgent[Ops: Optimize Routes & Batching];
          Agents --> CSAgent[Customer Success: Live ETA SMS/WhatsApp];
          Agents --> FinanceAgent[Finance: Delivery Fee Reconcile];
      end
  ```

  ### Mobile UX Flow (375px First)
  1. **Daily Dispatch:** Maya opens the OHC app. The Agent Feed presents a card: "You have 5 local deliveries today. The Operations Agent has optimized your route to save 12 miles. [Start Route]"
  2. **Driver Mode:** Tapping "Start Route" enters a specialized, high-contrast driver view. Touch targets are large (minimum 64x64px for easy tapping while parked). It shows the current stop, order details (e.g., "Fragile - Wedding Cake"), and a one-tap button to open native navigation (Apple Maps/Google Maps).
  3. **Automated Communication:** As Maya marks a stop as "Delivered" (or as her GPS nears the next stop), the CS Agent autonomously texts the next customer: "Hi, Maya is 10 minutes away with your order!"
  4. **Proof of Delivery:** Maya can optionally snap a photo of the cake on the porch, which is instantly synced via the CRDT to the unified ledger and texted to the customer.

  ### AI Agent Integration Points
  - **Operations Agent:** Monitors incoming orders, groups them by geographic proximity, and calls external mapping APIs to generate the TSP (Traveling Salesperson) optimized route.
  - **Customer Success (CS) Agent:** Triggers SMS/WhatsApp updates for "Out for Delivery", ETAs, and "Delivered" with photo proof.
  - **Finance Agent:** Automatically calculates distance-based delivery fees during checkout and attributes them accurately in the ledger.

  ### Key Design Decisions & Security
  - **Offline-First Delivery:** The Driver UI must operate flawlessly offline using CRDTs. Drivers often lose service in apartment complex elevators or rural areas. Status updates sync when the connection is restored.
  - **Zero-Trust Delivery Hand-off:** For businesses (like Nora's agency) that use contractors, the delivery manifest is secured via SPIFFE SVIDs, ensuring the contractor only sees data for their assigned route, not the entire customer DB.

  ## Implementation Prompt
  Implement the Autonomous Embedded Local Delivery & Route Optimization Engine.
  - **User-Facing Outcome:** The business owner receives an auto-generated, optimized daily delivery route and a dedicated mobile "Driver Mode" interface, while customers receive autonomous ETA updates.
  - **CUJ:** An owner accepts 3 local orders. The app generates the optimal route. The owner enters Driver Mode, marks the first order as delivered, and the system autonomously texts the second customer their ETA.
  - **Acceptance Criteria:**
    - Develop the Driver Mode UI ensuring strict 375px mobile compatibility and large touch targets.
    - Implement background route optimization logic (TSP) triggered by the Ops Agent.
    - Support offline capabilities (CRDT syncing) for the Driver Mode.
    - Integrate the CS Agent to handle ETA notifications automatically.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, architecture, mobile-first]
assignees: []
