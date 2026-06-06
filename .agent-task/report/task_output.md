issue_title: "Implement 'The Advisor' Agent: Weekly Insights & Next Actions Feature"
issue_description: |
  # Research Report & Design Brief: "The Advisor" Agent - Weekly Insights & Next Actions

  ## 1. Problem Statement
  Small business owners often launch their store and then face the "Now What?" syndrome (Pain Point #9). They have limited time to sift through complex analytics dashboards to understand their business performance. They need proactive, plain-language insights and actionable recommendations delivered directly to their mobile device to help them grow and manage their business effectively.

  ## 2. Research Report
  - **Market Context**: Traditional platforms like Shopify or Wix provide analytics dashboards that require the user to proactively log in, analyze charts, and derive their own conclusions. This is often overwhelming for non-technical users (e.g., Maya the Baker, Fatima the Food Cart Operator).
  - **The OHC Opportunity**: By introducing "The Advisor" (Business Advisory Agent), OHC can autonomously analyze weekly business data and push a synthesized, easy-to-understand summary to the user's mobile feed, along with 1-tap actionable suggestions.
  - **Differentiation**: Moving from a "pull" model (dashboards) to a "push" model (proactive agentic insights).

  ## 3. Design Doc
  ### Architecture
  - **Data Aggregation**: A scheduled background job (CRON) running every Friday at 5 PM (user's local time) aggregates weekly metrics (revenue, top-selling items, low stock, traffic trends) from the PostgreSQL database for each tenant.
  - **LLM Processing**: The aggregated data is fed into the LLM (Gemini) with a prompt instructing it to act as a Business Advisor and generate a friendly, plain-language summary and a specific, actionable recommendation (e.g., "Run a 10% sale on X", "Restock Y").
  - **Delivery**: The generated insight is published as an "Action Card" to the user's Agent Feed and triggers a mobile push notification.

  ### Mobile UX Flow (375px)
  1. **Notification**: User receives a push notification: "Your Weekly Business Report is ready! See how you did."
  2. **Agent Feed**: Tapping the notification opens the Agent Feed showing a new card from "The Advisor".
  3. **Insight Card**: The card displays a brief summary (e.g., "Great week! You made $450. Your top seller was the Vegan Chocolate Cake.")
  4. **Actionable Suggestion**: Below the summary, a suggestion is presented (e.g., "Suggestion: You have 0 Vegan Chocolate Cakes left. Create a restock order?") with "Approve" (creates draft order) or "Dismiss" buttons.

  ## 4. Implementation Prompt
  **Feature Name**: Weekly Business Insights ("The Advisor" Agent)
  **Target Persona**: Fatima the Food Cart Operator
  **Outcome**: Fatima receives a simple weekly summary of her top-selling items and revenue, with an actionable suggestion (e.g., to prepare more of a sold-out item for next week), without having to read a complex dashboard.

  **Next Actions**:
  1. Create the backend scheduling mechanism (CRON/worker) to aggregate weekly tenant data (orders, revenue, inventory).
  2. Implement the LLM prompt and integration to generate the plain-language summary and action recommendation based on the aggregated data.
  3. Design and implement the "Insight Action Card" UI for the mobile Agent Feed.
  4. Ensure end-to-end functionality is covered by Playwright E2E tests simulating the weekly trigger and user interaction.

  **Priority**: P1
  **Estimated Scope**: Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
