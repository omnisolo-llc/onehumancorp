issue_title: "Implement Centralized Inventory & Distributed POS Architecture"
issue_description: |
  # Centralized Inventory & Distributed POS Architecture

  ## Problem Statement
  Small business owners like Priya (boutique owner) require seamless inventory tracking between online (web/mobile) and in-store operations (tap-to-pay or card reader). Currently, OHC lacks a real-time, strongly consistent inventory locking and caching mechanism, as well as a robust distributed sync protocol for hybrid merchants. Without this, double-booking and out-of-stock scenarios occur during simultaneous online and offline purchases.

  ## Research Findings
  Competitors like Shopify dominate the e-commerce space with extensive POS capabilities but often fail micro-SMEs due to complexity. Their inventory management can be disjointed—online inventory frequently falls out-of-sync with in-person sales unless costly third-party integration tools or higher-tier plans are employed. Square and Stripe Terminal provide robust POS hardware but lack the integrated, agentic workflow automation needed to unify the business operations effortlessly.

  ## Architectural Design
  ### Data Model & Sync Protocol
  - **Central Ledger (PostgreSQL):** The ultimate source of truth for all inventory counts. We utilize row-level locking or optimistic concurrency control for critical updates.
  - **Distributed Locks (Redis Redlock):** A temporary inventory reservation system applied during the checkout process to prevent double-booking. The lock duration is dynamically tuned (e.g., 5 minutes for online carts vs. 15 seconds for rapid tap-to-pay transactions). Lock key pattern: `ohc:lock:{tenant_id}:inventory:{product_id}`.
  - **Offline/Local First POS Client:** The mobile POS client caches catalog data locally. It employs an eventual consistency mechanism to sync finalized offline sales and reconcile with the central ledger asynchronously when the network is restored.

  ### System Overview Diagram

  ```mermaid
  graph TD
      subgraph Offline "POS Device (Mobile)"
          POSApp[Mobile POS App]
          LocalDB[(Local SQLite / Cache)]
      end

      subgraph Edge "Network Layer"
          Gateway[API Gateway]
      end

      subgraph Backend "Rust Backend"
          InventoryService[Inventory Service]
          SyncManager[Eventual Sync Manager]
      end

      subgraph Storage
          Redis[(Redis Distributed Locks)]
          PG[(PostgreSQL Central Ledger)]
      end

      subgraph AI "Agent Workforce"
          OpsAgent[Operations Agent: The Manager]
          CSAgent[Customer Success Agent: The Ambassador]
      end

      POSApp -->|Read| LocalDB
      POSApp -->|Checkout Attempt| Gateway
      POSApp -.->|Offline Sync| SyncManager

      Gateway --> InventoryService
      InventoryService --> Redis
      Redis -- "Acquire Lock" --> PG

      InventoryService --> SyncManager
      SyncManager --> PG

      PG --> OpsAgent
      OpsAgent -.->|Low Stock Alert| CSAgent
  ```

  ### Entity-Relationship Diagram

  ```mermaid
  erDiagram
      TENANT ||--o{ PRODUCT : owns
      PRODUCT ||--o{ INVENTORY_LEDGER : tracks
      PRODUCT ||--o{ INVENTORY_LOCK : holds

      PRODUCT {
          uuid id PK
          uuid tenant_id FK
          string name
          decimal price
          boolean is_active
      }

      INVENTORY_LEDGER {
          uuid id PK
          uuid product_id FK
          int available_quantity
          int reserved_quantity
          datetime last_updated
      }

      INVENTORY_LOCK {
          string lock_key PK
          uuid product_id FK
          uuid session_id
          datetime expires_at
      }
  ```

  ### Sequence Diagram: Distributed Lock Checkout

  ```mermaid
  sequenceDiagram
      participant C as POS Client / Web
      participant API as OHC API
      participant R as Redis
      participant DB as PostgreSQL
      participant A as Ops Agent

      C->>API: Initiate Checkout (Product A)
      API->>R: SETNX ohc:lock:tenant_id:inventory:product_A
      alt Lock Acquired
          R-->>API: Success (Lock TTL 15s)
          API->>DB: UPDATE Inventory SET reserved = reserved + 1
          DB-->>API: Row Locked & Updated
          API-->>C: Proceed to Payment
      else Lock Failed (In use)
          R-->>API: Fail
          API-->>C: Item unavailable right now
      end
  ```

  ### AI Agent Coordination
  - **Operations Agent ("The Manager"):** Actively monitors stock levels across all channels. It tracks incoming orders, triggers low-stock alerts, coordinates with the sync mechanism to reconcile conflicts, and suggests restock plans.
  - **Finance Agent ("The Accountant"):** Processes splits for Terminal transactions and correlates POS data with online purchases for unified financial reporting.
  - **Customer Success Agent ("The Ambassador"):** Automatically updates online storefront availability and notifies customers if an item in their cart becomes unavailable due to an in-store purchase.

  ### Mobile UX Flow (375px)
  - **Storefront / Catalog View:** A clean, image-led grid layout where tapping an item adds it to the cart instantly. Status tokens clearly indicate "Low Stock" or "Sold Out."
  - **Checkout Flow:** Large, easily tappable buttons (≥ 44x44px). The payment process utilizes the native mobile keyboard for manual entry or delegates to tap-to-pay functionality.
  - **Optimistic UI:** When the user taps 'Add', the UI updates immediately. If the Redis reservation fails (due to another concurrent purchase), the item gracefully shakes, turns red, and a toast notification explains "Just sold out!".
  - **Glassmorphism Elements:** Used for sticky 'Cart Total' action bars floating at the bottom of the screen.

  ## Implementation Prompt
  **Feature:** Centralized Inventory & Distributed POS Architecture
  **Target Persona:** Priya (Boutique Owner)
  **User Story:** As Priya, I want my inventory to be automatically synced across my online store and in-person POS so I never accidentally sell something that's out of stock.

  **Acceptance Criteria:**
  1. Implement row-level locking / OCC in the PostgreSQL central ledger for inventory counts.
  2. Implement Redis Redlock-based distributed locks during the checkout and POS reservation process to prevent double-booking.
  3. Integrate the Operations Agent to trigger low-stock alerts and suggest restocks.
  4. Integrate the Customer Success Agent to handle out-of-stock notifications during active checkouts if a lock fails or is preempted by a POS sale.
  5. Provide Playwright E2E tests validating the checkout flow and inventory locking, simulating simultaneous online and offline purchase attempts.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
