issue_title: "Implement Instant Payouts & Virtual Card Issuing Engine Architecture"
issue_description: |
  # Research Report: Instant Localized Payouts and Virtual Card Issuing Engine

  ## Problem Statement
  Small business owners often rely on deposits or daily sales to buy supplies. Traditional payment processors hold funds for 2-5 business days, creating a cash flow choke point. OHC needs to bypass this latency by issuing a platform-native Virtual Wallet and Business Debit Card, providing instant liquidity.

  ## Competitive Analysis
  - **Shopify:** Requires rigorous business verification (EIN, SSN), geared toward established LLCs.
  - **Wix/Squarespace:** Relies on third-party gateways (Stripe, Square) with standard rolling payout delays.
  - **GoDaddy:** Instant payouts carry a 1-2% extra penalty fee.

  ## Proposed Architecture
  - **Virtual Wallet:** A native OHC Wallet balance.
  - **Virtual Card:** Immediate provisioning of an Apple Pay / Google Pay ready virtual card.
  - **Ledger Engine:** Strict row-level tenant isolation in PostgreSQL.
  - **AI Integration:** AI Finance Department monitors spend velocity; AI Fraud Department scores every deposit/payout.

  ## Next Steps
  - Implement the strictly isolated ledger data model.
  - Build a generic issuing interface for virtual card provisioning.
  - Develop mobile UI components (macOS translucent glass, 375px optimized).
  - Integrate AI Finance and Fraud background events.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
