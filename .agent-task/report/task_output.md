issue_title: "[architecture]_autonomous_yield_management_and_dynamic_pricing_engine"
issue_description: |
  # Issue Brief: Autonomous Yield Management & Dynamic Pricing Engine

  ## Problem Statement
  Service-based and non-standard small businesses suffer from unoptimized capacity and missed revenue opportunities. Specifically:
  - **Leo (music tutor, 22)** has empty slots during weekday mornings and turns away students on weekends, losing potential income. He lacks a dynamic pricing model to incentivize off-peak bookings.
  - **Carlos (handyman, 42)** handles emergency plumbing repairs but charges the same flat rate during a Tuesday afternoon as he does on a Sunday at 2 AM.
  - **Priya (boutique owner, 35)** has seasonal inventory sitting on shelves that needs markdowns, but she doesn't know when or how much to discount without destroying her margins.

  Currently, these users must manually adjust prices, create complex discount codes, or mentally negotiate rates on the fly. This requires deep financial acumen and constant monitoring, which violates our core tenet of removing cognitive load from the small business owner. The platform currently lacks a centralized, autonomous engine that dynamically adjusts prices based on real-time supply, demand, and temporal factors (time of day, seasonality).

  ## Research Report
  - **Competitor Analysis**:
    - **Shopify / Wix**: Support manual sale prices and discount codes, but lack native, autonomous dynamic pricing (yield management) based on real-time inventory or calendar capacity.
    - **Airlines / Ride-Sharing (Uber, Airbnb)**: Master yield management (surge pricing, smart pricing), but their tools are enterprise-grade.
    - **Mindbody / Acuity**: Have basic off-peak pricing, but require extensive manual configuration of pricing rules and schedules.
  - **The Gap**: Small businesses need "Enterprise Yield Management in a Box." They need an AI agent that silently monitors their calendar or inventory and autonomously adjusts prices within pre-approved boundaries to maximize revenue and utilization.
  - **The Solution**: An Autonomous Yield Management & Dynamic Pricing Engine. It connects the Universal Capacity & Inventory Ledger to the AI Sales/Finance Agents. The user simply says, "Maximize my revenue, but never charge less than $40/hr," and the engine takes over.

  ## Design Doc
  ### High-Level Architecture
  - **Trigger Events**:
    - *Inventory/Capacity Change*: A booking is made, a cancellation occurs, or an item is sold.
    - *Temporal Trigger*: Time passes (e.g., a perishable slot like a Tuesday 10 AM lesson is approaching and remains unfilled).
    - *Demand Spike*: A sudden surge in profile views or booking inquiries.
  - **Yield Engine**: Analyzes the trigger against the Merchant's configured bounds (Floor/Ceiling prices).
  - **Agent Action**: The AI Operations Agent recalculates the optimal price and updates the Universal Ledger. The AI Marketing Agent can optionally trigger a promotional message (e.g., SMS to inactive students: "Last minute slot available tomorrow at 20% off").

  ### Architecture Diagram
  ```mermaid
  erDiagram
      MERCHANT ||--o{ PRICING_STRATEGY : "configures"
      PRICING_STRATEGY ||--o{ DYNAMIC_RULE : "contains"
      MERCHANT ||--o{ CAPACITY_LEDGER : "owns"
      CAPACITY_LEDGER ||--o{ PRICE_MODIFIER_EVENT : "triggers"
      PRICE_MODIFIER_EVENT ||--|{ YIELD_ENGINE : "processed by"
      YIELD_ENGINE ||--o{ AI_OPERATIONS_AGENT : "orchestrates"

      PRICING_STRATEGY {
          string id
          string merchant_id
          float absolute_floor_price
          float absolute_ceiling_price
          boolean autonomous_mode
      }
      DYNAMIC_RULE {
          string id
          string strategy_id
          string trigger_type "TIME_TO_EXPIRY, DEMAND_SURGE, INVENTORY_LOW"
          float adjustment_percentage
      }
  ```

  ```mermaid
  sequenceDiagram
      participant Customer (Web/Mobile)
      participant OHC Storefront/Booking API
      participant Universal Capacity Ledger
      participant Autonomous Yield Engine
      participant AI Marketing Agent

      Customer (Web/Mobile)->>OHC Storefront/Booking API: "View availability for Leo (Tutor)"
      OHC Storefront/Booking API->>Universal Capacity Ledger: Fetch available slots
      Universal Capacity Ledger->>Autonomous Yield Engine: Request real-time price for slots
      Autonomous Yield Engine-->>Universal Capacity Ledger: Compute dynamic prices (e.g. Off-peak discount applied)
      Universal Capacity Ledger-->>OHC Storefront/Booking API: Return slots with dynamic prices
      OHC Storefront/Booking API-->>Customer (Web/Mobile): Display prices

      Note over Autonomous Yield Engine, AI Marketing Agent: Background Optimization
      Autonomous Yield Engine->>Universal Capacity Ledger: Detect high expiring inventory (tomorrow's slots)
      Autonomous Yield Engine->>AI Marketing Agent: Signal to run flash promotion
      AI Marketing Agent->>Customer (Web/Mobile): SMS: "Last minute 20% off tomorrow's lesson!"
  ```

  ### Mobile UX Flow (375px First)
  1. **Activation Screen**: A translucent glass card in the Settings tab titled "Smart Pricing Engine."
  2. **Natural Language Input**: "How do you want to handle pricing?" Options: "Fixed Rates" vs. "Smart Yield (Maximize Revenue)."
  3. **Boundary Configuration**: If Smart Yield is selected, a simple slider or numeric input asks: "What is your absolute minimum acceptable price?" and "What is your premium surge price?"
  4. **The "Set & Forget" State**: The engine activates. The dashboard shows a subtle pulsing indicator: "AI Yield Management Active."
  5. **Insight Briefing**: The weekly plain-text SMS from the Business Advisory Agent includes yield stats: "Your Smart Pricing filled 4 empty Tuesday slots this week, earning you an extra $160."

  ### Zero Trust & Security Guarantees
  - Pricing Strategies and Rules are strictly scoped to the Tenant ID.
  - The Yield Engine runs as an isolated microservice, executing pricing algorithms statelessly against the tenant's bounded context.

  ## Implementation Prompt
  **Role:** Implementer Agent
  **Task:** Build the core backend data structures and evaluation logic for the Autonomous Yield Management Engine.
  **Outcome:**
  1. Define the `PricingStrategy` and `DynamicRule` entities with robust multi-tenant isolation.
  2. Implement the `YieldEngine` service that takes a base price, a capacity context (e.g., time to expiry, current utilization), and evaluates it against the merchant's `PricingStrategy` to return a real-time modified price.
  3. Create event listeners that trigger the yield recalculation when capacity changes in the Universal Capacity Ledger.
  **Acceptance Criteria:**
  - The engine must reliably enforce the merchant's configured floor and ceiling prices, regardless of dynamic rule combinations.
  - Evaluation latency must be under 50ms to ensure storefront edge-caching and real-time checkout flows are not blocked.
  - Ensure comprehensive unit tests covering edge cases like overlapping rules and extreme demand spikes.
  - Do not build frontend UI; focus entirely on the robust backend service and API boundaries.

  ## Estimated Scope
  Large

issue_priority: "P1"
issue_category: "research"
issue_type: "task"
issue_label:
  - "agent-report"
assignees: []
