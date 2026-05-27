issue_title: "Architecture: Autonomous Dynamic Pricing & Flash Sale Engine"
issue_description: |
  # Title: Autonomous Dynamic Pricing & Flash Sale Engine

  ## Problem Statement
  Small business owners, especially those dealing with perishable goods (like Fatima's food cart) or seasonal inventory (like Priya's boutique), struggle to optimize pricing to minimize waste and maximize revenue. They often rely on manual, reactionary markdowns or simply throw away unsold inventory. They lack the time, data, and tools to predict demand and execute timely, targeted flash sales or dynamic pricing adjustments. They need an automated system that intelligently discounts items, broadcasts the sale to high-intent customers, and clears inventory before it loses value, all without requiring manual intervention or complex configuration.

  ## Research Report

  ### Industry Context & Competitive Analysis
  The ability to dynamically adjust prices based on inventory levels, time of day, and customer demand is a hallmark of enterprise retail and airline industries. SMBs have historically been locked out of these capabilities.

  *   **Shopify:** Offers basic discounting and "compare at" pricing. Third-party apps (like Bold Custom Pricing or Prisync) exist but are complex, require manual rule setup, and do not proactively analyze inventory staleness to autonomously trigger flash sales.
  *   **Wix & Squarespace:** Similar to Shopify, they offer manual coupon codes and basic sales features. They lack autonomous, inventory-driven pricing optimization.
  *   **Square:** Provides solid inventory tracking and manual discounts. While they have strong data, they do not offer an "autopilot" mode for clearing out perishable or stale inventory via dynamic flash sales.
  *   **Too Good To Go:** A consumer app focused specifically on food waste. While effective, it takes a significant cut, requires businesses to use a separate platform, and doesn't integrate natively with their primary storefront or POS.

  ### Key Findings & Opportunities
  1.  **Zero-Touch Automation:** The primary barrier is not the ability to change a price, but the cognitive load of deciding *when* and by *how much*. The system must be proactive.
  2.  **Omnichannel Broadcasting:** A flash sale is only effective if customers know about it. The engine must seamlessly integrate with the Omnichannel AI Inbox to notify past customers via SMS, Email, or WhatsApp.
  3.  **Perishable vs. Seasonal Context:** The engine needs to understand the temporal nature of items (e.g., a croissant expires in 4 hours; a summer dress becomes stale in 3 months).

  ## Design Doc

  ### Architecture Diagram

  ```mermaid
  erDiagram
      INVENTORY_ITEM ||--o{ PRICING_STRATEGY : "governed by"
      INVENTORY_ITEM ||--o{ INVENTORY_LEDGER : "tracked in"
      PRICING_STRATEGY {
          string id
          string tenant_id
          string item_id
          string strategy_type "enum: PERISHABLE, SEASONAL, OVERSTOCK"
          float min_price_floor
          boolean auto_broadcast
      }
      FLASH_SALE_EVENT {
          string id
          string tenant_id
          string item_id
          float discount_percentage
          datetime start_time
          datetime end_time
          string status "enum: ACTIVE, COMPLETED, CANCELLED"
      }
      FLASH_SALE_EVENT ||--o{ BROADCAST_CAMPAIGN : "triggers"
      BROADCAST_CAMPAIGN {
          string id
          string event_id
          string channel "enum: SMS, EMAIL, WHATSAPP"
          int audience_size
      }

      AI_OPERATIONS_DEPT }|--|{ INVENTORY_LEDGER : "monitors"
      AI_OPERATIONS_DEPT }|--|{ PRICING_STRATEGY : "evaluates"
      AI_OPERATIONS_DEPT }|--|{ FLASH_SALE_EVENT : "creates & manages"
      AI_MARKETING_DEPT }|--|{ BROADCAST_CAMPAIGN : "executes"
  ```

  ```mermaid
  sequenceDiagram
      participant Catalog as Universal Catalog
      participant Inv as Inventory Ledger
      participant OpsAgent as AI Operations Dept
      participant MktAgent as AI Marketing Dept
      participant Store as Edge Storefront
      participant Cust as Customer (Mobile)

      OpsAgent->>Inv: Periodically scan for stale/expiring inventory
      Inv-->>OpsAgent: Return items matching criteria (e.g., 10 croissants left at 4 PM)
      OpsAgent->>Catalog: Retrieve Pricing Strategy for items
      OpsAgent->>OpsAgent: Determine optimal discount (e.g., 50% off to clear before close)
      OpsAgent->>Store: Publish Flash Sale Event (Update edge cache)
      OpsAgent->>MktAgent: Request Broadcast Campaign
      MktAgent->>Cust: Send targeted SMS/WhatsApp ("Flash Sale: 50% off remaining croissants!")
      Cust->>Store: View item & Checkout
      Store->>Inv: Deduct inventory
      Inv-->>OpsAgent: Inventory cleared
      OpsAgent->>Store: End Flash Sale Event
  ```

  ### UI Wireframes & Mobile UX Flow (375px first)
  1.  **Dashboard Alert (Card):** A translucent glass card appears on the main dashboard: "Auto-Sale Alert: 12 Vegan Cupcakes expire in 3 hours. Autopilot discounted them by 40% and notified 45 local regulars." Action: [Cancel Sale] [View Details].
  2.  **Product Detail "Advanced Settings":** Inside a product's edit screen, a simple toggle under the price: "Smart Clearance (Autopilot)". Tapping it opens a bottom sheet:
      *   Minimum acceptable price (e.g., $2.00)
      *   Shelf life / Staleness threshold (e.g., "End of day" or "90 days without sale").
  3.  **Storefront View (Customer):** The item card on the mobile storefront gets a glowing "Flash Sale" badge. A countdown timer ("Ends in 2h 15m") is displayed prominently next to the crossed-out original price.

  ### AI Agent Integration Points
  *   **AI Operations Department:** Runs as a high-performance background job. It continuously evaluates the Inventory Ledger against the predefined Pricing Strategies. It decides the discount curve (e.g., 10% off at 2 PM, 50% off at 4 PM) based on historical sales velocity.
  *   **AI Marketing Department:** Receives the trigger from Ops and automatically drafts a casual, urgency-driven message in the persona of the business owner. It selects the audience (e.g., "Customers who bought baked goods in the last 30 days and live within 5 miles").

  ### Zero Trust & Security, Performance Targets
  *   **Multi-tenant Isolation:** All background evaluation jobs must strictly partition queries by `tenant_id`. AI operations cannot cross-pollinate inventory data between tenants.
  *   **Edge Caching:** When a flash sale activates, the updated price and countdown timer must propagate to the edge cache within < 500ms to prevent latency during checkout rushes. The storefront must not hit the primary DB to fetch the active price.
  *   **Price Invariants:** Hard guarantees must be enforced at the API layer that a dynamic price can *never* fall below the user-defined `min_price_floor`.

  ### Key Design Decisions
  *   **Opt-out, not Opt-in for complexity:** The system defaults to standard pricing unless "Smart Clearance" is toggled. When toggled, the AI takes over; the user does not write the rules manually.
  *   **Unified Campaign Generation:** The flash sale engine doesn't just change the price; it must autonomously trigger the marketing broadcast. Changing the price in a vacuum for an SMB is useless if traffic isn't driven to the storefront.

  ## Implementation Prompt

  **Goal:** Implement the Autonomous Dynamic Pricing & Flash Sale Engine for OneHumanCorp.

  **User Journey (CUJ):**
  1. Fatima sets up a new menu item "Halal Chicken Over Rice". She enables "Smart Clearance", setting a minimum price of $4.00 and indicating it clears at the end of the day.
  2. At 8:00 PM, with 15 portions remaining, the system autonomously creates a Flash Sale Event reducing the price to $5.00.
  3. The system automatically sends an SMS broadcast to recent customers nearby.
  4. Customers see the updated price on her OHC storefront and order. The system automatically ends the sale when inventory hits 0.

  **Acceptance Criteria:**
  *   **Data Model:** Extend the product/inventory models to support `PricingStrategy` and `FlashSaleEvent` entities with strict `tenant_id` isolation.
  *   **Agent Logic:** Create a background job process for the Operations Agent that can evaluate inventory and trigger price adjustments based on staleness/perishability.
  *   **Marketing Trigger:** Ensure the creation of a `FlashSaleEvent` securely queues an action for the Marketing Agent to generate and send a localized broadcast (mock the actual SMS delivery if necessary, but the queue must work).
  *   **UI (Mobile Parity):** Build the "Smart Clearance" toggle in the product edit flow (Tauri app). Ensure the storefront UI visually reflects the active flash sale with a countdown and discounted price.
  *   **Performance:** Storefront price queries must be cacheable at the edge, invalidating only when a `FlashSaleEvent` starts or ends.
  *   **Safety:** The backend MUST enforce the `min_price_floor` invariant on checkout, even if the cache is stale.

  **Do NOT:**
  *   Prescribe specific database schemas (e.g., exact column types) or API endpoint URLs.
  *   Assume a specific background job framework (use the existing OHC orchestration).

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
