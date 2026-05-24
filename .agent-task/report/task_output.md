issue_title: "[Architecture] Universal Autonomous Cashflow Forecasting & Smart Capital Engine"
issue_description: |
  # Problem Statement
  Small business owners (like Carlos the handyman or Fatima the food cart operator) constantly struggle to predict their cash flow. They often run out of cash to buy supplies before getting paid for big jobs, or can't afford to repair broken equipment. Traditional loans are slow, require extensive paperwork, and feel intimidating. They need an invisible, AI-driven engine that proactively forecasts cash dips and offers 1-tap micro-capital directly within the OneHumanCorp platform.

  # Research Report
  - **Competitor Analysis:**
    - **Shopify Capital & Stripe Capital:** Offer proactive loans based on GMV and processing history. Highly successful, seamless repayment (percentage of daily sales). However, they lack deep autonomous forecasting—they are mostly reactive to past sales rather than predictive of upcoming seasonal dips, booked but unpaid jobs, or recurring invoice obligations.
    - **Square Loans:** Similar model, deeply integrated into the POS ecosystem. Effective, but not autonomous in its forecasting.
  - **Data Gap:** OHC has the unique advantage of seeing the *full* picture: calendar bookings, outstanding quotes, regular inventory purchases, and payroll, not just settled card transactions.
  - **Opportunity:** By combining full-stack business context (quotes, bookings, inventory) with an AI Finance Department, OHC can proactively alert users (e.g., "Carlos, you have a $2,000 cash dip coming next week because of pending supply orders. Tap here to get a $1,500 advance, repaid automatically from your upcoming invoices.")

  # Design Doc
  ## Architecture
  The Universal Autonomous Cashflow Forecasting & Smart Capital Engine bridges the `Ledger`, `BookingEngine`, `InventoryManager`, and the `AI Finance Department`.

  ```mermaid
  erDiagram
      MERCHANT ||--o{ CASHFLOW_FORECAST : generates
      MERCHANT ||--o{ CAPITAL_OFFER : receives
      CASHFLOW_FORECAST {
          string forecast_id
          date target_date
          float expected_inflow
          float expected_outflow
          float net_position
          string risk_level
      }
      CAPITAL_OFFER {
          string offer_id
          float amount
          float fee_percentage
          float repayment_rate
          string status
      }
      LEDGER ||--o{ CASHFLOW_FORECAST : informs
      BOOKING_ENGINE ||--o{ CASHFLOW_FORECAST : informs
  ```

  ```mermaid
  sequenceDiagram
      participant O as OHC Platform (AI Finance Dept)
      participant M as Merchant (Mobile App)
      participant L as Ledger / Payments
      O->>L: Analyze recent revenue, upcoming bookings, outstanding quotes
      O->>O: Detect upcoming cash flow dip ($2k deficit in 7 days)
      O->>O: Pre-qualify Merchant for $2k Capital Advance
      O->>M: Push Notification: "Cash flow alert: Upcoming dip. Tap to cover."
      M->>M: Views 1-tap Capital Offer (macOS-style glass card UI)
      M->>O: Taps "Accept Advance"
      O->>L: Instantly fund Merchant Balance
      L-->>O: Future sales automatically deduct 10% until repaid
  ```

  ## AI Agent Integration Points
  - **AI Finance Department:** Runs daily background jobs analyzing multi-tenant ledger data securely. Predicts cash flows based on historical seasonality and current CRM/booking data.
  - **AI Ops Department:** Works with Finance to identify if a cash dip is due to necessary inventory replenishment.

  ## Mobile UX Flow (375px First)
  1. **Push Notification:** Plain English, no jargon. "Heads up Maya, looks like your ingredient costs are due before your big wedding cake orders pay out. Need a quick advance?"
  2. **Dashboard Card (Glassmorphism):** A clean, translucent card on the home screen showing a simple graph (green for cash, red for upcoming bills).
  3. **1-Tap Action:** A large, accessible button: "Get $1,000 now. Repay automatically from future sales." No forms, no fine print (hidden behind a simple "Advanced Settings" switch for full terms).

  # Implementation Prompt
  **To the Implementer Agent:**
  Build the backend infrastructure for the Universal Autonomous Cashflow Forecasting Engine.
  - Create the background worker jobs (e.g., in our high-performance queue) that securely process a tenant's upcoming bookings, pending invoices, and historical ledger data to generate a rolling 30-day cash flow forecast.
  - Implement the internal API for the AI Finance Department to query this forecast and trigger a pre-qualified `CapitalOffer` entity.
  - Build the endpoint that allows a mobile client to 1-tap accept the offer, instantly crediting their platform ledger balance and setting up the automatic split-payment repayment mechanism on future incoming transactions.
  - Ensure strict Zero-Trust multi-tenant isolation so no merchant's data leaks into another's forecast.
  - Provide a mock UI component that demonstrates the 1-tap acceptance on a 375px viewport using our macOS-style Translucent Glass materials.

  # Priority
  P1

  # Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
