issue_title: "[architecture]_autonomous_dynamic_pricing_and_promotion_engine"
issue_description: |
  # Issue Brief: Autonomous Dynamic Pricing & Promotion Engine

  ## Problem Statement
  Small business owners (especially in food, boutique retail, and seasonal services) struggle to manage inventory lifecycles optimally. When items like baked goods near expiration (Maya the baker) or seasonal clothing reaches the end of the month (Priya the boutique owner), they must manually calculate discounts, create promotional graphics, and push notifications to clear the stock. This manual cognitive load means stock often goes to waste or margins are lost. Non-technical users need an invisible system that autonomously tracks inventory age/velocity, triggers dynamic pricing (flash sales), and coordinates with the Marketing Agent to draft 1-tap approval promotions before the inventory becomes dead stock.

  ## Research Report
  ### Track 1: Architectural Gap & Scaling Discovery
  - **Codebase & Docs Audit**: OHC currently handles static pricing well, and has foundational inventory ledgers (`docs/research/[architecture]_universal_capacity_and_inventory_ledger.md`), but lacks real-time, event-driven temporal triggers based on inventory velocity and expiration schedules.
  - **Competitor Systems Audit**: Shopify uses complex third-party apps for dynamic pricing which require heavy rule configuration. Wix lacks robust automated markdowns. Enterprise systems (like Amazon) use highly complex algorithmic pricing. OHC's opportunity is zero-config dynamic pricing.
  - **Identify Gaps**: OHC is missing an "Inventory Lifecycle Intelligence" layer that bridges the operations (inventory) and marketing (promotions) departments autonomously.

  ### Track 2: Selected Architecture Deep Dive
  - **Business Journey Mapping**:
    - *Activation*: User logs inventory with a simple shelf-life or "season" tag.
    - *Retention & Revenue*: As an item nears its "staleness" threshold, the Pricing Engine automatically computes an optimal discount to clear stock while maximizing margin.
    - *Engagement*: The Engine signals the Marketing Agent, which drafts an SMS/IG blast. The user gets a 1-tap notification: "Blue sweaters are moving slow. Tap to run a 20% flash sale and notify VIPs."
  - **Data Model & Invariants**:
    - **Entities**: `InventoryLedger`, `PricingRuleEngine`, `PromotionCampaign`.
    - **Invariants**: Pricing modifications must never drop below a hard floor (COGS + minimum margin) set during onboarding.
    - **Agent Coordination**: Operations Agent (tracks velocity) -> Finance Agent (validates margin) -> Marketing Agent (drafts comms).

  ### Track 3: Technical Integrity & Mobile-First Review
  - **Mobile-First UX Flow**: A "Yield Opportunities" card appears on the 375px dashboard. It uses a clean, translucent glassmorphism UI showing the proposed flash sale. One large primary button: "Approve & Launch Sale".
  - **Performance & Offline Targets**: The heavy lifting (velocity calculation) happens in background job queues. The UI only fetches pre-computed, lightweight promotion recommendations.
  - **Zero Trust & Security**: Multi-tenant isolation at the DB layer ensures pricing algorithms and sales velocity data cannot leak across tenant boundaries.

  ### Track 4: Strategic Feature Issue Dispatch
  - **Mermaid Diagrams**:

  #### Architecture Diagram
  ```mermaid
  erDiagram
      INVENTORY_LEDGER ||--o{ PRICING_EVENT : "monitors"
      PRICING_EVENT ||--o| PROMOTION_CAMPAIGN : "triggers"
      TENANT ||--o{ INVENTORY_LEDGER : "owns"

      INVENTORY_LEDGER {
          uuid id
          string item_name
          int current_stock
          timestamp expiry_or_season_end
          float base_price
          float floor_price
      }

      PRICING_EVENT {
          uuid id
          float suggested_discount
          string reason "e.g., nearing expiration"
          timestamp created_at
      }

      PROMOTION_CAMPAIGN {
          uuid id
          string channel "SMS, IG, Email"
          string status "Draft, Approved, Live"
      }
  ```

  #### Agent Coordination Sequence
  ```mermaid
  sequenceDiagram
      participant Cron as KAIROS Job Queue
      participant Ops as Operations Agent
      participant Finance as Finance Agent
      participant Marketing as Marketing Agent
      participant User as Mobile Dashboard

      Cron->>Ops: Daily Inventory Velocity Check
      Ops->>Ops: Detects "Vegan Cupcakes" expiring in 4 hours
      Ops->>Finance: Request Flash Sale Margin Check
      Finance-->>Ops: Approves 30% discount (Maintains floor margin)
      Ops->>Marketing: Trigger Flash Sale Draft
      Marketing->>User: Push Notification: "Clear Cupcakes: Tap to text VIPs with 30% off"
      User->>Marketing: 1-Tap Approve
      Marketing->>Marketing: Execute Campaign
  ```

  ## Implementation Prompt
  **To Implementer Agent:**
  Build the Autonomous Dynamic Pricing & Promotion Engine. Implement a background job queue service that periodically scans the `InventoryLedger` for items nearing their end-of-lifecycle or experiencing low sales velocity. Create the `PricingRuleEngine` that calculates a safe discount ensuring the price never falls below the predefined `floor_price`. Once a discount is calculated, generate a `PricingEvent` that triggers the Marketing Agent to draft a `PromotionCampaign`.

  Expose a mobile-first (375px) API endpoint that serves these drafted campaigns to the user dashboard as "Yield Opportunities" for 1-tap approval. Ensure all database queries and agent memory contexts enforce strict multi-tenant isolation. Do not prescribe specific libraries, but ensure the background tasks are resilient and do not block the main API thread.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
