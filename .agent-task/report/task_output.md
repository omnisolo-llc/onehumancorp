issue_title: "Autonomous Capital and Funding Engine"
issue_description: |
  # Architecture Brief: Autonomous Capital and Funding Engine

  ## Title
  Autonomous Capital and Funding Engine

  ## Problem Statement
  Small business owners—especially those without extensive credit histories like Fatima (food cart) or Maya (baker)—struggle to secure working capital. Traditional banks require complex paperwork, business plans, and weeks of underwriting. When Fatima needs $500 immediately to repair a broken generator, or Maya needs $1,000 to buy bulk ingredients for a massive wedding order, they are forced to use high-interest personal credit cards. Because OHC possesses the complete, unified financial ledger (sales velocity, booking history, refund rates), we have perfect underwriting data. We need an invisible "Finance Department" that proactively offers micro-loans or revenue-based financing instantly, directly within the mobile app, with 1-tap approval and automated repayment via a percentage of daily sales.

  ## Research Report
  - **Competitor Landscape**:
    - **Shopify Capital**: Highly successful, proving the model that platform-embedded financing works. However, it's focused on e-commerce product sales.
    - **Square Loans**: Excellent execution for in-person retail.
    - **Traditional Banks**: Too slow, minimum loan amounts are too high (often $10k+), and they don't understand the cash flow of a solopreneur.
  - **OHC Opportunity**: OHC serves service providers and hybrid businesses that competitors miss. OHC's AI agents can *proactively* suggest capital when they detect an anomaly (e.g., the Operations agent detects a broken piece of equipment or the Sales agent sees a huge influx of bookings requiring more inventory).

  ## Design Doc

  ### Key Design Decisions
  1.  **Event-Driven Underwriting**: The system constantly evaluates the `UniversalWalletLedger` and `BookingEngine`. When a merchant crosses a risk threshold (e.g., 3 months of consistent >$2k/mo revenue, low dispute rate), they are automatically pre-approved.
  2.  **Proactive Suggestion (Not Just a Dashboard)**: If Maya receives a $5,000 custom cake inquiry but her inventory ledger shows she lacks funds for supplies, the Finance Agent drafts a message: "I see a large order from John. You are pre-approved for $1,000 in working capital to fulfill this. Tap to accept."
  3.  **Frictionless Repayment**: Repayment is handled invisibly by deducting a fixed percentage (e.g., 10%) from every incoming OHC payment until the balance + flat fee is cleared. No monthly bills.
  4.  **Zero Jargon**: Terms must pass the Grandmother Test. No APR calculations; instead: "Borrow $1,000 today. We will automatically take 10% of your daily sales until you have paid back $1,100."

  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  graph TD
      Ledger[Universal Wallet Ledger] --> Underwriter[AI Underwriting Service]
      Bookings[Booking & Invoice Engine] --> Underwriter

      Underwriter -- Calculates Risk --> PreApproval{Pre-Approved?}
      PreApproval -- Yes --> OfferTrigger[Event: Capital Offer Available]

      EventAction[Business Event: Large Order / Low Stock] --> OpsAgent[Ops Agent]
      OpsAgent -->|Checks Funding Status| OfferTrigger
      OfferTrigger --> UINotify[Mobile UI: Proactive Offer]

      UINotify -->|1-Tap Accept| Contract[Contract & Liability Engine]
      Contract --> Payout[Instant Payout to OHC Wallet]

      IncomingPayment[New Customer Payment] --> Ledger
      Ledger --> Splitter[Split Payment Engine]
      Splitter -->|10% Repayment| CapitalPool[OHC Capital Pool]
      Splitter -->|90% Revenue| MerchantWallet[Merchant Wallet]
  ```

  ### AI Agent Integration Points
  - **Finance & Legal Agent**: Handles the continuous underwriting and formulates the simple-English contract terms.
  - **Operations Agent**: Triggers the capital offer if it detects a supply chain blockage or an unusually large incoming order.

  ### Mobile UX Flow (375px First)
  1.  **The Trigger**: A translucent Glassmorphism card appears in the Action Feed: "Funding Available: $1,000 to grow your business."
  2.  **The Details Screen**: Clean, large typography. "Get $1,000 instantly to your OHC Wallet. Repay $1,100 automatically from your sales (10% per sale). No hidden fees."
  3.  **The Action**: A swipe-to-confirm slider at the bottom (prevents accidental taps).
  4.  **The Result**: Instant confetti animation. The OHC Wallet balance immediately increases by $1,000.

  ## Implementation Prompt
  Implement the backend architecture for the `Autonomous Capital Engine`.
  - Create an underwriting worker that periodically scans merchant ledgers to calculate eligibility scores.
  - Implement the `CapitalOffer` and `CapitalLedger` entities.
  - Integrate with the existing `SplitPaymentsLedger` to ensure that when a merchant has an active advance, a defined percentage of every incoming transaction is routed to the platform's repayment account before the remainder hits the merchant's wallet.
  - Ensure strict tenant isolation and atomic ledger transactions to prevent double-funding or incorrect repayment tracking. Do NOT prescribe specific DB tables.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
