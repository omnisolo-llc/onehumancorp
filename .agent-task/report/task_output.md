issue_title: "Implement Autonomous AI Loyalty & Rewards Engine"
issue_description: |
  # Mission Queue Protocol: Omnichannel Unified Loyalty & Rewards Mesh

  ## Problem Statement
  Small business owners like Priya (Boutique Operator) and Fatima (Food Cart Operator) struggle to encourage repeat business seamlessly. Current loyalty programs are heavily siloed—either physical punch cards, expensive third-party tools (like Smile.io) integrated awkwardly into Shopify, or require customers to download an entirely separate app. When a customer buys online via Instagram DMs and later taps-to-pay in-store, their loyalty points often don't sync. Owners need an invisible, omnichannel loyalty engine that tracks customer engagement across all touchpoints (POS, DMs, Website) and allows AI agents to automatically propose and fulfill rewards without manual tracking.

  ## Research Report
  - **Competitor Systems Audit:**
    - **Shopify:** Relies almost entirely on apps like Smile.io or Yotpo. These are complex to set up, have fragmented APIs, and require technical knowledge to unify in-store and online channels effectively.
    - **Square:** Offers Square Loyalty, which is robust for in-store purchases but lacks deep integration into conversational commerce (e.g., redeeming points directly inside an Instagram DM).
    - **Wix/Squarespace:** Basic discount codes, but lacks a persistent "wallet" or "mesh" that AI agents can use proactively to recover leads.
  - **Identify Gaps:** OHC needs a native, multi-tenant `Loyalty Mesh` integrated tightly with the `Customer Identity` and `Ledger` systems. When a customer interacts via DM, the "Sales & Acquisition" Agent should instantly know their loyalty status and automatically offer to apply points to their deposit, utilizing the Conversational Checkout Engine.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer Touchpoint: DM / POS / Web] -->|Transaction / Interaction| B(Omni Context Routing);
      B -->|Resolve Customer ID| C[Customer Identity Engine];
      C -->|Add/Redeem Points| D[Universal Loyalty Mesh];
      D -->|Atomic Balance Update| E[(Postgres - tenant isolated)];

      F[Sales & Acquisition Agent] -->|Reads Balance| D;
      F -->|Proposes Discount in DM| A;

      G[Finance Agent] -->|Tracks Loyalty Liability| D;

      D -.->|Webhook/Event| H[Marketing Agent];
      H -.->|Auto-send "You earned a free cake!"| A;
  ```

  ### Mobile-First UX Flow (375px)
  - **For the Owner (Dashboard):**
    - A new "Loyalty & Rewards" card featuring frosted macOS-style translucent glass.
    - Toggle switch to "Enable Automated Loyalty" (AI manages points).
    - Simple sliders for "Points per dollar" and "Redemption value", with AI Business Advisory suggesting optimal values based on the business type.
  - **For the Customer (Checkout/DM):**
    - Inside the Conversational Checkout sheet (WebP-optimized webview), a seamless "Apply 500 points ($5 off)" toggle switch appears natively if the customer identity is resolved. No app download required.

  ### AI Agent Integration Points
  - **Customer Success (The Ambassador):** Uses the `Loyalty Mesh` to appease angry customers by instantly granting points.
  - **Sales & Acquisition:** Uses the `Loyalty Mesh` to close hesitant buyers by reminding them they have enough points for a free add-on.
  - **Finance (The Accountant):** Reconciles the loyalty points as a liability on the unified ledger.

  ### Security & Zero Trust
  - **Tenant Isolation:** The loyalty ledger in Postgres must enforce strict Row Level Security (`tenant_id`).
  - **Idempotency:** All point issuance and redemption endpoints must use idempotency keys to prevent double-spending during network partitions at the offline POS.

  ## Implementation Prompt
  **Task for Implementer Agent:**
  Implement the Omnichannel Unified Loyalty & Rewards Mesh backend and owner-facing settings.
  - **User-Facing Outcome:** The business owner can toggle on "Automated Loyalty" from their mobile dashboard. Customers earn points seamlessly across POS and DMs, and AI agents can query and apply these points during conversational checkouts.
  - **Acceptance Criteria:**
    - Create the Postgres data models for `LoyaltyWallets` and `LoyaltyLedger` with strict RLS and Double-Entry bookkeeping.
    - Expose gRPC endpoints for `GrantPoints`, `RedeemPoints`, and `GetWalletBalance`.
    - Update the AI Agent tools to allow the Sales and Customer Success agents to read balances and grant points.
    - Provide at least 5 Playwright E2E tests verifying an owner can configure loyalty, and a simulated customer checkout flow correctly applies points.
    - Zero mock data; use the real database.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
