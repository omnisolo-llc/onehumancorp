issue_title: "Implement 'The Analyst' Agent - Proactive Insights for Small Businesses"
issue_description: |
  # Research Report: "The Analyst" Agent - Proactive Insights for Small Businesses

  ## Target Persona: Priya (Boutique Owner) and Maya (Home Baker)

  ## 1. Problem Statement
  Small business owners often lack the time and expertise to dive into complex analytics dashboards. They need actionable insights pushed to them, not raw data they have to pull and interpret. Existing platforms offer dashboards, but they require the owner to actively seek out the information and figure out what to do with it.

  *Persona Need:* Priya needs to know which products are driving the most revenue this week so she can reorder or feature them. Maya needs to know if her Instagram engagement is actually translating into cake orders.

  ## 2. Research & Gap Analysis (Track 1 & Track 3)
  - **Competitor Landscape**: Tools like Shopify provide extensive analytics but require the user to navigate to the "Analytics" tab, select date ranges, and interpret charts. Google Analytics is too complex for most micro-SMBs.
  - **The OHC Opportunity**: We can leverage the existing `Agent Feed` and our unified data models (orders, inventory, messages) to push *plain-language* insights directly to the owner's mobile feed.
  - **The Gap**: We currently have "The Ambassador" (Customer Success) and "The Promoter" (Marketing), but we lack a dedicated "Analyst" agent that proactively summarizes performance and suggests strategic actions based on data.

  ## 3. Architecture & Design Flow (Track 2 & Track 3)
  - **Data Ingestion**: A scheduled asynchronous worker (e.g., running weekly or daily) that aggregates data from the PostgreSQL ledger (orders, revenue, top-selling items).
  - **Processing Layer (LLM)**: An intent/summarization LLM prompt that takes the raw JSON aggregated data and turns it into a concise, owner-friendly insight.
  - **Actionable Output**: The Agent must not just report data; it must suggest an action. (e.g., "The Red Dress sold out twice as fast this week. Should I draft a restock order?" or "Revenue is up 15% but web traffic is down. Should I run a quick Instagram promo?").
  - **Mobile UX (375px)**: The insight is pushed as a "Card" to the `Agent Feed` (via `agent_feed_items`). It must include a clear summary and a 1-tap action button (e.g., "Draft Restock" or "Dismiss").

  ### Mermaid Diagram - Architecture Sequence
  ```mermaid
  sequenceDiagram
      participant Cron as Scheduled Job Trigger
      participant AnalystWorker as The Analyst Worker
      participant DB as PostgreSQL (Ledger)
      participant LLM as AI/LLM Provider
      participant AgentFeed as Mobile Agent Feed
      participant Owner as Business Owner (Mobile)

      Cron->>AnalystWorker: Trigger Weekly Analysis
      AnalystWorker->>DB: Fetch Sales/Revenue Stats for Tenant
      DB-->>AnalystWorker: Return Raw JSON Data
      AnalystWorker->>LLM: Pass Data + Prompt (Analyze & Suggest Action)
      LLM-->>AnalystWorker: Plain Language Insight & Action Suggestion
      AnalystWorker->>DB: Save Insight as Feed Card (`agent_feed_items`)
      AnalystWorker->>AgentFeed: Push Notification
      AgentFeed-->>Owner: Display Card on 375px Viewport
      Owner->>AgentFeed: Tap 1-Click Action (e.g., "Restock Item")
  ```

  ## 4. Implementation Prompt (For Engineering Swarm)
  **Feature Name**: The Analyst Agent - Proactive Insights Feed
  **Target Persona**: Priya (Boutique Owner)

  **Outcome**: Priya receives a weekly mobile push notification summarizing her top products and overall revenue trend, with a 1-tap suggested action if relevant.

  **Critical User Journey (CUJ)**:
  1. The system's cron job triggers the Analyst Worker.
  2. The Worker aggregates weekly sales data for Priya's tenant.
  3. The LLM generates a concise insight (e.g., "Weekly Summary: Revenue up 10%. 'Summer Hat' is your top seller.").
  4. The insight is saved to the `agent_feed_items` table.
  5. Priya opens the OHC mobile app (375px view) and sees the new Analyst Card in her feed.
  6. She taps "Dismiss" (or an action if suggested) and the card is marked as processed.

  **Next Actions for Engineering**:
  - **Step 1**: Create the `AnalystWorker` (similar pattern to `MessageTriageWorker`) that can be triggered on a schedule to query basic order/revenue stats for a tenant.
  - **Step 2**: Implement the LLM prompt to convert raw stats into a plain-language summary.
  - **Step 3**: Insert the resulting insight into the existing `agent_feed_items` (or `daily_work_items`) system so it appears on the frontend feed.
  - **Step 4**: Add basic Playwright E2E tests verifying the card appears in the feed.

  ## Priority & Scope
  - **Priority**: P1
  - **Estimated Scope**: Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
