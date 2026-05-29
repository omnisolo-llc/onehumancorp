issue_title: "[Architecture] Autonomous AI Dynamic Pricing & Yield Management Engine"
issue_description: |
  # [Architecture] Autonomous AI Dynamic Pricing & Yield Management Engine

  ## Problem Statement

  Small business owners—from bakers like Maya to service providers like Carlos and tutors like Leo—struggle with pricing. They typically set a static price and forget it, leading to left-on-the-table revenue during high-demand periods or lost sales during slow times.

  *   **Maya (baker):** During wedding season (May-September), she is booked solid but still charges her standard rates, turning away customers because her calendar is full. She loses the opportunity to capture a premium from desperate last-minute requests.
  *   **Carlos (handyman):** When a sudden freeze causes pipes to burst across town, demand spikes. Carlos charges his standard $80/hr, burning out while competitors charge $150/hr for emergency services.
  *   **Leo (music tutor):** Has empty slots on Tuesday mornings while his Wednesday evenings are waitlisted. He doesn't know how to incentivize off-peak bookings without cheapening his brand.

  Existing platforms (Shopify, Wix, Squarespace) offer "sale" badges or manual discount codes, but they **require the user to act as a data analyst and manually adjust prices**. They offer no yield management logic out-of-the-box. OHC needs an invisible agent that acts like an airline pricing engine, optimizing revenue autonomously while feeling natural and transparent to the end customer.

  ## Research Report

  **Competitor Analysis:**

  *   **Shopify:** Offers robust manual discount systems (percentage, fixed amount, buy-x-get-y) and third-party apps (e.g., Prisync, Priceva) for dynamic pricing. These apps are geared towards large e-commerce merchants competing on Amazon or Google Shopping, scraping competitor prices. They are complex, require configuration of "rules," and cost $50-$200+/month. They are useless for service businesses or unique local physical goods.
  *   **Wix/Squarespace:** Basic manual discounts. No dynamic pricing or yield management native features.
  *   **Uber/Airlines (Inspiration):** Utilize sophisticated algorithmic yield management. High demand/low supply = higher prices. Low demand/high supply = lower prices (or promotions).

  **The OHC Gap (The SMB Yield Opportunity):**
  The gap is bringing enterprise-grade yield management to a 1-person business *with zero configuration*. If a local service provider's calendar is 90% full, the remaining 10% of slots should automatically command a premium. If an inventory item is languishing, it should be quietly promoted to specific past buyers or bundled.

  ## Design Doc

  ### Architecture Diagram

  ```mermaid
  graph TD
      subgraph OHC Core Edge
          A[Customer App/Storefront]
      end

      subgraph Pricing Mesh (Multi-Tenant)
          B[Yield Management Gateway]
          C[Dynamic Pricing Engine]
          D[AI Pricing Analyst Agent]
      end

      subgraph Data Layer & Signals
          E[Inventory/Capacity Ledger]
          F[Market & Weather Signal Ingest]
          G[Historical Transaction DB]
      end

      A -->|Request Price Quote/View Item| B
      B --> C
      C <-->|Query Availability| E
      C <-->|Get Real-time multipliers| D
      D <-->|Read Signals| F
      D <-->|Analyze Past Conversions| G
      C -->|Return Optimized Price| B
      B --> A
  ```

  ### Mobile UX Flow (375px First)

  **Merchant View (The "Grandmother Test"):**

  1.  **Dashboard Alert Card:** A clean, translucent card appears on the OHC app home screen: "Demand is unusually high this weekend for Plumbing Services. Should we enable Surge Pricing (+$40/hr)?"
  2.  **One-Tap Action:** A simple "Enable Surge" button. (Or, in advanced mode, this happens completely autonomously based on pre-approved rules).
  3.  **Pricing Strategy Toggle (Settings):** Under a product or service setting: "Auto-Optimize Pricing" toggle (On/Off). Subtext: "OHC will adjust prices slightly based on demand to maximize your profit."

  **Customer View:**

  *   **Transparency:** When a higher price is shown, the UI gracefully explains why (e.g., a small tag: "High Demand Slot" or "Only 2 spots left at this time").
  *   **Off-Peak Incentives:** When booking an off-peak time: "Save 15% by booking this Tuesday morning slot."

  ### AI Agent Integration Points

  *   **Finance Department Agent (The Analyst):** Continuously monitors the Inventory/Capacity Ledger. If inventory for SKU A is high and velocity is low, it signals the Pricing Engine to allow discounts.
  *   **Operations Department Agent (The Dispatcher):** For services, monitors local weather APIs (e.g., heavy snow = high demand for roofers) and local calendar density.

  ### Key Design Decisions & Rationale

  1.  **Real-Time Price Calculation at the Edge:** Prices are not statically stored on the product model; they are computed at request time by the Dynamic Pricing Engine, taking into account the base price, current capacity, and AI multipliers.
  2.  **Opt-in Autonomy vs. Full Autonomy:** Given the sensitivity of pricing, merchants must be able to toggle "Auto-Optimize" per item/service or rely on 1-tap approvals via the activity feed.
  3.  **Isolation:** Pricing rules and historical data must be strictly partitioned per tenant to prevent cross-contamination of pricing strategies (Zero Trust multi-tenancy).

  ## Implementation Prompt

  **Role:** Implementer Agent
  **Task:** Build the core logic for the Autonomous AI Dynamic Pricing & Yield Management Engine.

  **User Journey (CUJ):**
  As Carlos the Handyman, when my calendar is 80% booked for the week, I want OHC to automatically increase my rate for the remaining slots by 20% (with my permission) so that I maximize revenue without doing any math. When a customer views my booking page, they see the standard rate for next week, but a premium "high demand" rate for this week's remaining slots.

  **Acceptance Criteria:**

  1.  **Yield Calculation:** Implement a service that accepts a base price, current capacity percentage (0-100%), and an optional external demand multiplier, returning the dynamically adjusted price.
  2.  **Tenant Isolation:** Ensure that all pricing calculations strictly respect tenant boundaries; Tenant A's high demand cannot affect Tenant B's prices.
  3.  **Edge Integration:** Expose this service such that the edge storefront/booking UI can request the real-time price synchronously with sub-50ms latency.
  4.  **Audit Trail:** Every dynamically adjusted price must log *why* it was adjusted (e.g., "Calendar 85% full" or "Local weather event") for the merchant's daily briefing.

  *Note: Do not prescribe specific database schemas or API endpoints. Design the internal structure that best fulfills these requirements while maintaining high performance and reliability.*

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
