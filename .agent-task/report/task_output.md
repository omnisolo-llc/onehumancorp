issue_title: "OHC Architecture Research: Agentic Smart Pricing & Dynamic Discount Engine"
issue_description: |
  # Research Report: Agentic Smart Pricing & Dynamic Discount Engine

  ## Executive Summary
  This report details an architectural deep-dive into an automated, "invisible" Smart Pricing & Dynamic Discount Engine for OneHumanCorp (OHC). For non-technical SMB owners (like Maya the Baker or Leo the Musician), knowing when, why, and how much to discount a product is a high-cognitive-load problem. Existing platforms provide static rules-based discount codes, but require manual intervention and deep understanding of inventory and margins. Our proposed architecture leverages the Business Advisory Agent and Operations Agent to completely automate this process.

  ## 1. Market Mapping & Competitor Discovery (Track 1)
  - **Shopify & BigCommerce**: Offer extremely powerful "compare-at" pricing and rules-based discounts. However, setting these up requires merchants to manually analyze their inventory, define rules, calculate margin impacts, and activate codes.
  - **Wix & Squarespace**: Offer basic coupon code generation but lack sophisticated inventory-linked logic.
  - **The Gap**: No major SMB platform proactively identifies slow-moving stock or seasonal trends and automatically drafts a targeted discount campaign (with margin protection) for the user to approve via a single tap.

  ## 2. OHC Gap & Pain Point Identification (Track 3)
  - **Persona Focus**: Priya (Boutique Owner) and Fatima (Food Cart Operator).
  - **Pain Points**:
    - Priya has excess winter inventory taking up space as spring approaches. She doesn't know what discount % will move the stock without destroying her profit margins.
    - Fatima wants to offload remaining food prep 1 hour before closing but cannot manually log into a dashboard to adjust prices while managing the physical cart.
  - **The Gap**: OHC currently lacks an autonomous pricing engine that marries inventory telemetry, time-of-day/seasonality, and COGS (Cost of Goods Sold) margin safety boundaries.

  ## 3. Deep Dive Architecture Design (Track 2 & Track 3)

  ### Data Model & Invariants
  - **Pricing Rules Ledger (PostgreSQL)**:
    - Tables: `smart_pricing_policies`, `active_discounts`.
    - Columns for `smart_pricing_policies`: `id`, `tenant_id`, `product_id`, `min_margin_percent`, `auto_discount_trigger_days_stagnant`, `max_discount_percent`.
    - Strict row-level multi-tenant isolation (`tenant_id`).
  - **Fast-Path Price Resolution (Redis)**:
    - For high-scale storefront reads, final prices are cached in Redis. Key pattern: `ohc:price:{tenant_id}:{product_id}`. Cache is invalidated on policy application.

  ### Architecture Diagram

  ```mermaid
  erDiagram
      TENANT ||--o{ SMART_PRICING_POLICIES : configures
      PRODUCT ||--o{ SMART_PRICING_POLICIES : applies_to
      SMART_PRICING_POLICIES ||--o{ ACTIVE_DISCOUNTS : triggers

      SMART_PRICING_POLICIES {
          uuid id
          uuid tenant_id
          uuid product_id
          float min_margin_percent
          int auto_discount_trigger_days_stagnant
          float max_discount_percent
      }
      ACTIVE_DISCOUNTS {
          uuid id
          uuid policy_id
          float discount_amount
          datetime expires_at
      }
  ```

  ### Sequence Diagram

  ```mermaid
  sequenceDiagram
      participant Advisor as The Advisor Agent
      participant DB as PostgreSQL Ledger
      participant App as OHC Mobile App (Owner)
      participant Promoter as The Promoter Agent
      participant Ops as The Operations Agent
      participant Cache as Redis (Edge)

      Advisor->>DB: Query stagnant inventory & margins
      DB-->>Advisor: Return slow-moving items
      Advisor->>Advisor: Calculate margin-safe discount
      Advisor->>App: Push Notification: "Smart Price Suggestion"
      App-->>App: Owner reviews Action Card
      App->>Ops: Owner taps "Approve"
      Ops->>DB: Insert into active_discounts
      Ops->>Cache: Invalidate ohc:price:{tenant_id}:{product_id}
      Ops->>Promoter: Trigger marketing broadcast
      Promoter-->>App: Draft social media post for review
  ```

  ### AI Agent Coordination
  - **The Advisor (Business Advisory Agent)**:
    - Runs a weekly CRON job analyzing the `inventory` and `order_history` tables.
    - Identifies stagnant stock (e.g., items with 0 sales in 30 days) or time-sensitive stock.
    - Calculates a safe discount percentage that respects the `min_margin_percent` invariant.
    - Generates a plain-language proposal: *"Your 'Winter Scarf' hasn't sold in 3 weeks. Would you like to apply a 20% discount this weekend to clear space? Your profit margin will remain safe at 35%."*
  - **The Promoter (Marketing Agent)**:
    - Upon user approval, automatically drafts a social media post/email announcing the flash sale.
  - **The Manager (Operations Agent)**:
    - Executes the price change in the database and invalidates the Redis cache.

  ### Mobile-First Implementation
  - **UX Flow (375px)**:
    1. A premium "Glassmorphism" Action Card appears in the user's Agent Feed on the OHC mobile app.
    2. Card title: "Smart Price Suggestion: Winter Scarf".
    3. The card displays the current price crossed out, the new suggested price, and the projected sales increase.
    4. Two prominent 44x44px touch targets: "Approve & Run Sale" or "Dismiss".
    5. Upon tapping "Approve", an optimistic UI updates the feed to show the sale is active.

  ## 4. Proposed Implementation Steps & Issue Prompt

  **Feature Name:** Agentic Smart Pricing & Dynamic Discount Engine

  **Target Persona:** Priya the Boutique Owner

  **Outcome:** The system proactively identifies stagnant inventory, calculates a margin-safe discount, and surfaces a one-tap approval card to the business owner, subsequently updating prices and drafting marketing materials autonomously.

  **Critical User Journey (CUJ):**
  1. Priya opens the OHC mobile app on a Friday morning.
  2. In her Agent Feed, she sees a suggestion from "The Advisor" noting that 4 units of "Blue Summer Dress" have been in stock for 60 days.
  3. The card suggests a 15% weekend discount to clear the inventory, noting her margin stays above 40%.
  4. Priya taps "Approve & Run Sale".
  5. The product price instantly updates on her live edge-cached storefront.
  6. The Promoter agent drafts an Instagram post about the sale for her review.

  **Next Actions for Engineering:**
  - **Step 1:** Implement the `smart_pricing_policies` schema in PostgreSQL with strict tenant boundaries.
  - **Step 2:** Develop the CRON-based inventory analysis pipeline for The Advisor agent to detect stagnant stock based on sales velocity.
  - **Step 3:** Build the mobile Agent Feed action card UI component matching the UniFi/Glassmorphism design tokens.
  - **Step 4:** Plumb the approval action through to the Operations agent to update the price and invalidate edge cache.

  **Priority:** P2
  **Estimated Scope:** Medium
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
