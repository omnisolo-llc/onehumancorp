issue_title: "Implement Autonomous Cash Flow Prediction & Tax Reserve Engine (The CFO Agent)"
issue_description: |
  ## Title
  Implement Autonomous Cash Flow Prediction & Tax Reserve Engine (The CFO Agent)

  ## Problem Statement
  Small business owners (like Maya the Baker and Carlos the Handyman) frequently suffer from "cash flow panic" and "tax time terror." Traditional platforms (Shopify, Wix) provide historical revenue dashboards but fail to proactively predict future cash crunches based on upcoming liabilities (e.g., subscription renewals, supply purchases). Furthermore, non-technical owners often forget to withhold estimated taxes on their daily sales, leading to massive liabilities at year-end. They do not need complex accounting jargon; they need an invisible CFO Agent that tells them exactly what their safe spending limit is today.

  ## Research Report
  **Market Context & Competitor Analysis**
  - **Traditional SMB Platforms:** Shopify, Wix, and Squarespace focus exclusively on revenue capture. They leave expense tracking and cash flow forecasting to third-party integrations (like QuickBooks Online or Xero).
  - **Accounting Tools:** Tools like QuickBooks or Xero require accounting knowledge (chart of accounts, reconciliation, accrual vs. cash basis). The cognitive load is too high for micro-SMBs and operators who run their business entirely from a 375px mobile screen.
  - **Data Insights:** According to U.S. Bank studies, 82% of small businesses fail due to poor cash flow management. The gap in the market is "Actionable Predictive Finance" — moving from historical reporting to forward-looking AI alerts.
  - **Dogfooding & Gap Observation:** In reviewing the current OHC `ledger.rs` and `capital.rs` architecture, the system tracks current balances and completed transactions well. However, it lacks a predictive time-series projection and automated tax-withholding allocations. The dashboard shows "Total Revenue," but a real owner needs to see "Safe to Spend" (Total Revenue - Upcoming Subscriptions - Estimated Tax).

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Stripe/Terminal Webhook] --> B[OHC API Gateway]
      B --> C[Payment Ledger]
      C --> D[CFO Agent - Cash Flow Engine]
      D --> E{Tax & Expense Allocation}
      E -->|Estimated Taxes| F[Virtual Tax Reserve DB]
      E -->|Upcoming Subscriptions| G[Liability Tracker]
      F --> H[Operations Feed / Mobile App]
      G --> H
      D --> I[LLM Intent / Summary Generator]
      I --> H[Actionable Push Notifications]
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **Home Dashboard (Command Center):**
    - Replaces the static "Revenue: $1,200" metric with a unified "Safe to Spend: $850" UniFi-style translucent glass card.
    - **Tap Interaction:** Tapping the card opens the CFO breakdown screen.
  - **CFO Breakdown Screen:**
    - A clean, vertically scrolling layout (max width 375px).
    - Top section: Large typography showing "Safe to Spend".
    - Middle section: "Reserved for Taxes: $200" (with an info icon explaining the automated 15% withholding rule).
    - Bottom section: "Upcoming Bills (Next 7 Days): $150 (Shopify, Mailchimp)".
  - **Agent Notification Flow:**
    - The CFO Agent monitors real-time balances. If Maya tries to purchase $900 of baking supplies when her "Safe to Spend" is $850, the agent pushes an actionable alert: "Heads up! A $900 purchase will cut into your tax reserve. Proceed anyway?"

  ### AI Agent Integration Points
  - **CFO Agent Prompt:** The system prompt for the `Finance & Decision Assistant` will be expanded to ingest a structured 30-day ledger projection.
  - **Agent Memory:** The agent will retain tenant-specific tax rates (e.g., 15% self-employment reserve) and recurring expense identifiers to automatically classify incoming/outgoing funds.
  - **Action Router:** The CFO agent connects to the `action_router.rs` to generate and persist daily "Cash Flow Health" plain-language summaries to the `seo_discovery_reports` or a new `financial_reports` table.

  ### Key Design Decisions
  - **Virtual Envelopes:** Instead of creating complex multi-account bank integrations right away, we implement "Virtual Envelopes" (Reserves) at the DB ledger level (using PostgreSQL row-level security for tenant isolation).
  - **Zero-Config Rule:** The user is not asked to enter their tax rate during onboarding. The CFO Agent defaults to a conservative 15% estimated reserve, which the user can tweak later via natural language ("Agent, lower my tax reserve to 10%").
  - **No Mock Data in UI:** The Flutter/PWA frontend must derive the "Safe to Spend" metric exclusively from the real PostgreSQL ledger aggregations.

  ## Implementation Prompt
  Implement the Autonomous Cash Flow Prediction & Tax Reserve Engine.
  1. Add a Virtual Reserve schema to the Ledger domain to track "Tax" and "Upcoming Liability" envelopes.
  2. Extend the CFO Agent (Finance & Decision Assistant) to intercept incoming payment webhooks, calculate a default 15% tax allocation, and deduct it from the available balance.
  3. Create an API endpoint (`/api/finance/safe-to-spend`) that returns the aggregated metrics (Current Balance, Tax Reserve, Upcoming Liabilities, Safe to Spend).
  4. Write comprehensive unit tests for the ledger math and E2E Playwright tests verifying that the Mobile Dashboard correctly displays the "Safe to Spend" card after a successful Stripe checkout session. Do not mock the database; use the E2E seed mechanisms.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
