issue_title: "Implement AI-Driven Adaptive Pricing & Smart Discounts"
issue_description: |
  # Research Report: AI-Driven Adaptive Pricing & Smart Discounts

  ## Problem Statement
  Small business owners (like Priya the boutique owner and Maya the home baker) struggle with setting optimal prices and managing discounts. They either offer flat discounts that erode margins or miss out on sales because prices are too rigid. They lack the time and analytical tools to dynamically adjust pricing based on demand, inventory levels, or customer loyalty.

  ## Research Report
  - **Market Context**: Enterprise platforms use dynamic pricing (like airlines and Uber), but SMBs rely on manual, static pricing. Shopify offers apps for discount rules, but they are rigid and require complex setup. Square offers loyalty programs but not dynamic, AI-driven pricing adjustments.
  - **The OHC Opportunity**: By integrating an AI-driven pricing and discount engine, OHC can automatically suggest or apply price adjustments and targeted discounts. For example, if inventory of a specific dress is high and hasn't sold in 30 days, the AI can propose a temporary flash sale to targeted past customers.
  - **Competitor Gaps**:
    - *Shopify*: Requires paid 3rd-party apps for dynamic pricing.
    - *Square*: Basic discount rules, no AI-driven yield management.

  ## Design Doc
  ### Data Model (PostgreSQL)
  - `PricingRule`: Defines rules (e.g., clearance, time-of-day for food carts).
  - `TargetedDiscount`: A temporary, personalized discount applied to a specific customer or segment.
  - `PriceRecommendation`: AI-generated suggestions for the owner to approve.

  ### AI Agent Integration
  - **Finance / Sales Agent**: Analyzes sales velocity, inventory levels, and customer purchase history.
  - Generates `PriceRecommendation` cards for the owner's Agent Feed (e.g., "Sales of Red Dress are down 20%. Recommend offering a 15% discount to your top 50 customers to clear inventory. [Approve] [Edit] [Discard]").

  ### Mobile UX Flow (375px)
  1. **Owner View (Agent Feed)**: The owner receives a notification card suggesting a pricing action.
  2. **Approval**: Tapping "Approve" instantly updates the storefront and optionally drafts an email/SMS campaign via the Marketing Agent.
  3. **Customer View**: Customers see the discounted price, perhaps with a "Personalized for you" badge.

  ## Implementation Prompt
  **Feature Name**: OHC AI-Driven Adaptive Pricing
  **Target Persona**: Priya (Boutique Owner)
  **Outcome**: Priya receives actionable AI recommendations to clear slow-moving inventory via targeted discounts, which she can approve with one tap on her phone.

  **Next Actions**:
  1. Design the `PriceRecommendation` and `TargetedDiscount` database schemas with tenant isolation.
  2. Extend the Sales Agent to analyze inventory age and sales velocity to generate recommendations.
  3. Implement the Mobile-first (375px) Agent Feed card for pricing approvals.
  4. Integrate the approved discount into the checkout and shopping cart flow.

  **Priority**: P2
  **Estimated Scope**: Medium
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
