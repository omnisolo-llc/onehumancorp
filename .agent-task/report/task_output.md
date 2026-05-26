issue_title: "[Architecture] Autonomous Predictive Cashflow Engine"
issue_description: |
  # Research Report: Autonomous Predictive Cashflow Engine

  The #1 reason small businesses fail is cash flow mismanagement. Solopreneurs often have a full pipeline of bookings or unpaid invoices, but struggle to pay immediate expenses while waiting 30-90 days for client payouts.

  ## Findings
  - **Competitor Landscape**:
    - Shopify offers "Shopify Capital," but it's typically reactive, based on historical aggregate sales, and opaque in its underwriting.
    - Wix/Squarespace lack native working capital solutions completely.
    - GoDaddy focuses on processing payments rather than predicting cash flow shortages.
    - QuickBooks has predictive cash flow tools but requires extensive manual categorization and isn't actionable instantly from a mobile phone without a lengthy loan application.
  - **User Needs**: Users don't want to apply for loans. They want the platform to say: "You have $2,000 in upcoming expenses this week, but your $3,500 invoice won't clear until next Friday. Tap here to get a $1,500 instant advance against that invoice."
  - **AI Differentiation**: Instead of passive dashboards, OHC's Finance Agent actively monitors connected bank feeds, upcoming calendar bookings, and sent invoices. It predicts cash flow valleys *before* they happen and offers 1-tap, risk-assessed micro-advances invisibly integrated into the daily workflow.

  ## Proposed Next Steps
  We have mapped out the architectural design for this feature and created a detailed issue brief at `docs/research/[architecture]_autonomous_predictive_cashflow_engine.md`. The implementer swarm should follow the design doc to build the background worker and mobile UX flow.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []