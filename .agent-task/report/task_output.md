issue_title: "[Architecture] Autonomous AI Dynamic Pricing & Yield Management Engine"
issue_description: |
  # Research Report: Autonomous AI Dynamic Pricing & Yield Management Engine

  ## Problem Statement
  Small business owners often struggle with pricing strategy. Priya (boutique owner) may have excess winter inventory sitting unsold while spring approaches, while Leo (music tutor) might have highly sought-after evening slots that are underpriced compared to his less popular morning slots. Manually analyzing sales velocity, competitor pricing, and inventory levels to adjust prices is time-consuming and requires specialized knowledge that these owners lack. They need an automated system that intelligently adjusts prices and creates promotions to maximize revenue and inventory turnover without manual intervention.

  ## Target Capabilities
  - **Autonomous Price Adjustments:** Automatically adjust prices based on predefined rules, inventory levels, sales velocity, and external factors (e.g., seasonality, competitor pricing data if available).
  - **Yield Management for Services:** Dynamically price booking slots based on demand (e.g., surge pricing for peak hours, discounts for off-peak hours).
  - **Agentic Promotions:** Automatically draft and suggest promotional campaigns (discounts, bundles) when inventory velocity drops below a threshold.
  - **Transparent Owner Control:** Present all AI-driven pricing changes and promotional suggestions as actionable cards in the mobile feed for easy approval, ensuring the owner remains in control.

  ## Competitive Landscape
  - **Enterprise E-commerce (Amazon, Airlines):** Utilize highly sophisticated, real-time dynamic pricing algorithms.
  - **Shopify/Wix:** Require third-party apps for dynamic pricing, which are often complex to configure and target mid-market rather than micro-SMBs.
  - **OHC Opportunity:** Democratize yield management by integrating it natively as an autonomous, invisible agent that works on behalf of the SMB, requiring zero configuration.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      SalesData[Sales & Booking Data] --> AI_Pricing_Agent[AI Pricing & Yield Agent]
      InventoryData[Inventory Levels] --> AI_Pricing_Agent
      MarketSignals[External/Seasonal Signals] --> AI_Pricing_Agent

      AI_Pricing_Agent -- Generates --> PricingStrategy[Pricing Strategy / Rules]
      PricingStrategy --> RuleEngine[Dynamic Pricing Rule Engine]
      RuleEngine --> Checkout[Checkout & Storefront]

      AI_Pricing_Agent -- Proposes --> OwnerFeed[Owner Mobile Feed]
      OwnerFeed -- Approves --> RuleEngine
  ```

  ### Mobile UX Flow (375px First)
  - **The "Smart Pricing" Card:** An actionable card appears in the owner's feed: "Winter coats are selling 40% slower than expected. Recommend a 15% discount to clear inventory. [Approve Discount] [Edit Details]"
  - **Service Yield Management:** Leo sees a card: "Your 5 PM - 7 PM slots are consistently booked out weeks in advance. Recommend increasing the price of these premium slots by $10. [Apply New Pricing]"
  - **Pricing Dashboard (Advanced):** A simple view showing current active dynamic rules (e.g., "Peak Hour Surge: Active", "Clearance Mode: Active").

  ### Key Design Decisions
  - **Rule-Based Execution Engine:** The AI Agent doesn't update database prices directly for every transaction. Instead, it generates structured *rules* (e.g., `IF time_of_day BETWEEN 17:00 AND 19:00 THEN price = base_price * 1.2`) that are evaluated quickly at the edge or during checkout.
  - **Approval-First Workflow:** To build trust, significant pricing changes or broad promotions must be explicitly approved by the owner via the 1-tap feed interface. Minor, boundary-constrained optimizations can be fully autonomous if the user opts in.

  ## Implementation Prompt
  Implement the Autonomous AI Dynamic Pricing & Yield Management Engine.
  1.  **Rule Engine:** Develop a high-performance rule engine that can evaluate dynamic pricing rules (time-based, inventory-based) during the checkout and storefront rendering processes.
  2.  **Data Ingestion & Analysis:** Create a background worker (part of the Sales/Advisory Agent) that periodically analyzes sales velocity, booking density, and inventory levels.
  3.  **Agentic Proposals:** Implement the logic for the AI Agent to generate actionable pricing and promotion proposals based on the analysis, and push these to the user's Unified Agent Feed.
  4.  **Database Schema:** Design the necessary tables to store pricing rules, historical price changes, and AI proposals. Ensure strict multi-tenant isolation.
  5.  **Integration:** Ensure the dynamic pricing seamlessly integrates with the existing catalog and booking mesh.

  ## Priority
  P2

  ## Estimated Scope
  Medium
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
