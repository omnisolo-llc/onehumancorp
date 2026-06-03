issue_title: "Unified Promotions and Discounts Engine Architecture"
issue_description: |
  # Unified Promotions and Discounts Engine

  ## Problem Statement
  Small business owners struggle to create, manage, and track promotions across different channels using disjointed tools. They need a single, unified system managed by an AI agent that handles all types of promotions and discounts across their online storefront, in-person POS, and marketing channels, requiring zero technical setup.

  ## Research Report
  - **Current Solutions**: Shopify, Wix, and Squarespace offer discount engines but they are complex or lack advanced features without significant manual configuration.
  - **User Pain Points**: Difficulty setting up specific rules (e.g., BOGO on certain products), managing overlapping discounts, and tracking ROI of campaigns.
  - **The OHC Advantage**: A unified engine entirely managed by AI agents (Marketing and Finance) that translates natural language requests into complex rules, handles conflict resolution, and provides post-sale reporting.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[User Intent via Voice/Chat] --> B(Marketing AI Agent);
      B --> C{Unified Promotions Engine};
      C --> D[Discount Rules Engine];
      C --> E[Conflict Resolution Matrix];
      D --> F(Storefront/Checkout);
      D --> G(In-Person POS);
      F --> H[Cart Evaluation];
      G --> H;
      H --> I(Finance AI Agent);
      I --> J[Analytics & Reporting];
  ```

  ### Mobile UX Flow (375px)
  1.  **Trigger:** User taps "Promotions" on the dashboard or uses the voice assistant: "Set up a 10% discount for new customers."
  2.  **AI Configuration:** The app displays a glassmorphism card summarizing the AI's interpretation: "Create: 'WELCOME10' code. 10% off entire order. Applies to: First-time buyers only. Start: Now. End: No end date. Correct?"
  3.  **Confirmation:** User taps "Looks good."
  4.  **Reporting:** A unified dashboard view shows active promotions, usage count, and generated revenue.

  ### AI Agent Integration
  - **Marketing (The Promoter):** Translates natural language requests into complex discount rules. Automatically generates marketing copy and social media posts announcing the promotion.
  - **Finance (The Accountant):** Tracks the financial impact of the promotion, calculating the total discount given and the net revenue generated.

  ## Implementation Prompt
  **Role:** Implementer Agent
  **Task:** Build the Unified Promotions and Discounts Engine core logic and data models.

  **Requirements:**
  1.  Design the PostgreSQL schema for storing promotion rules. It must support various types: percentage off, fixed amount off, BOGO, and free shipping. It must also support conditions: minimum purchase amount, specific product/category inclusion/exclusion, and customer tag requirements.
  2.  Implement the evaluation logic (in Go) that takes a cart's contents and a customer's profile and returns the optimal set of applicable discounts, adhering to a defined conflict resolution strategy (e.g., only one code allowed per order, but automatic discounts can combine).
  3.  Expose gRPC endpoints for the Marketing AI agent to create, read, update, and delete promotions.
  4.  Expose an endpoint for the checkout/POS systems to calculate the final price of a cart given a list of applied discount codes.
  5.  Ensure all database interactions enforce strict multi-tenant isolation via `tenant_id`.
  6.  Provide 100% unit test coverage for the evaluation logic, covering edge cases like overlapping rules and minimum purchase thresholds.

  **Priority:** P1
  **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
