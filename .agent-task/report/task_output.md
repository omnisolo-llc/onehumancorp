issue_title: "[Architecture] Implement Universal AI-Native Omni-Channel Payment & Ledger System"
issue_description: |
  ## Title: Implement Universal AI-Native Omni-Channel Payment & Ledger System

  ## Problem Statement
  Owners across all business types (Maya's custom cakes, Carlos's field service, Priya's boutique, Leo's tutoring, Fatima's food cart) struggle with disconnected payment flows. Currently, they have to manually reconcile Stripe Dashboard deposits, in-person Tap-to-Pay transactions, subscription billing, and offline cash. They lack a unified ledger where an AI assistant (the Finance/Decision Agent) can instantly query "How much did we make today?" across all channels, automate invoice follow-ups, and trigger operational workflows (e.g., automatically confirming Carlos's booking when a $50 deposit clears). Without this, owners are stuck doing administrative reconciliation instead of running their business.

  ## Research Report
  **Market Gap & Competitor Analysis:**
  - **Shopify:** Excellent omni-channel POS and online checkout, but built primarily for retail products, not flexible enough for service deposits (Carlos) or custom quote approvals (Maya). Admin heavy.
  - **Square:** Strong Tap-to-Pay and invoicing, but lacks native AI orchestration to automatically draft follow-up messages for unpaid invoices or summarize daily operations contextually.
  - **Stripe:** The underlying engine for most, but the Stripe Dashboard is too technical for non-technical operators (requires understanding of PaymentIntents, Webhook signatures, Customers, etc.).
  - **OHC Opportunity:** By building an AI-Native unified ledger on top of Stripe (Checkout, Terminal, Billing), we hide the complexity. The Finance Agent interacts with our internal Ledger (PostgreSQL) and handles Stripe syncing, idempotency, and webhook recovery invisibly. The owner just sees "Priya collected $450 today (Tap-to-Pay + Online)."

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      actor Customer
      actor Owner (Mobile 375px)
      participant OHC Mobile App
      participant AI Agent (Finance/Sales)
      participant OHC Backend (Payment API)
      participant Stripe API
      participant Ledger (PostgreSQL)

      Owner (Mobile 375px)->>OHC Mobile App: Initiates Tap-to-Pay OR creates Quote
      OHC Mobile App->>OHC Backend (Payment API): Create PaymentIntent/Session (Idempotent)
      OHC Backend (Payment API)->>Stripe API: Stripe Request
      Stripe API-->>OHC Backend (Payment API): Client Secret / Terminal Session
      OHC Backend (Payment API)-->>OHC Mobile App: Initialize Payment UI
      Customer->>OHC Mobile App: Taps Card / Pays Online
      OHC Mobile App->>Stripe API: Process Payment
      Stripe API-->>OHC Backend (Payment API): Webhook (charge.succeeded)
      OHC Backend (Payment API)->>Ledger (PostgreSQL): Record Transaction & Update Balance
      Ledger (PostgreSQL)->>AI Agent (Finance/Sales): Event Triggered
      AI Agent (Finance/Sales)-->>Owner (Mobile 375px): Push: "Payment Collected! $50 from Carlos."
  ```

  ### Mobile UX Flow (375px First)
  1. **Home/Command Center:** Owner opens the app and sees "Today's Revenue: $120" at the top as a unified metric.
  2. **Payment Action:** Owner taps the prominent `+` button and selects "Request Payment".
  3. **Omni-Channel Selection:**
     - Option A: "Tap to Pay" (initiates Stripe Terminal SDK on device).
     - Option B: "Send Link" (generates an AI-drafted SMS with a Stripe Payment Link).
  4. **Processing State:** A translucent, blur-backed modal shows the payment status ("Waiting for card...", "Approved").
  5. **Completion:** The UI returns to the Home view with an updated revenue counter and a generated AI summary ("Order #102 marked paid").

  ### AI Agent Integration Points
  - **Finance Assistant:** Listens to the Unified Ledger events. When a payment is marked as `failed` or `disputed`, it drafts a notification to the owner explaining the issue in plain English.
  - **Sales Assistant:** When a quote is accepted and a deposit is paid, it automatically moves the CRM state to `Booked` and informs the Operations Assistant to block the calendar.
  - **Advisor Assistant:** Reads the Ledger daily to generate the "Morning Brief" (e.g., "Tap-to-pay sales are up 20% this week.").

  ### Key Design Decisions and Why
  - **Unified Internal Ledger:** We do not rely solely on Stripe's API for the source of truth for display. We sync Stripe webhooks to a local PostgreSQL `ledger_entries` table with strict `tenant_id` isolation. This ensures 0-latency loads for the mobile app and offline-tolerant reads.
  - **Idempotency by Default:** Every payment request from the mobile app generates a UUID `idempotency_key`. Flaky networks (Fatima's food cart) will not result in double charges.
  - **Zero Technical Jargon:** No mention of "Payment Intents", "Charges", or "Webhooks" in the UI. We use "Collect Payment", "Paid", and "Failed".

  ## Implementation Prompt
  **For the Implementer Agent:**
  Your task is to implement the Universal Omni-Channel Payment & Ledger core system.
  1. **CUJ (Critical User Journey):** A business owner (like Carlos) creates a $50 deposit request via the mobile interface, sends it to a customer, the customer pays (simulated), and the owner's dashboard instantly reflects the updated revenue without a page refresh.
  2. **Acceptance Criteria:**
     - Create a backend `ledger_entries` table (with `tenant_id` RLS) and the corresponding gRPC/REST endpoints to fetch the balance.
     - Implement a robust Stripe webhook handler that idempotently processes `payment_intent.succeeded` events and inserts/updates the `ledger_entries`.
     - Build a visually premium, translucent 375px-friendly "Request Payment" flow in the Flutter/PWA UI that integrates with the backend.
     - Include full E2E Playwright tests covering the creation of a payment request, the webhook simulation, and the UI reflecting the paid status.
     - Ensure the AI Finance agent is triggered upon payment success to log an event in the owner's feed.
  Do not prescribe exact internal database schemas or Rust traits; design for multi-tenancy and robust retry logic.

  ## Estimated Scope
  Large

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
