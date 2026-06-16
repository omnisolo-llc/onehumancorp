issue_title: "[Research] OHC Dynamic Pricing & Margin Engine"
issue_description: |
  # Research Report: OHC Dynamic Pricing & Margin Engine

  ## 1. Problem Statement
  Owners like Priya (Boutique Operator) and Maya (Home Baker) lack the ability to intelligently adjust pricing dynamically based on demand, inventory levels, local competitor pricing, or operational capacity. Currently, setting up promotions, volume discounts, or adjusting base prices based on supply/demand is a manual, rigid process requiring constant checking. They are leaving money on the table or losing volume because their pricing is static.

  ## 2. Research Report
  - **Market Context**: Platforms like Shopify require third-party apps for dynamic pricing, which are often complex to set up, rules-based (not AI-driven), and focused strictly on e-commerce, ignoring in-store or service-based contexts.
  - **Competitor Gaps**:
    - *Shopify*: Relies heavily on apps. Native pricing is static.
    - *Wix/Squarespace*: Static pricing only. Requires manual intervention for sales or discounts.
    - *Square*: Basic pricing rules, not intelligent or demand-driven.
  - **The OHC Opportunity**: Integrating a dynamic pricing and margin engine powered by the Sales/Revenue Agent. This engine would automatically suggest (or automatically apply, with owner consent) price adjustments based on real-time data, balancing volume and margin.

  ## 3. Design Doc
  ### Data Model (PostgreSQL)
  - `Product/Service`: Base price and absolute minimum price (floor).
  - `PricingRule`: Defines conditions (e.g., inventory < 10, time of day, day of week) and actions (increase price by X%, apply discount Y).
  - `PriceHistory`: Audit log of all automated or manual price changes.
  - `DemandSignal`: Aggregated data points (page views, cart additions, local competitor pricing signals if available) used by the AI.

  ### AI Integration
  - **Sales & Revenue Assistant**: Monitors demand signals, inventory velocity, and historical sales data. It drafts a "Pricing Strategy Recommendation" (e.g., "Demand for Custom Vegan Cakes is 20% higher than normal for this weekend. Suggest raising the base price by $10 to maximize margin while managing your capacity. [Approve/Reject]").
  - **Operations Assistant**: Feeds capacity data (e.g., Leo's calendar is 90% full next week) to the Revenue agent to trigger dynamic pricing for remaining slots.

  ### Mobile UX Flow (375px)
  1. **Owner Work Feed**: A clear, actionable card appears: "Revenue Opportunity: High demand for [Item]. Adjust price?"
  2. **Detail View**: Tapping the card shows a simple graph of demand vs. current inventory/capacity, the projected revenue impact of the change, and large "Apply" or "Edit" buttons.
  3. **Configuration**: "Advanced Settings" (hidden by default) allows setting the absolute minimum price and whether the agent can auto-apply rules or only suggest them.

  ## 4. Implementation Prompt
  **Feature Name**: OHC Dynamic Pricing Engine

  **User-Facing Outcome**: The owner receives actionable, AI-driven suggestions to optimize pricing based on real-time demand and inventory/capacity. They can approve the changes with one tap, knowing their business is maximizing revenue without manual spreadsheet tracking.

  **Critical User Journey (CUJ)**:
  1. Priya logs into the OHC mobile app.
  2. Her work feed shows a notification from the Revenue Assistant: "Summer Dress inventory is moving slowly. Suggest a 15% flash sale to clear stock before Fall arrivals."
  3. Priya reviews the projected revenue vs. current trend.
  4. Priya taps "Approve."
  5. The product price is updated across all channels (online storefront and POS), and a summary is added to her daily report.

  **Acceptance Criteria**:
  - Implement the `PricingRule` and `DemandSignal` data models with strict tenant isolation.
  - Create the backend service for the Sales & Revenue Assistant to generate price adjustment suggestions based on mock demand signals.
  - Build the mobile-first UI cards for the owner feed to display, approve, and reject pricing suggestions.
  - Ensure all price updates are atomically applied to both online and offline (POS) contexts.

  ## 5. Priority & Scope
  - **Priority**: P1 (High)
  - **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
