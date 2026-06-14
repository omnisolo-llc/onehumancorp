issue_title: "[Research] OHC Autonomous Decision Assistant & Daily Briefing Architecture"
issue_description: |
  ## Problem Statement
  Small business owners and operators (like Priya the boutique owner or Jun the location manager) lack the time or analytical background to parse complex dashboards and raw data. Traditional systems (e.g., Shopify Analytics, Google Analytics) offer charts and metrics but fail to provide actionable context—they don’t tell the owner *what the data means* or *what to do next*. They are reactive, not proactive. Owners need an assistant that synthesizes performance, identifies anomalies (e.g., "vegan orders spiked yesterday," "pickup times are slowing down"), and suggests concrete next steps in plain language.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Shopify / Wix / BigCommerce Analytics:** Focus heavily on dashboards, graphs, and tabular data. They expect the user to act as their own data analyst. They do not natively summarize daily performance into a plain-text "morning briefing" or suggest operational changes based on anomalies.
  - **Notion AI / Microsoft Copilot:** Great at generic summarization but disconnected from operational business data (sales, inventory, appointments).
  - **OHC Opportunity:** The OHC "Decision & Reporting Assistant" can fill this void by generating a daily, plain-language business summary (the "Morning Briefing"). It will consume data from the unified data graph (sales, triage inbox volume, appointment bookings) and produce an actionable feed card. This aligns with the "Full-Spectrum Observability" and "Owner Clarity" values—no technical jargon, just "what happened" and "what to do."

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Unified Customer Graph DB] -->|Daily Sync / Event Stream| B(Analytics Aggregation Engine)
      C[Operational Systems: Sales, Inbox, Bookings] -->|Metrics| B
      B --> D[The Decision Assistant Agent]
      D -->|Query & Summarize| E[LLM Provider - Gemini/GPT]
      E -->|Generate Briefing| F[Action Required Queue]
      F --> G[Mobile App Feed 375px - Work Triage]
      G -->|Owner Review| H[Suggested Action Execution]
  ```

  ### Mobile UX Flow (375px First)
  1. **Morning Feed:** Owner opens the OHC app. At the top of the Work Triage feed is a "Daily Briefing" card.
  2. **Plain Language Summary:** The card highlights 2-3 key bullet points (e.g., "Sales are up 15% this week. 3 new custom cake inquiries waiting in triage.").
  3. **Anomaly / Actionable Insight:** A highlighted section notes a trend or anomaly (e.g., "Insight: 4 missed calls yesterday during lunch hour. Suggestion: Update automated response for missed calls.").
  4. **1-Tap Action:** A button underneath the insight allows the owner to instantly approve the suggested action (e.g., "Draft new auto-reply").

  ### AI Agent Integration Points
  - **Decision Assistant Agent:** Runs on a scheduled CRON or triggered daily event. It pulls aggregated metrics for the tenant from the previous 24 hours.
  - **Contextual Memory:** It uses the tenant-scoped memory to understand what the owner cares about (e.g., if Maya is a baker, it focuses on order volume and custom inquiries).
  - **Work Triage Integration:** The generated briefing is pushed into the `Work Triage` queue as a high-priority informational item, ensuring it's the first thing the owner sees.

  ## Implementation Prompt
  **Goal:** Implement the backend scheduled job and the AI agent logic to generate a plain-language daily business summary for a tenant, and expose it to the Work Triage feed.

  **CUJ (Critical User Journey):**
  1. The system aggregates daily metrics for a business (sales, new messages, bookings).
  2. The Decision Assistant Agent synthesizes this data into a short, readable summary with at least one actionable insight.
  3. This summary is injected into the Work Triage feed as a "Daily Briefing" card.
  4. The owner (e.g., Priya) opens the app, reads the plain-language summary, and understands the business health without looking at a chart.

  **Acceptance Criteria:**
  - Create a new background worker/job (e.g., `daily_briefing_worker`) that triggers the Decision Assistant Agent.
  - The agent must use the LLM to generate a summary based on dummy or aggregated tenant data.
  - The output must be formatted as a Work Triage item and saved to the database.
  - Expose this triage item through the existing Work Triage API/UI.
  - Provide complete unit and E2E Playwright tests verifying the generation and display of the briefing card.

  ## Priority
  `P1`

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
