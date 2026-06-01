issue_title: "[Architecture] Autonomous Split Payments & Commission Engine"
issue_description: |
  # Title: Autonomous Split Payments & Commission Engine (Stripe Connect)

  ## Problem Statement
  Carlos (Freelance Handyman, 42) frequently subcontracts specialized work (like electrical or plumbing) to other independent workers he trusts. Currently, if a customer books a "Kitchen Remodel Assessment" that requires an electrician, Carlos has to collect the full payment, manually calculate the subcontractor's cut, and send it via Venmo or bank transfer. This manual process causes accounting headaches, tax liabilities (1099 generation), and cash flow delays. Similarly, Maya (Baker) occasionally partners with a local florist for "Cake & Flower" bundles and faces the same split payment nightmare. OHC currently handles 1:1 payments well via Stripe, but lacks an automated mechanism to dynamically split a single customer transaction into multiple payouts (platform fee, primary merchant, subcontractors/partners) instantly and invisibly.

  ## Research Report
  *   **Competitor Analysis**:
      *   **Shopify / Wix**: Generally built for single-merchant e-commerce. Splitting payments requires expensive third-party apps (like CollabPay) which are complex to configure and require technical knowledge.
      *   **Stripe Connect**: Provides the underlying infrastructure for split payments and marketplace routing, but its API is heavily developer-centric. Expecting Carlos to understand "Destination Charges", "Transfer Reversals", or "Connected Accounts" violates OHC's zero-tech promise.
      *   **Square**: Offers team management and basic commission tracking, but is more geared towards internal staff (W2/1099 employees within one legal entity) rather than independent B2B splits.
  *   **The OHC Differentiator**: OHC must abstract Stripe Connect into a natural language "Partnership" concept. AI handles the complexity of onboarding the subcontractor to Stripe, calculating the dynamic splits (e.g., 80/20 or fixed $50), and automatically routing the funds at checkout without Carlos or the subcontractor ever looking at a dashboard.

  ## Design Doc

  ### High-Level Architecture
  ```mermaid
  graph TD;
      Customer[Customer Checkout] -->|Pays $200 Deposit| OHC_Stripe_Client[OHC Stripe Integration];
      OHC_Stripe_Client -->|Creates PaymentIntent| StripeAPI[Stripe API];
      StripeAPI -->|Webhook: payment_intent.succeeded| OHC_Webhook_Handler[Webhook Handler];
      OHC_Webhook_Handler --> EventMesh[Hybrid Event Mesh];
      EventMesh --> OpsAgent[AI Operations Agent];
      OpsAgent --> FinanceAgent[AI Finance Agent];
      FinanceAgent -->|Looks up Split Rules| SplitLedger[(Tenant Split Ledger)];
      FinanceAgent -->|Executes Transfers| ConnectAPI[Stripe Connect Transfers];
      ConnectAPI --> PrimaryAccount[Carlos's Bank Account: $140];
      ConnectAPI --> PartnerAccount[Electrician's Bank Account: $60];
      FinanceAgent -->|Generates Briefing| BusinessAdvisory[Business Advisory Agent];
  ```

  ### Key Design Decisions & Invariants
  *   **Stripe Connect Express/Custom Abstraction**: Subcontractors are onboarded as Stripe Connect accounts behind the scenes. When Carlos adds a "Partner" in OHC, the AI Finance Agent sends a simplified, OHC-branded, mobile-optimized SMS link to the partner to securely collect their payout details (KYC handled invisibly by Stripe).
  *   **Dynamic Split Ledger (Tenant Isolated)**: Split rules are stored in a new pgvector-indexed relational table `split_payment_rules` tied to specific `product_id`s or `booking_type_id`s, strictly isolated by `tenant_id` (Row-Level Security).
  *   **AI Finance Agent Coordination**: The Finance Agent ("The Accountant") acts as the intelligent router. It monitors the `payment_intent.succeeded` event on the Hybrid Event Mesh, reads the split rule, calculates OHC's platform fee, the primary merchant's cut, and the partner's cut, and issues the Transfer requests to Stripe Connect.
  *   **Fallback and Ledger Integrity**: If a split transfer fails (e.g., partner account not fully verified), the funds remain in the primary merchant's pending balance (or OHC escrow, depending on liability model) and the Ops Agent triggers a plain-language notification to Carlos and the partner.

  ### Mobile UX Flow (375px First)
  1.  **Adding a Partner (Carlos's App)**:
      *   Carlos creates a new service: "Kitchen Remodel (+ Electrician)".
      *   He taps "Add Partner Split" (massive, friendly button).
      *   He enters the electrician's phone number and selects "Electrician gets 30%".
  2.  **Partner Onboarding**:
      *   The electrician gets an SMS: "Carlos added you to a job on OHC. Tap here to tell us where to send your 30% cut."
      *   Electrician completes a 3-screen, frictionless mobile flow to link their debit card/bank (Stripe Connect onboarding framed in glassmorphism).
  3.  **The Briefing**:
      *   After the job, Carlos gets his daily briefing: "You earned $140 today. $60 was automatically sent to your electrician."

  ### Performance & Security Targets
  *   **Idempotency**: All split transfer calls to Stripe Connect MUST use strictly generated idempotency keys based on `payment_intent_id` and `partner_id` to prevent double payouts.
  *   **Zero Trust**: Subcontractor financial data (bank info) never touches OHC servers; it is tokenized directly to Stripe.
  *   **Latency**: Webhook processing to Transfer initiation < 2 seconds.

  ## Implementation Prompt
  **Objective**: Implement the data models, Stripe Connect service layer, and AI Finance Agent capabilities to support automated Split Payments.

  **User Journey (CUJ) & Acceptance Criteria**:
  1.  **Rule Creation**: An API endpoint and database schema must support defining a split rule (e.g., % or flat fee) linked to a specific service or product and a partner's phone/email.
  2.  **Ledger & Webhook Integration**: The existing Stripe webhook handler must emit a `PaymentSucceeded` event to the Event Mesh. The Finance Agent must consume this, identify if the purchased item has a split rule, and calculate the exact cent amounts.
  3.  **Transfer Execution**: The `StripeClient` must be extended to support Connect `Transfer` creations using the calculated amounts. Ensure strict idempotency.
  4.  **Error Handling**: If a partner is not yet onboarded (missing Connect Account ID), the system must gracefully hold the funds and queue an SMS reminder task for the Operations Agent.

  **Constraints**:
  Do not modify the core `payment_intents` logic for the primary charge; the splits must happen post-charge (Destination Charges or Separate Charges and Transfers) to keep the customer's checkout flow pristine and uninterrupted.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
