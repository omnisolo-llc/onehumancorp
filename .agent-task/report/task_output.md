issue_title: "Automated Cart Recovery via Agents"
issue_description: |
  # Automated Cart Recovery via Operations & Ambassador Agents

  ## Problem Statement
  Small business owners lose up to 70% of potential sales due to cart abandonment. Following up on abandoned carts is often a premium feature on legacy platforms (Shopify) or requires third-party plugins. For users like Priya (Boutique Owner) and Maya (Home Baker), recovering these sales manually is too time-consuming, and configuring complex marketing flows is outside their technical comfort zone.

  ## Research Report
  - **Competitor Landscape**:
    - Shopify provides abandoned checkout emails, but advanced SMS or multi-channel flows require apps like Klaviyo, which are complex and expensive.
    - Wix and Squarespace offer basic recovery emails, but they lack native integration with AI to personalize the outreach based on user history or inventory scarcity.
  - **OHC Opportunity**: OHC can automate cart recovery entirely through its AI agents without requiring any configuration from the business owner.
    - "The Salesperson" or "The Ambassador" agents can detect when an item is left in the cart.
    - The agent can draft a personalized, contextual message (e.g., "Hi! Looks like you left the vegan cake in your cart. Only 2 slots left for this weekend!").
    - The business owner simply approves the drafted message from their mobile feed, or the system can be set to auto-send based on pre-approved rules.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Cart Update Event] -->|Timeout 1 hr| B(Operations Agent)
      B --> C{Determine Context}
      C -->|High Value/Low Stock| D[Draft Urgent SMS/Email]
      C -->|Standard| E[Draft Reminder Email]
      D --> F[Agent Feed: Approval Card]
      E --> F
      F -->|Owner Approves| G[Send Communication]
  ```

  ### Mobile UX Flow
  1. **Agent Feed (375px)**: An Action Card appears: "Recover Cart: $120. Drafted message: 'Hey, still interested in the Red Dress? We saved it for you!'"
  2. **Action Buttons**: "Send Now", "Edit", "Dismiss".
  3. **Settings**: A toggle in the "Ambassador" settings to "Auto-send cart recovery messages after 1 hour."

  ### AI Agent Integration
  - **The Operations Agent**: Monitors cart sessions and triggers events when a cart is abandoned.
  - **The Ambassador Agent**: Drafts the contextual message based on the cart contents and customer history.

  ## Implementation Prompt
  Implement the backend event pipeline to detect abandoned carts (e.g., no checkout action within 1 hour of adding to cart). Integrate this with the Agent Feed so that "The Ambassador" drafts a personalized recovery message (email/SMS) and presents it to the business owner as an Action Card on their mobile dashboard. Include a setting to fully automate sending.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
