issue_title: "Architecture: Autonomous Multi-Location & Pop-Up Routing Mesh"
issue_description: |
  # [Architecture] Autonomous Multi-Location & Pop-Up Routing Mesh

  ## Title
  Autonomous Multi-Location & Pop-Up Routing Mesh

  ## Problem Statement
  Small business owners often scale from a single operation to multiple locations or temporary pop-ups. Maya (baker) operates her main kitchen but does weekend pop-ups at farmer's markets. Fatima (food cart) operates her primary cart but sometimes runs a second cart for festivals. Managing inventory, orders, and staff routing across these dynamic, sometimes temporary, locations is chaotic. Existing platforms assume static retail locations or complex warehouse routing. They force users to create "warehouses" or "new stores" for a simple 3-day pop-up, splitting inventory manually and risking double-selling. There is no simple, mobile-first way to spin up a temporary location, route specific inventory to it, localized pre-orders to the right pickup spot, and consolidate reporting seamlessly.

  ## Research Report

  ### Competitive Analysis

  | Platform | Multi-Location Capabilities | Strengths | Weaknesses (The OHC Opportunity) |
  |---|---|---|---|
  | Shopify | Locations & Inventory Routing | Robust logic for warehouses | Assumes permanent brick-and-mortar or warehouses. Complex setup. Not meant for 2-hour pop-ups. |
  | Square | Locations | Good for permanent stores | Rigid. Hard to share real-time inventory dynamically between a main store and a mobile cart without manual transfers. |
  | Wix | Multiple Locations | Basic support | Designed for static addresses, lacks dynamic geo-fenced pop-up features. |
  | **OHC (Target)** | **Dynamic Multi-Location Mesh** | **Zero-config pop-ups, geo-aware routing, shared or isolated inventory pools** | **Must be 1-tap to "start pop-up" from mobile.** |

  ### Key Architectural Findings
  Current eCommerce architectures bind orders and inventory rigidly to static Location IDs. To support the fluid nature of modern SMBs (food trucks, pop-up shops, farmer's markets), the system needs a more flexible "Event/Pop-Up" overlay on top of the traditional location model. This requires dynamic routing rules where an active pop-up can temporarily claim a subset of inventory or share a pool with the main hub, while dynamically updating the storefront to offer localized pickup options based on the customer's proximity or selected event.

  ## Design Doc

  ### Architecture Diagram

  ```mermaid
  erDiagram
      MERCHANT ||--o{ LOCATION : has
      LOCATION {
          string id
          string name
          boolean is_permanent
          geopoint coordinates
      }
      LOCATION ||--o{ INVENTORY_POOL : manages
      MERCHANT ||--o{ POP_UP_EVENT : creates
      POP_UP_EVENT {
          string id
          string name
          datetime start_time
          datetime end_time
          geopoint location
      }
      POP_UP_EVENT ||--o{ LOCATION : temporarily_acts_as
      INVENTORY_POOL ||--o{ INVENTORY_ITEM : contains
      ORDER ||--o{ POP_UP_EVENT : routed_to
      ORDER ||--o{ LOCATION : routed_to
      AI_OPERATIONS_AGENT ||--o{ POP_UP_EVENT : manages_routing
  ```

  ### Mobile UX Flow
  1. **Start Pop-Up (1-Tap):** Maya opens the OHC app, taps "Start Pop-Up".
  2. **Details:** She names it "Sunday Farmer's Market", sets the duration, and selects "Share main inventory" or "Allocate specific items".
  3. **Auto-Update Storefront:** The AI Operations Agent immediately updates her online storefront. Customers now see "Pickup at Sunday Farmer's Market" as an option for orders placed today.
  4. **Geo-Awareness:** Customers nearby get a notification or see the pop-up location prioritized.
  5. **End Pop-Up:** When the event ends, the AI automatically reconciles inventory back to the main pool and removes the pickup option from the storefront.

  ### Zero Trust & Security
  - Multi-tenant isolation ensures pop-up data (orders, inventory) is strictly bound to the merchant's tenant ID.
  - Staff assigned to a pop-up location only receive access tokens valid for that specific location's POS and inventory during the event window.

  ### Performance & Offline Targets
  - Pop-up creation must sync locally first (IndexedDB) and push to the cloud queue, allowing merchants to start a pop-up even with poor cellular reception at the event.
  - Inventory decrements must be highly available via edge nodes to prevent double-selling across the main store and pop-up.

  ## Implementation Prompt
  **User-Facing Outcome:** A merchant can instantly create a temporary pop-up location from their mobile device. This automatically updates their online storefront to offer localized pickup and manages inventory routing without manual warehouse configuration.
  **CUJ:**
  1. Merchant taps "Start Pop-Up" on mobile dashboard.
  2. Selects inventory mode (Shared vs. Allocated).
  3. System automatically adds a new pickup location option to the storefront.
  4. Orders routed to the pop-up appear in a localized queue.
  5. Merchant ends pop-up, system auto-reconciles inventory.
  **Acceptance Criteria:**
  - Backend supports dynamic creation of temporary locations with TTL (Time-To-Live).
  - Inventory engine can handle shared pools or temporary localized allocation.
  - Online storefront dynamically fetches active pickup locations based on current pop-up events.
  - AI Agent can automate the teardown and reconciliation process when the event ends.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
