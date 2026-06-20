issue_title: "The CFO Agent: Proactive Cash Flow & Expense Forecasting"
issue_description: |
  # Research Report: The CFO Agent - Proactive Cash Flow & Expense Forecasting

  ## Problem Statement
  Small business owners (such as Carlos the Handyman or Maya the Home Baker) constantly juggle cash flow. Traditional accounting software like QuickBooks or Xero act as retrospective ledgers—they tell the owner what happened last month. However, micro-SMBs operate on thin margins and need predictive insights. A missed invoice payment or an unexpected subscription renewal can cause an overdraft, yet monitoring upcoming expenses, pending payouts, and low balances requires manual reconciliation across banks, Stripe, and spreadsheets.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Shopify/Wix:** Offer basic reporting and sales dashboards. They show incoming revenue but do not integrate deeply with outbound cash flow, business bank accounts, or upcoming recurring expenses (e.g., software subscriptions, rent).
  - **QuickBooks/Xero:** Robust for accountants, but overwhelming for solopreneurs. These tools require manual categorization and reconciliation to provide accurate forecasts, which non-technical operators often neglect.
  - **Stripe Dashboard:** Excellent for revenue, but does not provide a holistic cash flow picture that includes external expenses or manual cash operations.
  - **OHC Opportunity:** Leverage our "AI Teammate" philosophy to introduce "The CFO Agent" (Finance & Decision Assistant). Instead of presenting complex dashboards, The CFO Agent continuously monitors the Ledger, Stripe connected accounts, and recurring billing schedules to predict cash crunches. It pushes actionable alerts to the owner's 375px feed (e.g., "Warning: Rent is due in 3 days, but your payout is delayed. Would you like me to send a reminder to Nora for her overdue invoice?").

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Ledger / Invoices DB] -->|Event Stream| B(The CFO Agent)
      C[Stripe Payouts API] -->|Webhook| B
      D[Recurring Expenses Store] -->|Sync| B
      B -->|Cash Flow ML/Heuristic Model| E{Anomaly/Crunch Detected?}
      E -- Yes --> F[Action Required Queue]
      F --> G[Mobile App Feed 375px]
      G -->|1-Tap Action| H[e.g., Send Invoice Reminder]
      G -->|1-Tap Action| I[e.g., Delay Vendor Payment]
  ```

  ### Mobile UX Flow (375px First)
  - **Home Feed (Mobile):** A prominent Glassmorphism card appears: "⚠️ Potential Cash Crunch".
  - **Interaction:** Tapping the card opens a detail view. The top half shows a simple bar chart of the next 7 days (expected cash in vs. expected cash out). The bottom half explains the issue in plain language: "You have $1,200 in expenses due by Friday, but only $800 in confirmed payouts."
  - **Action:** A section titled "Suggested Actions":
    - Button 1: "Send reminders for $450 in overdue invoices"
    - Button 2: "Pause $50 Canva subscription"
  - **Visual Design:** Clean Apple/Ubiquiti-style hierarchy. The warning uses the `#FF9500` (Warning) token. The actions use native buttons `rounded-[8px]`.

  ### AI Agent Integration Points
  - **The CFO Agent (Finance):** Uses Gemini Pro to analyze the raw ledger and payout data, formatting it into a plain-language summary and formulating the "Suggested Actions".
  - **The Manager (Operations):** Interacts with The CFO Agent to execute actions, such as sending reminders or interacting with third-party APIs to pause subscriptions (if supported).

  ## Implementation Prompt
  Implement the core logic for "The CFO Agent" to monitor upcoming cash flows.
  - Create a background worker that runs daily (or reacts to major ledger events) to project cash balance for the next 7 and 30 days.
  - Integrate with the existing `invoice.rs` and `ledger.rs` systems to identify overdue or pending incoming payments.
  - If a cash deficit is projected within 7 days, generate a plain-language alert and push an "Action Card" to the user's unified feed.
  - The Action Card must allow the user to 1-tap trigger a follow-up action (e.g., dispatching an email reminder to customers with overdue invoices).

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
