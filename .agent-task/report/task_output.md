issue_title: "[Architecture] Autonomous Capital & Growth Funding Engine"
issue_description: |
  # Research Report: OHC Capital & Growth Funding

  ## 1. Executive Summary
  OneHumanCorp (OHC) is missing a native mechanism to provide liquidity and growth capital to its merchants. While competitors like Shopify and Square offer "Capital" products, they remain transactional and dashboard-heavy. OHC has the opportunity to redefine business funding as an autonomous "Growth Engine" that identifies needs (via AI agents) and provides 1-tap capital disbursement with zero applications or jargon.

  ## 2. Problem Statement
  Small business owners like **Maya (baker)** and **Carlos (handyman)** often hit a "Growth Ceiling" where they have more demand than they can handle but lack the liquid capital to invest in the equipment or tools needed to scale.

  Current funding options (bank loans, SBA) are notoriously slow, require mountains of paperwork (P&L statements, tax returns), and use fixed monthly repayments that don't care if a business has a slow week. Alternative "Fintech" loans (Shopify Capital, Square) are better but still rely on static dashboards and jargon-heavy offers. Maya doesn't want to "apply for a loan"; she just wants her business to grow. She needs a system that sees her success, understands her needs, and provides the capital to buy her next oven with a single tap, with repayments that automatically scale with her sales.

  ## 3. Research Report
  ### Market Gap & Competitor Analysis
  - **Market Gap**: 70% of small businesses cite "access to capital" as a primary constraint, yet 60% find the application process "discouraging" or "too complex."
  - **Competitor Analysis**:
      - **Shopify/Square Capital**: Offer funding based on sales history. Repayment is via a percentage of sales.
      - **OHC Advantage**: We go beyond simple sales history. Because OHC's AI agents (The Vigilant Manager) monitor *intent* (DMs, missed bookings, stock-out velocity), we can predict funding needs *before* the owner even realizes them. We replace the "Offer Dashboard" with a "Proactive Growth Proposal."
  - **Revenue-Based Financing (RBF)**: This is the ideal model for OHC. It aligns the platform's success with the merchant's. If Maya sells more, she pays back faster. If she has a slow week, her repayment drops automatically, preventing a cash-flow crisis.

  ## 4. Design Doc

  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  erDiagram
      TENANT ||--o{ FUNDING_OFFER : receives
      TENANT ||--o{ FUNDING_AGREEMENT : enters
      FUNDING_AGREEMENT ||--o{ REPAYMENT_LEDGER : tracks
      FINANCE_AGENT ||--o{ GROWTH_SIGNAL : analyzes

      GROWTH_SIGNAL {
          uuid id
          string type "SELL_OUT | MISSED_BOOKING | INVENTORY_VELOCITY"
          float confidence
      }

      FUNDING_OFFER {
          uuid id
          float amount
          float factor_rate "1.1 - 1.15"
          float payback_percentage "5% - 15%"
          timestamp expires_at
      }

      FUNDING_AGREEMENT {
          uuid id
          float total_payback_amount
          float remaining_balance
          string status "ACTIVE | SETTLED | DEFAULT"
      }

      REPAYMENT_LEDGER {
          uuid id
          uuid transaction_id
          float amount_paid
          timestamp processed_at
      }
  ```

  ### AI Agent Integration Points
  - **The Vigilant Manager (Operations Agent)**: Monitors "Growth Signals" (e.g., "Maya has been sold out of sourdough by 11 AM for 14 consecutive days").
  - **The Treasurer (Finance Agent)**: Calculates the risk-adjusted funding offer based on the tenant's ledger history and growth signals. It generates the `FundingOffer`.
  - **The Collector (Sub-agent of Finance)**: Automatically intercepts the `payback_percentage` from every incoming transaction on the `Universal Ledger` to settle the `Remaining Balance`.

  ### Key Design Decisions
  - **No-Application Funding**: Offers are pre-approved based on OHC's internal data. The user never "applies."
  - **Proactive Growth Proposals**: Instead of a "Capital" tab, the AI pushes a "Growth Proposal" (e.g., "Your bakery is thriving. I've secured $2,500 for a second oven to help you double your output. Tap to accept.").
  - **Revenue-Linked Repayment**: Repayment is a fixed percentage of daily sales, not a fixed monthly fee. Zero sales = zero repayment.
  - **Zero Jargon**: Terms like "Principal", "APR", and "Amortization" are replaced with "Funding Amount", "Total to Pay Back", and "Daily Share".

  ### Mobile UX Flow (375px First)
  1. **The Growth Proposal (Action Feed)**: A macOS Translucent Glass card appears in the main feed: *"🚀 Growth Opportunity Detected: You're selling out fast! I've secured $2,000 for your business growth. [See how this helps]"*
  2. **The 1-Tap Terms (Bottom Sheet)**: Tapping the card opens a clean summary:
      - *"Amount: $2,000 (Available Instantly)"*
      - *"Total Payback: $2,240"*
      - *"How it works: We'll take a 10% share of your daily sales until it's paid back. No interest, no late fees."*
  3. **The Success Chime**: Maya taps "Accept & Deposit." The $2,000 instantly appears in her OHC Wallet.

  ### Technical Integrity & Security
  - **Multi-Tenant Isolation**: Financial records are strictly scoped by `tenant_id` at the database level using PostgreSQL RLS.
  - **Zero-Trust Identity**: Funding actions require biometric or secure token validation (SPIFFE/SPIRE) to prevent unauthorized debt assumption.
  - **Performance & Latency**:
      - Funding disbursement must be processed within < 1 second to the merchant's `OHC_WALLET`.
      - Growth signal detection background jobs must have zero impact on synchronous transaction latency.
  - **Offline Resilience**: The "Growth Proposal" card must be pre-fetched and cached locally on the mobile client. If the user accepts while offline, the cryptographic signature is queued and processed immediately upon reconnection.
  - **Payload Targets**: Funding offer payloads must be < 10KB to ensure instant loading on low-end Android devices (Fatima's use case).

  ## 5. Implementation Prompt
  **To the Implementer:**
  Build the "OHC Capital: Autonomous Micro-Funding & Growth Engine."

  **Core User Journey (CUJ):**
  The Finance AI detects a "Growth Signal" (high sales velocity or missed bookings). It generates a pre-approved `FundingOffer`. The user (e.g., Maya) receives a push notification and a dashboard card. She taps the card, reviews the simple terms (amount, total payback, and daily share percentage), and taps "Accept." The funds are instantly credited to her `OHC_WALLET`, and the system begins automatically deducting the "Daily Share" from every subsequent transaction until the balance is settled.

  **Acceptance Criteria:**
  1. **Growth Signal Processor**: Implement a service that monitors the `InventoryLedger` and `Order` tables to detect sustained high performance.
  2. **Funding Ledger**: Create multi-tenant isolated tables for `FundingOffers`, `FundingAgreements`, and `RepaymentLedger`.
  3. **Automated Repayment Hook**: Implement a hook in the `PaymentProcessor` that intercepts the configured `payback_percentage` and routes it to the `FundingAgreement` balance before settling the remainder to the merchant's wallet.
  4. **Mobile-First UI**: Build the "Growth Proposal" card and the "1-Tap Accept" bottom-sheet using OHC Translucent Glass tokens.
  5. **Grandmother Test**: Ensure the UI uses zero financial jargon. All terms must be plain language.
  6. **Security**: Financial records must be strictly isolated via `tenant_id` and audit-logged immutably.

  ## 6. Priority
  **P1**

  ## 7. Estimated Scope
  **Large**
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
