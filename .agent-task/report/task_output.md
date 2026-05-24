issue_title: "[architecture] Autonomous Working Capital & Dynamic Cash Flow Advance Engine"
issue_description: |
  ## Problem Statement
  Small business owners like Maya (the 28-year-old custom baker) and Fatima (the 50-year-old food cart operator) frequently encounter cash flow bottlenecks when trying to grow. Maya might need a $1,200 commercial stand mixer to fulfill holiday orders, while Fatima needs bulk capital for weekend festival ingredients. Traditional bank micro-loans are out of the question due to massive paperwork, credit checks, and week-long wait times. Although OneHumanCorp (OHC) possesses real-time, perfect visibility into their daily transaction velocity, customer retention, and overall financial health, the platform completely lacks an embedded capital advance mechanism. This forces users off-platform for financing and introduces massive friction into their growth loop.

  ## Research Report
  *   **Industry Context & Competitor Analysis**: Leading SMB platforms have recognized that extending capital based on platform data is a massive differentiator.
      *   *Shopify Capital*: Offers proactive funding based on sales history; repays automatically via a fixed percentage of daily sales. Has advanced billions of dollars to merchants.
      *   *Square Loans*: Integrates directly into the POS flow. A merchant gets a notification, taps accept, and funds are deposited instantly. Repayment is a frictionless daily deduction.
      *   *Stripe Capital*: Uses machine learning on payment volume to extend predictive offers.
  *   **OHC Architectural Gap**: Currently, OHC routes 100% of transaction settlements directly to the merchant's wallet. We do not have a "Split Ledger" architecture capable of dynamically intercepting a configurable percentage (e.g., 10%) of daily GMV to service an internal advance. Furthermore, our AI Finance Agents currently lack a risk-modeling framework to proactively surface these pre-approved offers based on multi-tenant transaction telemetry.
  *   **Business Impact**: By introducing this architecture, OHC solves a massive pain point for all personas (Physical, Service, Food/Bev). It removes the "financial jargon" barrier of traditional loans (no APR, just a fixed fee) and significantly boosts OHC platform stickiness and revenue (via the fixed fee on the advance).

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  flowchart TD
      subgraph Edge System
          M[Mobile App 375px] -->|Accepts Offer| API[API Gateway]
          C[Customer Payment Tap] -->|Pays $100| API
      end

      subgraph OHC Core Services
          API --> RO[Risk & Offer Engine AI]
          RO -->|Pre-approves| M

          API --> SL[Split Payment Ledger]
          SL -->|90%| MW[Merchant Treasury Wallet]
          SL -->|10%| CR[Capital Repayment Pool]

          CR -->|Update| RO
      end

      subgraph AI Orchestration
          FA[Finance AI Agent] -.->|Monitors Velocity| RO
          OA[Ops AI Agent] -.->|Reconciles Books| SL
      end
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  1.  **Dashboard Nudge**: A beautiful, Translucent Glass card appears natively on the merchant's home feed: *"Maya, you're pre-approved for $1,200 to grow your bakery. Tap to review."*
  2.  **Offer Configuration (Grandmother Test Passed)**:
      *   No jargon. No "APR" or "Amortization".
      *   A simple slider to select the amount (e.g., $500 to $1,200).
      *   Clear readouts: "Amount you get today: $1,200." "Total you repay: $1,320." "Repayment: 10% of future sales."
  3.  **One-Tap Deposit**: User taps "Accept & Transfer." The UI transitions smoothly; a checkmark appears, and the $1,200 is immediately available in their OHC Treasury Wallet.
  4.  **Persistent Tracking**: The main financial tab now features a minimalist, reassuring progress bar showing "Advance Repayment: 12% complete ($158 / $1,320)."

  ### AI Agent Integration Points
  *   **Finance AI Department**: Continuously evaluates merchant transaction history, seasonal velocity, and refund rates to dynamically determine capital eligibility and size the offer in the background. It surfaces the offer when the merchant hits a peak growth phase.
  *   **Operations AI Department**: Invisibly manages the double-entry accounting. When a customer buys a $100 cake, the AI automatically routes $90 to the merchant's spendable balance and $10 to the capital repayment balance, auto-generating the clean tax-ready ledger entries without the merchant needing to do manual bookkeeping.

  ### Key Design Decisions
  *   **Zero-Trust & Multi-Tenant Isolation**: The Risk Engine runs strictly within tenant-bound memory limits. Anonymized data used to train the global underwriting model must be stripped of all PII via our SPIFFE/SPIRE identity layer.
  *   **Mobile Parity & Offline Targets**: The capital offer and current repayment state are aggressively cached at the edge. If the merchant opens the app in a low-connectivity environment (e.g., a basement prep kitchen), the UI must instantly render the cached offer state.
  *   **Fixed-Fee Model**: By utilizing a flat fee rather than an interest rate, we maintain maximum UX simplicity and ensure regulatory compliance across diverse global markets, perfectly aligning with the "no manuals" platform ethos.

  ## Implementation Prompt
  **For the Implementer Agent:**
  Implement the backend capability for the Split Payment Ledger and the mobile-first (375px) UI components for the Autonomous Working Capital Engine.
  The core User Journey (CUJ) is: A merchant taps a pre-approved capital offer, selects the advance amount via a slider, and accepts. Upon acceptance, the system must instantly credit their wallet and configure the global payment router to automatically intercept and split a fixed percentage of all future incoming customer payments until the advance is repaid.
  Acceptance Criteria:
  1. The merchant UI must clearly display the offer, the fixed fee, and the split percentage without using financial jargon.
  2. The core ledger must mathematically guarantee that incoming payments are split correctly and the repayment progress is updated atomically.
  Do NOT prescribe the specific database schema, ORM, or API signatures. Design the internal boundaries to enforce Zero-Trust and multi-tenant isolation.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
