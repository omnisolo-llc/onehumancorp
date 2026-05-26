issue_title: "[Architecture] Autonomous Multi-Location Expansion Engine"
issue_description: |
  # [Architecture] Autonomous Multi-Location Expansion Engine

  ## Title
  Autonomous Multi-Location Expansion Engine: The Zero-Friction Growth Mesh

  ## Problem Statement
  Scaling a micro-business from one location to two (e.g., adding a second food cart, opening a pop-up shop, or teaching at multiple studios) is the most dangerous transition for a small business owner. It breaks their single-location mental models. Suddenly, inventory is split, staff scheduling becomes a routing problem, and consolidated reporting requires complex spreadsheets.

  For our personas:
  - **Priya (Boutique Owner):** Wants to open a seasonal pop-up at a holiday market but is terrified of overselling inventory that physically exists in her main store versus the pop-up.
  - **Fatima (Food Cart):** Wants to launch a second cart but cannot physically be at both places to manage cash, stock levels, and order routing.
  - **Leo (Music Tutor):** Teaches at two different music studios and online, causing calendar conflicts and confusing students about where to go.

  Competitors (Shopify, Square, Wix) treat "Multi-Location" as an enterprise feature. They require expensive plan upgrades and present complex "Location Mapping" dashboards that fail the Grandmother Test. OneHumanCorp (OHC) needs an invisible multi-location mesh where adding a new location is as simple as tapping "Add Pop-up", and the AI Operations Agent automatically splits inventory, updates the booking calendar dynamically, and routes orders to the correct fulfillment node without manual configuration.

  ## Research Report
  ### Competitive Landscape
  *   **Shopify POS / Admin:** Multi-location is supported but complex. The user has to manually assign inventory to locations, set up location-specific routing rules, and often requires third-party apps for complex store pickup logic.
  *   **Square:** Strong multi-location support, but the UI is disjointed. Switching between locations on the POS app often requires logging out or navigating deep menus.
  *   **Wix:** Limited native support for true multi-location inventory synchronization across physical and digital channels.
  *   **Legacy ERPs:** Overkill. Way too complex and expensive for a 2-5 location micro-business.

  ### Strategic Opportunity for OHC
  OHC will abstract the concept of "Locations" into "Nodes" on a graph. A Node can be a physical store, a temporary pop-up, a food cart, or an online digital channel. By leveraging the **Operations Agent** and the **HR/Staffing Agent**, OHC can proactively suggest inventory transfers, alert staff of location assignments, and dynamically route customer pickups based on real-time geolocation, completely invisibly.

  ## Design Doc

  ### High-Level Architecture
  - **Location Graph Data Model:** A strictly isolated, multi-tenant ledger where Inventory, Staff, and Orders are fundamentally tied to a `Node_ID`.
  - **Intelligent Order Routing Engine:** Evaluates customer proximity, inventory availability, and fulfillment capacity to route an order to the optimal Node.
  - **Agentic Rebalancing:** The Operations Agent monitors stock velocity across Nodes and autonomously prompts the owner to transfer items ("Pop-up A is selling out of Vegan Cakes faster than Main Store. Should I queue an inventory transfer?").

  ### Architecture Diagram

  ```mermaid
  erDiagram
      TENANT ||--o{ LOCATION_NODE : owns
      LOCATION_NODE {
          string node_id
          string type
          string geo_location
          boolean is_active
      }
      LOCATION_NODE ||--o{ INVENTORY_LEDGER : holds
      LOCATION_NODE ||--o{ STAFF_SHIFT : schedules
      LOCATION_NODE ||--o{ ORDER_FULFILLMENT : processes

      INVENTORY_LEDGER {
          int available_qty
          int reserved_qty
      }
  ```

  ```mermaid
  sequenceDiagram
      participant Customer
      participant Mobile UI (OHC)
      participant KAIROS Orchestrator
      participant Operations Agent
      participant Location Node A
      participant Location Node B

      Customer->>Mobile UI (OHC): Views Priya's Boutique (Online)
      Mobile UI (OHC)->>KAIROS Orchestrator: Request Product Availability
      KAIROS Orchestrator->>Operations Agent: Where is this item?
      Operations Agent-->>KAIROS Orchestrator: 2 at Node A (Main), 0 at Node B (Pop-up)
      KAIROS Orchestrator-->>Mobile UI (OHC): Display "Available at Main Store for Pickup"
      Customer->>Mobile UI (OHC): Places Order for Pickup at Main Store
      Mobile UI (OHC)->>KAIROS Orchestrator: Route Order
      KAIROS Orchestrator->>Location Node A: Drop order to KDS / Thermal Printer
      Operations Agent->>KAIROS Orchestrator: Detect low stock at Node A.
      KAIROS Orchestrator->>Priya's Mobile: Push: "Stock low at Main Store. Tap to reorder."
  ```

  ### UI Wireframes & Mobile UX Flow (375px)
  1.  **The "Add Location" Flow:**
      *   **Screen 1:** A simple floating action button (+) on the main dashboard -> "New Location/Pop-up".
      *   **Screen 2:** "What kind of location?" (Permanent Store, Temporary Pop-up, Food Truck).
      *   **Screen 3:** "Where is it?" (Use current location or type address).
      *   **Screen 4:** "Done. We've duplicated your catalog. Tap to adjust starting inventory for the new spot."
  2.  **The "Location Switcher" (Translucent Glass UI):**
      *   Instead of a deep settings menu, the top app bar features a pill-shaped dropdown. Tapping it slides up a macOS-style translucent bottom sheet showing live revenue and active staff at each location, allowing 1-tap switching of the dashboard context.
  3.  **Operations Agent Intervention Card:**
      *   A prominent UniFi-style dashboard card: "⚠️ **Inventory Imbalance:** Red Dresses are selling 3x faster at the Holiday Pop-up. [Transfer 10 from Main Store]".

  ### AI Agent Integration Points
  *   **Operations Agent:** Monitors cross-location inventory and suggests transfers.
  *   **CS Agent (Inbox):** Intercepts messages like "Are you at the farmer's market today?" and auto-replies based on the Location Graph's active schedule.
  *   **HR Agent:** Prevents double-booking a staff member across two distant locations on the same day.

  ### Zero Trust & Security Targets
  *   **Multi-Tenant Isolation:** `tenant_id` and `node_id` must be composite keys in the database.
  *   **Offline Mode Parity:** If the Pop-up loses internet, its local offline database queue accepts Tap-to-Pay transactions and syncs the inventory decrement to the main cloud graph when reconnected, utilizing CRDTs (Conflict-Free Replicated Data Types) to prevent phantom stock.

  ## Implementation Prompt
  **To the Implementer Swarm:**
  Your mission is to implement the underlying Multi-Location Expansion Engine.
  - **Outcome:** A small business owner must be able to spin up a secondary "Pop-up" location in the mobile app in under 30 seconds, with their catalog instantly available.
  - **CUJ (Critical User Journey):** Priya taps "Add Pop-up", selects her current GPS location, and immediately her online storefront updates to show two pickup options for local customers.
  - **Acceptance Criteria:**
    1. The schema must support N locations per tenant.
    2. Inventory ledgers must accurately track quantities per location without race conditions (use atomic decrements).
    3. The mobile API must expose a seamless way to query aggregate inventory AND location-specific inventory.
    4. Ensure CRDT or robust conflict resolution is designed for offline Tap-to-Pay sales at remote locations syncing back to the primary ledger.
  - Do NOT prescribe specific DB choices or API frameworks—implement using the existing OHC stack. Ensure it passes the Grandmother Test (no complex routing rules in the UI).

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
