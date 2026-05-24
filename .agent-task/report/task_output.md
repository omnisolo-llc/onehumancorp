issue_title: "[architecture]_autonomous_smart_working_capital_engine"
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
issue_description: |
  # Title: Autonomous Smart Working Capital & Cash Flow Prediction Engine

  ## Problem Statement
  Small business owners face constant anxiety regarding cash flow. Carlos (Handyman) needs to purchase expensive materials before starting a large renovation, while Maya (Baker) must buy bulk ingredients for wedding season before receiving final payments. Existing platforms like Square or Shopify offer static "loans" that require manual applications or are disconnected from future confirmed bookings and real-time inventory needs. These owners need an invisible, predictive system that proactively offers short-term working capital exactly when a cash flow gap is detected, seamlessly integrated into their daily mobile workflow.

  ## Research Report
  *   **Current Architecture Limits:** OHC currently processes payments and holds a ledger, but lacks predictive intelligence to forecast upcoming expenses versus confirmed future revenue (e.g., booked appointments, pending invoices).
  *   **Competitor Analysis:**
      *   *Square Capital / Shopify Credit:* Offer loans based on historical volume but are reactive. They don't analyze a calendar of upcoming jobs to say "You have a $2,000 job next week, here is $500 for materials now."
      *   *QuickBooks Capital:* Too complex, requires heavy manual bookkeeping to qualify.
  *   **Discovery:** OHC needs an Autonomous Smart Working Capital Engine. By combining the unified capacity/booking mesh, the multi-tenant ledger, and the Finance Agent, OHC can proactively detect cash flow dips and offer instant, micro-advances against future confirmed revenue with one tap.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      FINANCE-AGENT ||--o{ LEDGER : "Monitors Balances"
      FINANCE-AGENT ||--o{ BOOKING-ENGINE : "Analyzes Future Revenue"
      FINANCE-AGENT ||--o{ INVENTORY-MESH : "Predicts Material Costs"
      FINANCE-AGENT }|--|| RISK-EVALUATOR : "Calculates Offer"
      RISK-EVALUATOR ||--o{ MOBILE-APP : "Pushes Advance Offer"
      MOBILE-APP ||--o{ ADVANCE-LEDGER : "1-Tap Accept"
      ADVANCE-LEDGER ||--o{ PAYOUT-WALLET : "Instantly Funds"
  ```

  ### UI Wireframes & Mobile UX Flow (375px)
  *   **Proactive Notification:** Carlos receives a push notification: "You have 3 large jobs next week. Need $400 for materials now? Tap to advance."
  *   **Offer Screen (375px):** A clean, Glassmorphism card detailing the advance. "Get $400 instantly. Repaid automatically from next week's bookings (Fee: $12)."
  *   **1-Tap Acceptance:** A single large "Accept & Fund" button. No forms, no PDF uploads, no complex terms (all handled invisibly based on platform history).
  *   **Success State:** Instant green checkmark. "Funds are in your OHC Wallet. Ready to tap-to-pay for materials."

  ### Key Design Decisions
  *   **Predictive, Not Reactive:** The system relies on the OHC AI Finance Agent constantly running in the background, analyzing the delta between upcoming confirmed bookings and current wallet balance.
  *   **Zero-Friction Acceptance:** No credit checks or manual applications. The offer is pre-approved based on OHC platform data (Zero Trust Multi-Tenancy isolation ensures data privacy).
  *   **Automated Repayment:** The system automatically intercepts a percentage of the future incoming payments tied to those specific jobs to repay the advance seamlessly.

  ### AI Agent Integration Points
  *   **Finance Agent:** Continuously runs predictive models on the `tenant_id` ledger and calendar. Drafts plain-language offers when a gap is detected.
  *   **Operations Agent:** Informs the Finance Agent of low inventory that needs restocking for upcoming peaks (e.g., Maya's wedding season).

  ## Implementation Prompt
  Implement the Autonomous Smart Working Capital Engine for OneHumanCorp. The system must introduce a predictive cash flow analyzer that continuously monitors a tenant's unified ledger, upcoming bookings, and inventory restock needs. Build a `RiskEvaluator` service that calculates pre-approved micro-advance offers based strictly on the tenant's on-platform history and future confirmed revenue. Ensure strict multi-tenant isolation so financial data never leaks. Develop the mobile-first (375px) endpoint for the UI to receive these proactive offers and process a 1-tap acceptance that instantly credits the tenant's OHC payout wallet. The repayment logic must seamlessly and automatically deduct from the specified future transactions. Acceptance criteria include zero cross-tenant data leakage and the successful, automated lifecycle of a micro-advance from offer to automatic repayment.

  ## Priority
  P1

  ## Estimated Scope
  Large
