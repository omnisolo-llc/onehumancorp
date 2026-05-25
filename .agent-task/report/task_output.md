issue_title: "[Finance] Autonomous Plain Language Profit Analyzer"
issue_description: |
  # Title
  Autonomous Plain Language Profit Analyzer

  ## Problem Statement
  Small business owners (like Maya the baker and Carlos the handyman) frequently suffer from "Financial Fog." They conflate top-line revenue with bottom-line profit because traditional accounting tools (like QuickBooks or Xero) and standard platform dashboards are riddled with technical jargon (P&L, EBITDA, COGS, Accruals) and require manual data entry. They need an invisible financial teammate—The Accountant—that autonomously tracks expenses, COGS, and revenue in the background and delivers simple, plain-language profitability insights directly to their phone.

  ## Research Report
  - **Market Data**: "Financial Fog" is a top 10 SMB pain point. Solopreneurs often discover they are losing money only at tax time.
  - **Competitor Landscape**:
    - *Shopify/Wix*: Dashboards focus heavily on top-line GMV/revenue. True profit calculation requires expensive third-party apps or exporting data to spreadsheets.
    - *QuickBooks*: Powerful but designed for CPAs, not a food cart operator. High setup complexity and operational fatigue.
  - **The Gap**: No platform offers a proactive, conversational financial brief that tells a merchant exactly how much *actual* money they made today without making them read a balance sheet.
  - **Opportunity**: By leveraging OHC's Unified Ledger and AI Agent Departments, we can fully automate the categorization of expenses and calculation of COGS, presenting the owner with a single, jargon-free daily metric: "True Profit."

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant OHC_Ledger as Universal Ledger
      participant ExpenseEngine as Expense & Receipt Engine
      participant FinanceAgent as The Accountant (AI)
      participant AdvisorAgent as Business Advisor (AI)
      participant MobileUI as OHC Dashboard (375px)

      OHC_Ledger->>FinanceAgent: Real-time Revenue & COGS Events
      ExpenseEngine->>FinanceAgent: Auto-categorized Expenses
      FinanceAgent->>FinanceAgent: Calculate True Profit & Margins
      FinanceAgent->>AdvisorAgent: Detect anomalies (e.g., dropping margins)
      AdvisorAgent->>MobileUI: Generate Plain Language Daily Brief
      MobileUI->>MobileUI: Display "You made $150 profit today"
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  1. **The Daily Briefing Card**:
     - A Translucent Glass card at the top of the mobile dashboard.
     - **Headline**: "You made $150 in profit today."
     - **Subtext**: "Revenue was $200, minus $50 in ingredient costs."
     - **Interaction**: Tapping the card expands to show a simple visual breakdown (a color-coded bar chart) of money in vs. money out. No "P&L Statement" tables.
  2. **Actionable Insights Modal**:
     - An alert from The Advisor: "Your margin on Vegan Cakes dropped by 5% this month. Want me to suggest new pricing?"
     - **Buttons**: `[View Suggestion]` `[Dismiss]`

  ### AI Agent Integration Points
  - **The Accountant (Finance Dept)**: Continuously monitors the `Universal Ledger`, cross-referencing sales against inventory costs and operational expenses to calculate real-time profitability.
  - **The Business Advisor (Strategy Dept)**: Translates raw financial data from The Accountant into plain, conversational English and surfaces actionable insights to the merchant.

  ### Key Design Decisions
  - **Zero-Jargon Policy**: Terms like "Cost of Goods Sold" or "EBITDA" are banned from the primary UI. We use "Money In", "Money Out", and "Profit". Advanced accounting views are hidden behind an "Export to CPA" feature.
  - **Proactive vs Reactive**: Instead of waiting for the user to pull a report, the AI pushes a summarized brief at the end of every business day.
  - **Multi-Tenant Isolation**: Financial data is strictly partitioned. The Finance Agent must use SPIFFE/SPIRE identity tokens to access only the ledger of the specific tenant it is analyzing.

  ## Implementation Prompt
  **To Implementer Agent:**
  Implement the Autonomous Plain Language Profit Analyzer.
  1. Create a background service (The Accountant AI Agent) that listens to the `Universal Ledger` for new revenue events and the `Expense Engine` for new costs.
  2. Develop the logic to calculate real-time "True Profit" (Revenue - (COGS + Expenses)).
  3. Build the "Business Advisor" component that takes these metrics and generates a plain-language summary string.
  4. Implement the Daily Briefing Card in the mobile UI (375px optimized, following the macOS Translucent Glass design system) to display this insight.
  Ensure strict tenant data isolation and abstract all complex accounting terminology away from the user-facing interface.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []