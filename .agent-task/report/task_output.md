issue_title: "[Architecture] High-Concurrency Flash Sale & Drop Mechanics Engine"
issue_description: |
  ## Title
  [Architecture] High-Concurrency Flash Sale & Drop Mechanics Engine

  ## Problem Statement
  For modern creators and specific verticals (e.g., streetwear, limited-edition art, hyped food drops), standard eCommerce infrastructure fails. Creators often run "drops" where limited inventory sells out in minutes. When traffic spikes 1000x normal load, standard relational databases buckle under the write contention (race conditions), leading to overselling or site crashes. When a platform crashes during a drop, the business owner loses credibility and revenue. Existing platforms like Shopify offer high-scale infrastructure but often require upgrading to extremely expensive enterprise tiers (Shopify Plus) to handle massive drops reliably, or rely on complex queueing mechanisms that frustrate buyers. OHC needs a robust, high-performance architecture specifically designed to handle extreme burst traffic and ensure zero-overselling, natively built into the platform so a creator can run a drop with confidence without needing an enterprise plan.

  ## Research Report
  - **Competitive Analysis:**
    - **Shopify Plus:** Uses a complex queuing system and robust backend to handle flash sales. Very reliable but locked behind a high paywall ($2000+/mo). Regular Shopify plans can struggle or introduce queues that ruin the buyer experience.
    - **Stripe:** Exceptional at handling high TPS (transactions per second), but leaves the inventory locking mechanism to the merchant's application layer.
    - **Custom Architectures:** Usually involve Redis for atomic decrements, Kafka for event streaming, and asynchronous payment processing.
  - **Key Findings:**
    - **Race Conditions:** The primary failure point is the inventory decrement. If two users buy the last item simultaneously, a poorly designed system might process both, leading to overselling.
    - **Read vs. Write Load:** During a drop, read load (users refreshing the page) is massive. Write load (checkouts) is also massive but brief.
    - **Optimistic Locking:** Essential for preventing race conditions without massive performance hits.

  ## Design Doc
  ### Data Model (Drop Mechanics & High-Concurrency Inventory)
  We utilize an architecture that separates the read-heavy catalog from the write-heavy inventory reservation system, utilizing an in-memory datastore (like Redis) for atomic inventory operations.

  ```mermaid
  erDiagram
      TENANT ||--o{ DROP_EVENT : "hosts"
      DROP_EVENT ||--o{ DROP_INVENTORY : "allocates"
      DROP_EVENT {
          uuid id
          string name
          timestamp starts_at
          timestamp ends_at
          string status "Scheduled, Active, SoldOut, Ended"
      }
      DROP_INVENTORY {
          uuid id
          uuid product_id
          int allocated_quantity
          int available_quantity
      }
      DROP_INVENTORY ||--o{ RESERVATION_TICKET : "issues"
      RESERVATION_TICKET {
          uuid id
          uuid session_id
          timestamp expires_at
          string status "Pending, Confirmed, Released"
      }
  ```

  ### System Architecture
  ```mermaid
  sequenceDiagram
      participant Mobile (Buyer)
      participant Edge Cache (Cloudflare)
      participant OHC API Gateway
      participant Inventory Service (Redis)
      participant Payment Gateway
      participant OHC Ledger (Postgres)

      Mobile (Buyer)->>Edge Cache: Request Drop Page
      Edge Cache-->>Mobile (Buyer): Serve Static Assets & Cached State
      Mobile (Buyer)->>OHC API Gateway: Request Checkout (Item: X)
      OHC API Gateway->>Inventory Service: Atomic Decrement & Issue Reservation Ticket (TTL 5 mins)
      alt Inventory Available
          Inventory Service-->>OHC API Gateway: Ticket Issued
          OHC API Gateway-->>Mobile (Buyer): Checkout Granted (5 mins to pay)
          Mobile (Buyer)->>Payment Gateway: Process Payment
          Payment Gateway->>OHC Ledger: Payment Success Webhook
          OHC Ledger->>Inventory Service: Confirm Ticket (Remove TTL)
          OHC Ledger->>OHC Ledger: Record Final Order
      else Inventory Exhausted
          Inventory Service-->>OHC API Gateway: Sold Out
          OHC API Gateway-->>Mobile (Buyer): Drop Sold Out
      end
  ```

  ### Key Architectural Invariants
  1. **Atomic Reservations:** Inventory decrements MUST be atomic and handled in a high-throughput, low-latency datastore (e.g., Redis via LUA script) before any checkout screen is rendered.
  2. **Reservation TTL:** A reservation ticket is strictly time-bound. If payment is not completed within the TTL (e.g., 5 minutes), the ticket is automatically released, and the inventory is atomically incremented back.
  3. **Edge Caching:** The product page and "Add to Cart" state must be aggressively cached at the edge. The edge must independently serve a "Sold Out" state once informed by the backend, protecting the application layer from read-storms.
  4. **Multi-Tenant Isolation:** Redis keys and LUA scripts must include `tenant_id` to ensure absolute isolation between simultaneous drops from different merchants.

  ### Mobile-First UX & Wireframes (375px First)
  1. **The Drop Countdown:**
     - **Visual:** A sleek, Translucent Glass card displaying the product and a live countdown timer. The "Buy" button is disabled and grayed out.
  2. **The Frenzy (Drop Active):**
     - **Interaction:** Exactly at the start time, the Edge Cache invalidates, and the "Buy" button turns vibrant OHC Primary Green.
  3. **The Checkout Queue (If needed):**
     - **Visual:** If the system is at maximum capacity, a beautiful, non-stressful "Hold tight, you're in line" screen with a smooth pulse animation. No technical jargon.
  4. **The "Secured" State:**
     - **Notification:** Once a reservation ticket is acquired, a 5-minute countdown begins. Text: "Item secured! You have 5:00 to complete payment."

  ### AI Agent Integration Points & Coordination
  - **The Manager (Operations Agent):** Automatically sets up the scalable infrastructure required right before the drop, pre-allocating the Redis instances and pre-warming the Edge Cache, then spins them down afterward to save platform costs.
  - **The Marketer (Marketing Agent):** Monitors the `DROP_EVENT` and sends automated, hype-building social posts and SMS notifications in the hours leading up to the drop.
  - **The Ambassador (Customer Success Agent):** If a user complains about missing a drop (detected via IG DMs or unified inbox), the Ambassador automatically drafts an apologetic reply with an exclusive discount code or early access pass for the next drop, sent for 1-tap approval by the owner.

  ## Implementation Prompt
  **Goal:** Build the High-Concurrency Flash Sale & Drop Mechanics Engine to guarantee zero overselling and 100% uptime during massive traffic spikes.

  **Core User Journey (CUJ):**
  1. Maya (a creator) schedules a drop for 50 limited-edition custom jackets at 12:00 PM on Friday.
  2. At 11:59 AM, 10,000 buyers are refreshing the page. The Edge Cache absorbs this load entirely.
  3. At 12:00 PM, the drop goes live. 5,000 buyers hit "Buy Now" simultaneously.
  4. The Inventory Service processes the atomic requests. The first 50 receive a 5-minute Reservation Ticket. The remaining 4,950 instantly receive a "Sold Out" message without crashing the platform.
  5. The AI Operations agent scales the backend resources appropriately right before the drop and spins them down right after.

  **Acceptance Criteria:**
  - Implement an atomic inventory reservation system using an in-memory datastore (e.g., Redis).
  - Implement the TTL-based Reservation Ticket logic (auto-release on expiration).
  - Ensure the Edge Cache can be dynamically updated by the backend to serve the "Sold Out" state.
  - Load test the system to prove it can handle a minimum of 1,000 simultaneous checkout requests per second per tenant with zero overselling.
  - Strict multi-tenant isolation in the caching and reservation layers.

  ## Priority
  P1 (High) - Critical for modern creator economy personas.

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
