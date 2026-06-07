issue_title: "Architect The Advisor: Autonomous Business Advisory & Weekly Health Reporting Engine"
issue_description: |
  # Mission Queue Protocol: The Advisor Agent - Autonomous Business Advisory

  ## 1. Problem Statement
  Small business owners like Maya (Home Baker) and Priya (Boutique Owner) are overwhelmed by complex analytics dashboards. Traditional platforms (Shopify, Wix) provide charts and graphs, but non-technical users struggle to interpret them into actionable business decisions. They do not need more data; they need a personal consultant who tells them *what the data means* and *what to do next*.

  ## 2. Research Report
  - **Competitor Analysis**:
    - *Shopify*: Offers detailed analytics and reporting, but it requires the user to proactively log in, navigate to the analytics tab, and interpret the data themselves.
    - *Wix/Squarespace*: Provide basic traffic and sales overviews, but lack deep, actionable insights.
    - *GoDaddy*: Very basic metrics.
  - **The OHC Opportunity**: Instead of passive dashboards, OHC will deploy **The Advisor** — an AI agent that actively monitors the `ohc_universal_ledger` and `ohc_job_queue`. Every week, it synthesizes the data into a plain-language, conversational health report and pushes it directly to the owner's mobile device.
  - **User Evidence**: Many SMBs abandon their store's analytics page because they suffer from "dashboard fatigue." They want a text message-like summary: "Tuesday was your busiest day. Vegan cakes are trending. Want me to run a promo?"

  ## 3. Design Doc
  ### Architectural Diagram & Data Flow

  ```mermaid
  sequenceDiagram
      participant Cron as Kubernetes CronJob
      participant Advisor as The Advisor Agent
      participant Ledger as ohc_universal_ledger
      participant LLM as Gemini Pro (LLM)
      participant Queue as Notification Queue
      participant App as Flutter Mobile App
      participant Promoter as The Promoter Agent

      Cron->>Advisor: Trigger weekly run (e.g., Friday 5 PM)
      Advisor->>Ledger: Fetch last 7 days sales/refunds data
      Ledger-->>Advisor: JSON aggregated data
      Advisor->>LLM: Generate plain-language summary & action prompt
      LLM-->>Advisor: 3-bullet summary + 1 suggested action
      Advisor->>Queue: Push payload to tenant's device
      Queue->>App: Deliver Push Notification
      App->>App: User views 375px Glassmorphic Card
      alt User taps "Yes, draft it"
          App->>Advisor: Approval webhook
          Advisor->>Promoter: Dispatch job to draft social post
      end
  ```

  - **Cron Trigger**: A Kubernetes CronJob or internal scheduler triggers the Advisor Agent weekly (e.g., Friday at 5 PM local time for each tenant).
  - **Data Aggregation**: The agent queries the `ohc_universal_ledger` for the past 7 days (sales, refunds, booking deposits).
  - **LLM Synthesis**: The aggregated JSON data is passed to the Gemini Pro LLM with a system prompt to act as a supportive, brilliant business consultant. The LLM generates a 3-bullet plain-language summary and 1 actionable suggestion.
  - **Delivery**: The payload is stored in the notification queue and delivered via push notification to the Flutter mobile app.

  ### Mobile UX Flow (375px)
  1. **Notification**: Maya receives a push notification: "Your Weekly Business Health Report is ready. 📈"
  2. **Card View**: Tapping the notification opens a beautifully styled, glassmorphic card on her 375px screen.
  3. **Content**:
     - "You made $450 this week (up 15%!)."
     - "Your top seller was the Custom Vegan Cake."
     - "Action: Want me to draft an Instagram post celebrating your busy week?"
  4. **Action Buttons**: Large touch targets (≥44x44px) for "Yes, draft it!" or "Dismiss".

  ### AI Agent Integration
  - **Cross-Department Coordination**: If the user clicks "Yes, draft it", The Advisor dispatches a job to **The Promoter** (Marketing Agent) to generate the social media post.

  ## 4. Implementation Prompt
  **Target Persona**: Maya the Baker
  **Outcome**: Maya receives a weekly push notification that summarizes her business performance in plain English and offers a 1-tap action to grow her business.

  **Next Actions for Implementer**:
  1. Create a periodic worker routine that triggers the Weekly Health Report for active tenants.
  2. Implement the data aggregation logic to pull weekly revenue and top items from the `ohc_universal_ledger`.
  3. Integrate the Gemini LLM prompt to transform raw ledger stats into a friendly, plain-language summary.
  4. Build the mobile-first (375px) UI card in the dashboard to display the Advisor's weekly report, including 1-tap action buttons.

  ## 5. Priority & Scope
  - **Priority**: P1 (High)
  - **Estimated Scope**: Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
