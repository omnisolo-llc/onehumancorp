issue_title: "[Architecture] Autonomous Predictive Inventory & Restock Engine"
issue_description: |
  # Research Report: Autonomous Predictive Inventory & Restock Engine

  ## Findings
  Small business owners like Priya (boutique owner) and Maya (baker) constantly run out of critical supplies or popular variants without realizing it until a customer tries to buy them. Existing platforms (Shopify, Wix) only track what is currently in stock, but they don't predict when you will run out based on sales velocity, seasonality, or upcoming local events, nor do they automate the restocking process. For a solopreneur, stockouts mean lost revenue and manual inventory management is a time-consuming nightmare.

  *   **Shopify**: Provides basic low-stock alerts and manual purchase orders (PO), but relies on third-party apps (like Stocky) for demand forecasting, which are complex and expensive for micro-businesses.
  *   **Wix / Squarespace**: Very basic inventory tracking. No predictive capabilities out of the box. Users are forced to manually update numbers and remember to reorder.
  *   **Square**: Good real-time tracking, but predictive ordering requires upgrading to expensive retail tiers or using complex integrations.

  ## Proposed Next Steps
  Implement an Autonomous Predictive Inventory & Restock Engine. The system will track sales velocity and predict stockouts. The AI Operations Agent will draft automated supplier reorder requests for a "1-Tap Approval" by the business owner on their mobile device.

  See the full design doc at `docs/research/[architecture]_autonomous_predictive_inventory_and_restock_engine.md` for architecture diagrams and UX flows.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []