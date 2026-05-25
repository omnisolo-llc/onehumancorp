issue_title: "[Architecture] Predictive Cashflow and Autonomous Micro-Financing Engine"
issue_description: |
  # Predictive Cashflow and Autonomous Micro-Financing Engine

  ## Problem Statement
  Small business owners frequently experience cash flow anxiety. Waiting for Net-30 invoice payments, dealing with unexpected inventory costs, or managing seasonal dips can paralyze operations. While platforms like Shopify and Stripe offer capital, they are often reactive and require manual intervention. OHC currently lacks a proactive, conversational cashflow prediction and financing engine that seamlessly integrates with its unified ledger and AI agents.

  ## Research Report
  By leveraging the existing unified ledger (`InstantLocalizedInvoicingLedger`, `UniversalCapacityAndInventoryLedger`) and transaction history, OHC can deploy an AI Finance Department. This department will continuously model cash flow, predict shortfalls over a 14-day window, and proactively propose one-tap micro-loans directly in the conversational inbox (e.g., "Hey Carlos, it looks like you'll need $500 for supplies next week before the Smith invoice clears. Tap here to get an instant advance for a flat $15 fee.").

  The architecture relies on a background worker continuously underwriting merchants based on ledger data, completely eliminating the need for application forms.

  ## Design Doc

  ### Architecture Diagram

  ```mermaid
  graph TD
      A[Unified Ledger Data] -->|Real-time sync| B(Cashflow Prediction Model)
      C[Invoice/Deposit Engine] -->|Pending Receivables| B
      D[Inventory/Capacity Mesh] -->|Upcoming Costs| B
      B -->|Predicts Shortfall| E(AI Finance Department Agent)
      E -->|Generates Offer| F[Micro-Financing Risk Engine]
      F -->|Approved Offer| E
      E -->|Push Notification / DM| G[Mobile Conversational UI]
      G -->|1-Tap Accept| H(Treasury/Payout Wallet)
      H -->|Funds Disbursed instantly| I[User Bank/Debit Card]
      H -->|Repayment Schedule via Ledger| A
  ```

  ### Data Model & Entity Relationship (ER) Diagram

  ```mermaid
  erDiagram
      MERCHANT ||--o{ CASHFLOW_PREDICTION : "has"
      MERCHANT ||--o{ MICRO_LOAN : "holds"
      CASHFLOW_PREDICTION {
          uuid id
          uuid tenant_id
          date prediction_date
          decimal projected_shortfall
          timestamp created_at
      }
      MICRO_LOAN {
          uuid id
          uuid tenant_id
          decimal amount
          decimal flat_fee
          string status
          timestamp accepted_at
      }
  ```

  ### Sequence Diagram for Financing Approval

  ```mermaid
  sequenceDiagram
      participant Worker as Background Worker
      participant Ledger as Unified Ledger
      participant RiskEngine as Risk Engine
      participant Agent as AI Finance Agent
      participant Mobile as Mobile App (User)

      Worker->>Ledger: Poll for updated transactions
      Worker->>RiskEngine: Run predictive cashflow models
      alt Shortfall Predicted
          RiskEngine->>Agent: Alert predicted shortfall
          RiskEngine->>Agent: Issue pre-approved offer
          Agent->>Mobile: Proactive Push Notification & Inbox Message
          Mobile->>Agent: 1-Tap Accept (Biometric)
          Agent->>Ledger: Execute Treasury Transfer
      end
  ```

  The user experience must pass the "Grandmother Test."
  - **Mobile UI:** The experience is embedded in the daily plain-language briefing. It features glassmorphism cards presenting simple, readable charts and plain-language explanations of cashflow dips.
  - **Interaction:** Acceptance of micro-financing is a 1-tap action utilizing biometric authentication (FaceID/TouchID). Complex APR details are hidden behind an "Advanced Settings" toggle; users see simple flat fees.
  - **Security:** All financial predictions, risk scores, and funding offers are strictly isolated by `tenant_id`, with ledger queries enforcing SPIFFE/SPIRE identity checks.

  ## Implementation Prompt
  The engineering swarm should implement the background worker, API endpoints, and the mobile-first UI components to enable this capability. The background worker should periodically evaluate tenant ledgers and pending invoices to predict cash flow. API endpoints should allow the AI Finance Agent to surface predicted shortfalls and pre-approved funding offers.

  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []