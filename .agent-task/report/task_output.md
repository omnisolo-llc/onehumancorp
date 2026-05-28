issue_title: "Implement Invisible Fractional CFO and Cashflow Engine"
issue_description: |
  # Invisible Fractional CFO and Cashflow Engine

  ## Problem
  Small business owners lack the expertise for traditional accounting. They need proactive, invisible financial intelligence that predicts cash flow crunches and automates tax set-asides, rather than just historical reporting.

  ## Research Findings
  - Existing platforms (QuickBooks, Shopify) require manual work or only offer historical data.
  - A significant gap exists in proactive cash flow forecasting and automated virtual bucket segregation for taxes/expenses.
  - AI agents can bridge this gap by interpreting ledgers and calendar events to provide plain-language advice.

  ## Proposed Next Steps
  1. Build multi-tenant virtual account logic for automatic revenue sweeps (e.g., 15% to Tax bucket).
  2. Implement a background worker for 30-day cash flow predictions based on ledger and calendar data.
  3. Integrate the Finance Agent with the Unified Inbox for proactive alerts.
  4. Design the "Financial Health" mobile UI card using the UniFi modular design system.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
