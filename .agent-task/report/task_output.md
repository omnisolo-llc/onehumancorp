issue_title: "Agentic Smart Pricing & Dynamic Margin Optimization"
issue_description: |
  # Research Report: Agentic Smart Pricing & Dynamic Margin Optimization

  ## Title
  Agentic Smart Pricing & Dynamic Margin Optimization

  ## Problem Statement
  Small business owners often struggle with pricing their products or services optimally. They usually set a static price based on intuition or basic cost-plus analysis. They lack the time, tools, and expertise to dynamically adjust prices based on supply, demand, competitor pricing, seasonality, or customer lifetime value. Traditional e-commerce platforms like Shopify or Wix offer basic pricing fields or require expensive third-party tools to enable dynamic pricing, which are too complex for non-technical users like Maya (baker) or Carlos (handyman).

  ## Research Report
  - **Market Context**: Dynamic pricing is heavily utilized by enterprises (Amazon, Uber, Airlines) but is largely inaccessible to SMBs. Third-party apps on Shopify (like Prisync or BlackCurve) are expensive ($50-$200+/month) and complex to configure. Wix and Squarespace have static pricing with manual discount codes.
  - **OHC Opportunity**: OHC can democratize dynamic pricing by embedding an "Agentic Smart Pricing" engine directly into the product catalog and service offerings. This engine will autonomously analyze local data (inventory, booking availability), tenant history, and macro-trends to optimize margins invisibly.
  - **Competitor Gaps**:
    - *Shopify*: Relies on third-party apps for dynamic pricing. Basic native features only support manual "compare at" prices.
    - *Wix/Squarespace*: Purely manual pricing.
    - *Stripe*: Great for payments and subscriptions, but doesn't autonomously adjust prices based on external/internal signals.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Product/Service Catalog] --> B(Pricing Engine)
      C[Inventory/Availability Status] --> B
      D[Sales Velocity & Historical Data] --> B
      E[Competitor/Market Signals] --> B
      B --> F{The Analyst Agent}
      F -->|Proposes Price Change| G[Action Required Queue]
      G --> H[Mobile App Feed 375px]
      H -->|1-Tap Approve| I[Update Catalog Price]
      I --> J[Stripe Product/Price Update]
  ```

  ### Mobile UX Flow (375px)
  1. **Home Feed (Mobile)**: The owner receives a smart card: "High Demand Alert".
  2. **Interaction**: Tapping the card reveals insights: "Your custom vegan cakes are selling 3x faster than usual this week. I suggest raising the price by 10% to optimize margins without losing sales volume."
  3. **Action**: A prominent primary button "Approve Price Increase" and a secondary "Edit Suggestion".
  4. **Product Settings (Optional)**: In the product settings, the owner can set a toggle "Enable Smart Pricing" with minimum and maximum price boundaries (e.g., $40 - $60). If enabled, the agent can adjust prices autonomously within limits, or require approval outside limits.

  ### AI Agent Integration Points
  - **Finance/Analyst Agent (The Analyst)**: Continuously monitors sales velocity, inventory levels (via Operations Agent), and external signals. Uses tenant-scoped data to run predictive models.
  - **Customer Success Agent (The Ambassador)**: Ensures price changes are communicated gently if applicable (e.g., "Lock in this price before our seasonal rates apply").

  ### Key Design Decisions
  - **Owner Control**: The AI suggests, the owner approves. Full autonomous pricing is only active if explicitly enabled with strict boundaries.
  - **Actionable Insights**: Instead of just showing a chart of dropping margins, the agent proposes a concrete fix (e.g., "Raise price by $5").

  ## Implementation Prompt
  **User-Facing Outcome**: As an owner, I receive actionable recommendations on my mobile feed to adjust my prices based on real-time demand, helping me maximize my profit margins without having to stare at spreadsheets or guess.
  **CUJ & Acceptance Criteria**:
  1. Implement a new `PricingPolicy` entity associated with a `Product` or `Service`, allowing for dynamic pricing flags, min/max bounds.
  2. Develop a background worker (The Analyst Agent) that periodically analyzes sales velocity and inventory for a tenant.
  3. When an anomaly or optimization opportunity is detected (e.g., high sales velocity), generate an `ActionRequired` item in the feed.
  4. Provide a mobile-first UI card for the owner to review the suggested price change.
  5. Provide a 1-tap "Approve" action that securely updates the price in the PostgreSQL database and propagates it to the connected Stripe account (via API).
  6. Include Playwright E2E tests: A user logs in, sees a "Price Optimization" card in the feed, taps "Approve", and verifies the product price is updated in the catalog.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []