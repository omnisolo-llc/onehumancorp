issue_title: "[Sales] Autonomous Upsell & Cross-Sell Engine"
issue_description: |
  # Issue Brief: Autonomous Upsell & Cross-Sell Engine

  ## Problem Statement
  Small business owners consistently miss out on an estimated 15-30% in potential revenue because they do not have the time, skill, or context to offer relevant upsells at the exact moment of high customer intent. Traditional platforms require manual configuration of "frequently bought together" rules. Owners need an invisible, AI-driven engine that dynamically analyzes cart contents and past customer behaviors to generate and propose highly relevant, 1-tap upsell and cross-sell offers instantly.

  ## Research Report
  - **Competitive Audit**: Shopify / Wix rely on rigid, manual configurations. Amazon / UberEats use highly sophisticated algorithmic models out of reach for SMBs. OHC will leverage the AI Salesperson to instantly infer what complements a purchase based on the existing product catalog and inventory.
  - **Key Findings**: 35% of Amazon's revenue comes from its recommendation engine. SMBs see a 10-30% increase in AOV when relevant upsells are presented contextually. If a user has to manually link products, the feature is unused by 85% of merchants.

  ## Proposed Solution
  1. Build the event listener for `CartUpdated` and `CheckoutInitiated` events that calls the Salesperson Agent.
  2. Implement the AI logic to dynamically identify complementary products from the local SQLite/SIPDB catalog without explicit user rules.
  3. Coordinate with the Operations Agent to ensure suggested items are in stock.
  4. Implement the Glassmorphism mobile UI component (`UpsellBottomSheet`) that displays the offer seamlessly during checkout.
  5. Create the `Upsell_Ledger` metrics tracking so the Business Advisory agent can report on added revenue.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
