issue_title: "Dynamic Geo-Spatial Storefront & Routing Mesh"
issue_description: |
  # Title
  Dynamic Geo-Spatial Storefront & Routing Mesh

  # Problem Statement
  Small business owners who operate on the move face a unique challenge that static e-commerce platforms ignore. Fatima runs a highly popular food cart, but she moves between three different business parks depending on the day of the week. Her customers need to know *exactly* where she is right now to place a pre-order for pickup, and she needs to stop accepting orders if she's driving. Carlos, a handyman, travels to his clients, but his service area changes dynamically based on his current job's location. If a platform assumes a business has a single, static address, Fatima loses pre-orders and Carlos gets booked for jobs that are too far away. They need a storefront that dynamically updates their location, calculates distance or delivery/service radii in real-time, and routes orders intelligently without them ever touching a settings menu.

  # Research Report
  **Competitive Analysis:**
  - **Shopify / Wix / Squarespace:** These platforms are built around static physical addresses for shipping origins or static store locations for pickup. They offer plugins for local delivery, but these usually rely on static zip codes or fixed radii from a single location, rather than real-time GPS coordinates of the merchant's phone.
  - **Square:** Square POS is excellent for physical sales, but its online pickup ordering is tied to fixed location profiles. A food truck operator has to manually update their active location profile, which is error-prone.
  - **UberEats / DoorDash:** Masterclass in real-time geo-spatial routing, but these are aggregators that take 30% cuts, not owned-and-operated business platforms.

  **Market Gap:**
  There is no "storefront in your pocket" that natively broadcasts the merchant's real-time mobile location (when enabled) to power dynamic pickup coordinates, smart service-area availability, and dynamic travel-time quoting for service bookings. By integrating geo-spatial awareness deeply into the OHC platform, we can empower a massive segment of mobile micro-businesses.

  # Design Doc

  ## Architecture Diagram
  ```mermaid
  graph TD;
      subgraph Mobile Device
          App[OHC Mobile App 375px] --> GeoTracker[Native OS Location Services];
          GeoTracker --> LocalDB[(Local Cache & State)];
      end

      App -- "Periodic Geo-Ping" --> Gateway[OHC API Gateway];

      Gateway --> SpatialMesh[Geo-Spatial Storefront Mesh];
      SpatialMesh --> MainDB[(Cloud Postgres + PostGIS Ledger)];
      SpatialMesh --> EdgeCache[CDN / Edge Storefront Cache];

      Gateway --> Agents[AI Agent Swarm];

      subgraph Agent Departments
          Agents --> OpsAgent[Ops: Update Availability Radius];
          Agents --> SalesAgent[Sales: Dynamic Quote Travel Time];
          Agents --> MarketingAgent[Marketing: Proximity Push Notifications];
      end
  ```

  ## Mobile UX Flow (375px First)
  1. **Status Toggle:** On the OHC home dashboard, Fatima sees a beautiful Translucent Glass card at the top. It has a single toggle: "Accepting Orders".
  2. **Location Broadcast:** When toggled on, the app asks (once) for "Allow Location Access While Using App". Once granted, a subtle radar animation pulses on the card, indicating her current street location is live on her storefront.
  3. **Customer View:** A customer opens Fatima's OHC storefront link. Instead of a static address, they see a beautiful, branded map card showing "Fatima's Cart is currently at 4th & Pike. Open for 3 more hours."
  4. **Service Radius (Carlos):** For Carlos, the app tracks his current job site. When a new customer requests a quote, the Sales Agent calculates the travel time from Carlos's *current or next predicted location*, dynamically pricing the travel fee and proposing accurate time slots.

  ## AI Agent Integration Points
  - **Operations Agent:** Monitors the merchant's movement. If the GPS speed indicates driving, it automatically pauses new immediate-pickup orders to prevent customer frustration.
  - **Sales Agent:** Uses the real-time location to calculate dynamic travel fees and feasible booking slots for service businesses (e.g., Carlos the handyman).
  - **Marketing Agent:** Can automatically send a notification or SMS to highly engaged local customers when Fatima's cart sets up within 1 mile of their usual location.

  ## Key Design Decisions & Security
  - **Zero Trust & Security:** Location data processing and edge-cache invalidation are strictly isolated per tenant using SPIFFE SVIDs, guaranteeing that Fatima's location data cannot leak to another merchant's storefront.
  - **Battery & Privacy Conscious:** The app only broadcasts location when the business is "Active" or "On Shift". We rely on OS-level geofencing and significant-change APIs rather than constant GPS polling to save the merchant's battery.
  - **Edge-Cached Locations:** Storefront maps must load instantly. The Spatial Mesh pushes the merchant's current coordinates to edge nodes so the customer UI renders immediately without hitting the central database.
  - **Zero-Config Zones:** No drawing polygons on maps. The system infers service areas and travel limits based on natural language inputs like "I don't drive more than 30 minutes between jobs."

  # Implementation Prompt
  **To Implementer Agent:**
  Implement the Dynamic Geo-Spatial Storefront & Routing Mesh.
  - **User-Facing Outcome:** Mobile merchants (food carts, mobile services) can toggle their "Live Location" on their dashboard. Their public storefront instantly updates to show their current physical location for pickup, or uses it to calculate accurate travel times and availability for service bookings.
  - **CUJ (Critical User Journey):**
    1. Merchant toggles "Accepting Orders/Live" on the mobile app.
    2. The app captures and securely transmits the device's location to the backend.
    3. The merchant's public storefront immediately displays the updated location and recalculates pickup/travel logistics.
    4. The AI Sales Agent uses this real-time location to accurately quote a customer requesting an immediate service booking.
  - **Acceptance Criteria:**
    - The UI must feature a clear, one-tap "Go Live" toggle on the 375px dashboard utilizing the glassmorphism design system.
    - The backend must efficiently process location updates and invalidate the edge-cached storefront.
    - Integration with the Sales/Ops agents to dynamically adjust availability or quote pricing based on real-time distance.
    - No complex mapping configurations or polygon-drawing tools should be exposed to the user.

  **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
