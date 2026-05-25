issue_title: "Autonomous Cash Flow & Smart Capital Engine"
issue_description: |
  # Issue Brief: Autonomous Cash Flow & Smart Capital Engine

  ## Problem Statement
  For small business owners like Carlos (handyman) and Maya (baker), managing cash flow is a constant source of anxiety. Carlos often has to buy expensive materials upfront before a client pays the final invoice, leaving his bank account dangerously low. Maya gets a massive catering order but needs working capital to buy ingredients in bulk, which she doesn't have until the client pays the 50% deposit. They don't have time to fill out complex bank loan applications or wait weeks for approval. Traditional platforms require them to manually apply for capital or wait for periodic offers. They need an invisible, proactive financial engine that predicts cash flow crunches before they happen and instantly offers exactly the right amount of smart capital (with transparent, flat-fee repayment taken directly from future sales) exactly when they need it, accessible with a single tap on their phone.

  ## Research Report
  ### Current Landscape & Competitive Analysis
  *   **Shopify Capital / Stripe Capital / Square Loans:** These platforms offer working capital based on historical sales data. However, they are generally *reactive*. The business owner receives an email offering a loan, which they must then accept. The offers are based on trailing 6-12 month revenue run rates.
  *   **Wix / Squarespace:** Offer very limited or no native lending, relying heavily on third-party app integrations which require separate onboarding and disconnect the financial data from the operational data.
  *   **The OneHumanCorp (OHC) Advantage:** OHC possesses the complete context of the business journey. We know when Carlos issues a $5,000 quote that gets approved. We know when Maya receives a large custom cake inquiry via Instagram DM. Because OHC handles quoting, inventory, and messaging, our AI can be *predictive*. Instead of offering a generic $10k loan based on past sales, OHC can instantly offer a micro-loan specifically sized to cover the material costs of a newly signed, upcoming job, precisely at the moment the materials need to be purchased.

  ### Key Findings
  1.  **Context-Aware Lending Yields Higher Conversion:** When capital offers are tied to a specific business action (e.g., fulfilling a massive order) rather than a generic email blast, conversion is significantly higher, and the capital is deployed more effectively.
  2.  **Micro-Capital > Macro-Loans:** Solopreneurs often need $500 - $2,000 to bridge a specific gap, not a $20,000 general-purpose loan.
  3.  **Frictionless Repayment is Crucial:** Repayment must be an automated, fixed percentage of daily sales to ensure the business owner never has to think about making a manual payment or tracking due dates.

  ## Design Doc

  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  erDiagram
      BUSINESS_LEDGER ||--o{ TRANSACTION : records
      QUOTE_ENGINE ||--o{ APPROVED_QUOTE : generates
      AI_FINANCE_AGENT ||--o{ CASH_FLOW_PREDICTION : generates
      AI_FINANCE_AGENT ||--o{ CAPITAL_OFFER : creates
      CAPITAL_OFFER ||--o{ REPAYMENT_AGREEMENT : defines

      BUSINESS_LEDGER {
          string business_id
          decimal current_balance
      }

      APPROVED_QUOTE {
          string quote_id
          decimal expected_revenue
          decimal estimated_material_cost
          date expected_start_date
      }

      CASH_FLOW_PREDICTION {
          string prediction_id
          date date_of_shortfall
          decimal estimated_shortfall_amount
      }

      CAPITAL_OFFER {
          string offer_id
          decimal amount
          decimal flat_fee
          decimal repayment_percentage
          string trigger_context
      }
  ```

  ```mermaid
  sequenceDiagram
      participant Carlos as Carlos (Mobile App)
      participant OHC_UI as OHC Translucent UI
      participant FinanceAgent as AI Finance Agent
      participant Ledger as Universal Ledger
      participant Quoting as Unified Quoting Engine

      Carlos->>Quoting: Sends $5k Quote for Deck Build
      Quoting-->>FinanceAgent: Quote Approved Event (Materials: $1.5k)
      FinanceAgent->>Ledger: Check Current Available Cash
      Ledger-->>FinanceAgent: Balance: $800 (Shortfall: $700)
      FinanceAgent->>FinanceAgent: Calculate Risk & Generate Offer
      FinanceAgent-->>OHC_UI: Push Smart Capital Notification
      OHC_UI->>Carlos: Nudge: "Need funds for the Deck Build materials? Tap for $1,500 advance."
      Carlos->>OHC_UI: Taps "Accept & Deposit"
      OHC_UI-->>Ledger: Deposit Funds
      Ledger-->>Carlos: Funds available instantly
  ```

  ### UI Wireframes & Screen Flow (375px Mobile First)

  **Screen 1: The Intelligent Nudge (Dashboard)**
  *   A clean, macOS-glass style card appears at the top of the Home Dashboard.
  *   **Visual:** Subtle gradient background (e.g., soft green/blue) indicating financial opportunity.
  *   **Copy:** "Your $5,000 quote for 'Deck Build' was approved! Need to buy materials? Get $1,500 instantly."
  *   **Action:** Primary button "Review Offer". Secondary button "Dismiss".

  **Screen 2: Transparent Offer Detail (Half-Sheet Modal)**
  *   Slides up smoothly from the bottom.
  *   **Visual:** Large, clear typography. No tiny legal jargon.
  *   **Copy:**
      *   **Advance Amount:** $1,500
      *   **Flat Fee:** $100 (Total to repay: $1,600)
      *   **How you repay:** We automatically deduct 10% of your daily sales until it's paid off. If you make no sales, you pay nothing that day.
  *   **Action:** Swipe to Accept.

  **Screen 3: Instant Availability Confirmation**
  *   **Visual:** Confetti/Success animation.
  *   **Copy:** "$1,500 is in your OneHumanCorp account. Your virtual card is ready to use for materials."

  ### Mobile UX Flow
  The flow is designed to be completed in under 15 seconds. It intercepts the user precisely when their intent and need are highest (e.g., right after a large quote is approved). The "Swipe to Accept" mechanism ensures a deliberate action while keeping the interaction frictionless.

  ### AI Agent Integration Points
  *   **AI Finance Agent:** Continuously monitors the `BUSINESS_LEDGER` and upcoming scheduled events (e.g., from `QUOTE_ENGINE` or `CALENDAR_ENGINE`). It uses predictive models to forecast cash flow dips and calculates pre-approved capital limits dynamically based on platform-wide trust scores and historical completion rates.
  *   **AI Operations Agent:** Feeds data into the Finance Agent regarding supply costs and typical material requirements for specific job types (e.g., knowing that a "Deck Build" usually requires 30% upfront material costs).

  ### Key Design Decisions and Why
  *   **Predictive vs. Reactive:** We trigger offers based on *future confirmed revenue* (approved quotes, signed contracts) rather than just past performance. This solves the actual pain point: bridging the gap between starting a job and getting paid.
  *   **Flat Fee + Percentage Daily Repayment:** No compounding interest, no late fees, no fixed monthly payments. This is the only ethical way to lend to small businesses with volatile cash flow. It guarantees they won't default if they have a slow week.
  *   **Integrated Issuing:** The capital must be deposited instantly into their OHC balance, immediately accessible via an OHC virtual debit card (Tap-to-Pay/Apple Pay), so they can walk into Home Depot 5 minutes later and buy supplies.

  ## Implementation Prompt
  **To the Implementer:**
  Your task is to build the backend logic and the mobile-first frontend components for the Autonomous Cash Flow & Smart Capital Engine.
  The core Customer User Journey (CUJ) is: A business owner has a large quote approved but lacks the cash for materials. The system proactively calculates the required capital, pushes a notification with a transparent offer (Advance Amount, Flat Fee, Repayment %), and upon 1-tap approval, instantly credits their internal ledger balance. Repayment must be an automated, background process that intercepts future incoming payments, routing a defined percentage to the repayment ledger until the balance is cleared.
  Ensure the UI components follow the OHC macOS-style translucent glass design system and are perfectly optimized for a 375px viewport. Keep the underlying financial complexity (risk modeling, ledger routing) entirely invisible to the user. Do not prescribe specific ledger schemas, but ensure multi-tenant isolation and strict atomic transactions.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
