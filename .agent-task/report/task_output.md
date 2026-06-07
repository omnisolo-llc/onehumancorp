issue_title: "Implement The Analyst Agent: Proactive Plain-Language Financial Summaries & Anomaly Detection"
issue_description: |
  # The Analyst Agent: Proactive Plain-Language Financial Summaries & Anomaly Detection

  ## Problem Statement
  Small business owners (like Priya the Boutique Operator or Jun the Location Manager) are overwhelmed by traditional analytics dashboards. They do not have the time or technical background to interpret charts, conversion funnels, or balance sheets. When revenue drops or anomalies occur (e.g., a spike in pickup complaints, uncollected deposits), they often discover it too late. Existing platforms (Shopify, Wix) provide passive dashboards that require the owner to log in and interpret the data themselves.

  ## Research Report
  - **Competitive Analysis:**
    - **Shopify:** Provides robust analytics and reports, but relies on passive dashboards. The owner must actively seek out the information.
    - **Wix/Squarespace:** Similar passive analytics approach.
    - **Square:** Good daily sales summary emails, but lacks context on other operations (e.g., "You have 3 missed leads").
  - **OHC Opportunity:** OHC must shift from "dashboards" to "conversations." The Finance & Decision Assistant (The Analyst) should continuously monitor the PostgreSQL database and Stripe webhooks, identifying trends and anomalies. Instead of showing a chart, it pushes a plain-language summary to the owner's mobile feed.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[PostgreSQL Ledger] -->|Replication/Polling| B(Analytics Aggregator)
      C[Stripe Webhooks] --> B
      D[Operations/Booking Events] --> B
      B --> E{Anomaly & Trend Detection Engine}
      E -->|Significant Insight| F[The Analyst Agent LLM]
      F -->|Draft Plain-Language Summary| G[Mobile Unified Feed]
      G -->|Push Notification| H(Owner's 375px Device)
      G -->|1-Tap Action| I[Operations Agent e.g. Send Reminders]
  ```

  ### Mobile UX Flow
  1. **Notification:** Priya receives a push notification at 5:00 PM: "Daily Summary: Sales are up 15%, but 2 invoices are overdue."
  2. **Feed View (375px):** Priya opens the OHC app. The top card in the Agent Feed is the "Daily Insight" card.
  3. **Content:** "You made $450 today! However, Carlos and Sarah haven't paid their deposits for tomorrow's bookings."
  4. **Action:** A primary button reads "Send Payment Reminders". Tapping it triggers the Customer Success Agent to dispatch friendly SMS/Email reminders.

  ### AI Agent Integration Points
  - **The Analyst Agent:** Triggered by daily CRON jobs or anomaly thresholds (e.g., 20% drop in daily average revenue). Uses an LLM to translate raw SQL aggregations into empathetic, plain-language insights.
  - **Handoffs:** The Analyst can propose actions that hand off to the Sales Agent (drafting a promo) or Customer Agent (sending reminders).

  ### Key Design Decisions
  - **No Charts by Default:** The primary interface is text. Charts are only shown if the user explicitly taps "Show Data."
  - **Action-Oriented:** Every summary must conclude with a recommended next step or an explicitly stated "No action needed."
  - **Multi-Tenant Isolation:** The Analytics Aggregator must strictly respect `tenant_id` boundaries.

  ## Implementation Prompt
  **User-Facing Outcome:** As an owner, I want a daily plain-language summary of my business health and immediate notification of any financial anomalies (e.g., unpaid deposits), so I know exactly what actions to take without looking at a dashboard.

  **CUJ & Acceptance Criteria:**
  1. Create a background worker that aggregates daily transaction and booking data per tenant.
  2. Implement The Analyst Agent to process this aggregated data using the LLM provider (Gemini/MiniMax) and generate a plain-language insight.
  3. If there are overdue invoices, the insight must include a recommended action to send reminders.
  4. The summary is pushed to the tenant's Mobile Agent Feed.
  5. Provide Playwright E2E tests: A user logs in, sees the Daily Insight card in their feed, and clicks "Send Reminders" which successfully invokes the reminder workflow. No actual charts should be required for this CUJ.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
