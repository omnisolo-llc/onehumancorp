issue_title: "Implement Autonomous Cash Flow & Micro-Capital Engine"
issue_description: |
  # Research Report: Autonomous Cash Flow & Micro-Capital Engine

  Small businesses experience uneven cash flows due to seasonality, bulk inventory purchases, or unexpected equipment failure. They frequently need immediate, short-term capital to bridge gaps or seize growth opportunities. Traditional bank loan processes are manual, slow, require extensive paperwork, and evaluate risk based on outdated credit models. Competitors like Stripe Capital, Shopify Capital, and Square Loans provide capital, but often require manual applications, have opaque risk scoring, or aren't deeply integrated into a unified mobile-first ecosystem.

  OneHumanCorp (OHC) needs an AI-driven, transparent, and proactive micro-lending engine directly tied to the unified platform's sales ledger, offering instant, contextual capital without any manual application process.

  ## Design Doc

  ### Key Design Decisions
  1. **Ledger-Native Underwriting:** The engine continuously analyzes the `UniversalWalletLedger` and `SalesLedger` to calculate a dynamic, real-time "Capital Availability Score".
  2. **Proactive AI Finance Department:** The AI Finance agent monitors cash flow forecasts. If a cash crunch is predicted (e.g., due to an upcoming large inventory bill), it proactively offers a micro-loan via a dashboard nudge.
  3. **Frictionless Acceptance & Repayment:** Capital is deposited instantly into the OHC Treasury Wallet. Repayment is completely automated as a small, configurable percentage of daily sales, requiring zero manual transfers.

  ### Architecture Diagram (ER)

  ```mermaid
  erDiagram
      TENANT ||--o{ CAPITAL_OFFER : receives
      TENANT ||--o{ CAPITAL_ADVANCE : takes
      TENANT {
          string id PK
          string business_name
      }
      LEDGER_ENTRY {
          string id PK
          string tenant_id FK
          float amount
          datetime timestamp
      }
      RISK_PROFILE {
          string id PK
          string tenant_id FK
          float availability_score
          float max_eligible_amount
          datetime last_calculated
      }
      CAPITAL_OFFER {
          string id PK
          string tenant_id FK
          float offer_amount
          float fee_percentage
          float repayment_rate
          string status
          datetime expires_at
      }
      CAPITAL_ADVANCE {
          string id PK
          string tenant_id FK
          string offer_id FK
          float total_amount
          float amount_repaid
          float remaining_balance
          string status
      }
      RISK_PROFILE ||--o{ LEDGER_ENTRY : analyzes
      CAPITAL_OFFER }o--|| RISK_PROFILE : generated_from
  ```

  ### Sequence Diagram

  ```mermaid
  sequenceDiagram
      participant EventMesh as NATS Event Mesh
      participant FinanceAI as AI Finance Dept
      participant RiskEngine as Risk Scoring Engine
      participant Merchant
      participant Treasury as Treasury Wallet

      EventMesh->>RiskEngine: Daily Ledger Aggregation Event
      RiskEngine->>RiskEngine: Calculate Availability Score
      RiskEngine->>FinanceAI: Trigger: High Score, Cash flow dip predicted
      FinanceAI->>FinanceAI: Generate Capital Offer
      FinanceAI->>Merchant: Mobile Dashboard Nudge: "$5,000 available instantly for inventory"
      Merchant->>FinanceAI: 1-Tap Accept
      FinanceAI->>Treasury: Deposit $5,000 instantly
      Treasury->>Merchant: Push Notification: "Funds available"
      EventMesh->>FinanceAI: Daily Sales Event
      FinanceAI->>Treasury: Sweep repayment % automatically
  ```

  ### Mobile-First UX Flow (375px Viewport)
  1. **The Proactive Nudge:** Maya opens her OHC app. At the top of the dashboard, a clean, translucent glass UniFi card reads: *"Need to stock up for the holidays? You have $2,000 in instant capital available. Repay automatically from sales."*
  2. **Review & Accept:** Tapping the card opens a bottom sheet showing clear terms (e.g., "$2,000 now, fixed fee of $150, repaid via 8% of daily sales"). No complex interest rates. A single large primary button: "Get Funds Instantly".
  3. **The Repayment View:** In the "Finance" tab, a simple circular progress bar shows the remaining balance, visually indicating the automatic daily sweeps.

  ### Security & Multi-Tenancy
  - **Zero Trust:** Strict SPIFFE/SPIRE identity validation ensures capital offers and ledger sweeps are tightly bound to the authenticated `tenant_id`.
  - **Ledger Integrity:** Advance deposits and automated repayments are recorded as immutable, cryptographically verifiable events in the `UniversalWalletLedger`.

  ## Implementation Prompt

  **To the Implementer:**
  Build the `Autonomous Cash Flow & Micro-Capital Engine` backend services and necessary UI cards.
  1. Implement a real-time `Risk Scoring Engine` that ingests daily aggregates from the `SalesLedger` to calculate and cache a `Capital Availability Score` per tenant.
  2. Build the `Capital Offer Service` that generates predefined offer tiers based on the risk score and publishes them to the AI Finance Department's context.
  3. Create the automated repayment hook: subscribe to daily settlement events and automatically sweep the agreed-upon percentage towards active `CAPITAL_ADVANCE` balances.
  4. Implement the mobile dashboard card (React Native/Flutter equivalent) to display offers and acceptances cleanly.
  Ensure strict multi-tenant isolation for all financial data. All ledger transactions must be atomic. Design the database entities to integrate seamlessly with the existing wallet and ledger infrastructure. Maintain <100ms response times for the dashboard offer retrieval.

  ## Estimated Scope
  **Large**
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []