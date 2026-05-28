issue_title: "Implement Autonomous Multi-Location and Franchise Mesh"
issue_description: |
  # [Architecture] Autonomous Multi-Location and Franchise Mesh

  ## Problem Statement

  When our core users—like **Priya (boutique owner)** or **Fatima (food cart operator)**—expand their successful businesses to a second or third location, they hit a wall. Managing inventory, staff, and pricing across multiple physical sites turns their simple operation into a complex logistical nightmare. They are forced to either duplicate their entire setup in a new account (losing aggregated analytics and unified customer data) or manually reconcile sales and stock at the end of every day using paper or spreadsheets. A small business owner shouldn't need an Enterprise Resource Planning (ERP) system to open a second food cart or a pop-up shop.

  ## Research Report

  Currently, small business platforms handle multi-location scaling poorly, often treating it as an "enterprise" feature:

  *   **Square / Toast Multi-Location:** Require complex backend dashboard configuration. Owners have to manually map inventory items to specific locations, configure location-specific pricing matrices, and train staff to log into the correct location terminal. It is highly manual and error-prone.
  *   **Shopify POS:** Supports multi-location inventory, but transferring stock between locations requires manual entry in the desktop admin dashboard. It lacks proactive, AI-driven stock balancing between local stores.
  *   **The OHC Differentiator:** OneHumanCorp will introduce the **Autonomous Multi-Location Mesh**. Instead of the user managing locations, the AI Departments handle them. If Priya opens a second boutique, the system natively understands that "Boutique A" and "Boutique B" share a central product catalog but maintain separate physical ledger balances. The AI Operations Agent will automatically suggest inventory transfers between locations when one is running low and the other is overstocked, requiring only a "1-Tap Approval" from the business owner on their phone.

  ## Design Doc

  ### Architecture Diagram

  ```mermaid
  erDiagram
      TENANT ||--o{ LOCATION : operates
      LOCATION ||--o{ INVENTORY_LEDGER : tracks_stock_at
      LOCATION ||--o{ STAFF_SHIFT : scheduled_at
      TENANT ||--o{ MASTER_CATALOG : owns
      MASTER_CATALOG ||--o{ PRODUCT : contains
      PRODUCT ||--o{ INVENTORY_LEDGER : linked_to

      LOCATION {
          string location_id
          string address
          string tax_jurisdiction
      }

      INVENTORY_LEDGER {
          int available_qty
          int reserved_qty
      }
  ```

  ```mermaid
  sequenceDiagram
      participant P as Priya (Owner Mobile)
      participant O as AI Operations Agent
      participant L1 as Location 1 (Main Store)
      participant L2 as Location 2 (Pop-up)

      O->>O: Detect Low Stock at Location 2
      O->>O: Detect Excess Stock at Location 1
      O->>P: Push Notification: "Transfer 10 Linen Dresses from Main to Pop-up?"
      P->>O: 1-Tap Approve
      O->>L1: Issue Pick Task to Location 1 Staff
      L1-->>O: Staff Confirms Pick (Transit State)
      O->>L2: Issue Receive Task to Location 2 Staff
      L2-->>O: Staff Confirms Receipt
      O->>P: "Transfer Complete. Pop-up is fully stocked."
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)

  **Visual Language:** macOS-style Translucent Glass materials, Ubiquiti UniFi modular dashboard cards. No complex grids.

  1.  **The Hub View (Aggregated Analytics):**
      *   When the owner opens the app, they see a unified "Total Sales" card.
      *   A smooth, horizontal carousel below it shows "Locations" (e.g., Main Street, Downtown Cart). Swiping and tapping a card filters the entire app's context to that specific location smoothly.
  2.  **Autonomous Transfer Alert (The Magic Interaction):**
      *   A rich notification appears: *"Downtown Cart is running out of Halal Chicken. Main Cart has 50 extra portions. Transfer 20 portions?"*
      *   Buttons: [Transfer] (Primary, solid tint) / [Dismiss] (Ghost).
  3.  **Staff View (Context-Aware):**
      *   When staff (e.g., Carlos's apprentice) logs in, their app is geofenced. It automatically knows which location they are standing in and only shows them the terminal/tasks for that specific location. No "Select Location" drop-downs required.

  ### AI Agent Integration Points

  *   **Operations Agent:** Constantly runs background anomaly detection on inventory levels across all locations. Predicts stockouts at specific locations and suggests transfers before they happen.
  *   **Finance Agent:** Automatically routes payouts to the correct bank accounts if locations are franchised or have different LLPs, and handles location-specific tax compliance invisibly.
  *   **Customer Support Agent:** If a customer messages "Is this available in-store?", the agent checks the unified ledger and replies, "Yes! We have 2 left at the Downtown location. Shall I hold one for you?"

  ### Key Design Decisions

  *   **Unified Master Catalog, Decentralized Ledgers:** Users manage products once. The system automatically creates isolated inventory ledgers for each new location they add.
  *   **Zero-Trust Isolation by Geolocation:** Staff terminals are authenticated not just by login, but by physical proximity to the location coordinates, removing the risk of ringing up sales at the wrong store.
  *   **"Grandmother Test" Compliance:** All multi-location complexity (tax jurisdictions, stock transfers, unified vs. separate routing) is hidden. The user just sees their locations as separate cards they can tap into.

  ## Implementation Prompt

  **To the Engineering Swarm:**
  Your objective is to implement the foundational `Autonomous Multi-Location Mesh` capabilities for OneHumanCorp.

  **Core User Journey (CUJ):**
  A user with an existing successful business taps "Add New Location". The system instantly provisions a new location node, inheriting the Master Catalog. The user can then assign staff to this location. The AI Operations Agent must begin monitoring stock across both locations and push a "1-Tap Transfer" suggestion to the owner when an imbalance occurs.

  **Acceptance Criteria:**
  1. A Tenant can possess multiple Locations.
  2. The Master Catalog remains unified, but inventory counts are strictly ledgered per Location.
  3. The UI must render a unified dashboard that can filter seamlessly by Location via a mobile-first card carousel (375px viewport optimized).
  4. The Operations Agent must successfully detect an inventory imbalance between two locations and generate a transfer proposal.
  5. Do not implement complex enterprise settings screens; all configuration must be handled via conversational onboarding or one-tap approvals.
  6. Ensure Zero-Trust multi-tenant isolation rules are enforced across all location entities.

  **Priority:** P1
  **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
